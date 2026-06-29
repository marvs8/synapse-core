use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::{Args, Subcommand};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

use crate::formatter::{Formatter, OutputFormat};

/// A real-time transaction status update pushed by the server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionStatusUpdate {
    pub transaction_id: Uuid,
    pub tenant_id: Uuid,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub message: Option<String>,
}

#[derive(Args)]
pub struct EventsCmd {
    #[command(subcommand)]
    pub command: EventsSubcommand,
}

#[derive(Subcommand)]
pub enum EventsSubcommand {
    /// Stream real-time transaction status events from the server.
    ///
    /// Connects to the server WebSocket endpoint (GET /ws?token=<token>) and
    /// prints each incoming TransactionStatusUpdate. Press Ctrl-C to stop.
    ///
    /// Connection lifecycle: closing the subscription sends a proper WebSocket
    /// close frame; no dangling background task is left running.
    Watch {
        /// API token passed as the `?token=` query parameter.
        #[arg(long, env = "SYNAPSE_API_KEY", default_value = "")]
        token: String,

        /// Output format: table (default) or json.
        #[arg(long, default_value = "table")]
        format: String,
    },
}

/// Subscribe to real-time events by driving the WebSocket inline.
///
/// Calls `on_event` for each parsed [`TransactionStatusUpdate`] and `on_error`
/// for any parse / connection error. Returns when the server closes the
/// connection, `on_event` returns `false`, or `on_error` returns `false`.
///
/// **Connection lifecycle**: a Close frame is sent before returning; no
/// background task is left running.
pub async fn subscribe<FE, FErr>(
    base_url: &str,
    token: &str,
    mut on_event: FE,
    mut on_error: FErr,
) -> Result<()>
where
    FE: FnMut(TransactionStatusUpdate) -> bool,
    FErr: FnMut(anyhow::Error) -> bool,
{
    // Convert http(s) base_url to ws(s) and append the path + token.
    let ws_url = base_url
        .replacen("https://", "wss://", 1)
        .replacen("http://", "ws://", 1);
    let ws_url = format!("{}/ws?token={}", ws_url.trim_end_matches('/'), token);

    let (ws_stream, _) = connect_async(&ws_url)
        .await
        .map_err(|e| anyhow::anyhow!("WebSocket handshake failed: {}", e))?;

    let (mut write, mut read) = ws_stream.split();

    loop {
        match read.next().await {
            Some(Ok(Message::Text(text))) => {
                match serde_json::from_str::<TransactionStatusUpdate>(&text) {
                    Ok(event) => {
                        if !on_event(event) {
                            break;
                        }
                    }
                    Err(e) => {
                        if !on_error(anyhow::anyhow!("parse error: {}", e)) {
                            break;
                        }
                    }
                }
            }
            Some(Ok(Message::Close(_))) | None => break,
            Some(Ok(_)) => {} // ignore ping/pong/binary frames
            Some(Err(e)) => {
                if !on_error(anyhow::anyhow!("connection error: {}", e)) {
                    break;
                }
            }
        }
    }

    // Send close frame — ensures the socket is shut down cleanly and no
    // background task is left running.
    let _ = write.send(Message::Close(None)).await;
    let _ = write.close().await;

    Ok(())
}

/// Handle the `events watch` subcommand end-to-end.
pub async fn handle_events(cmd: EventsCmd, base_url: &str) -> Result<()> {
    match cmd.command {
        EventsSubcommand::Watch { token, format } => {
            let fmt = OutputFormat::from_str(&format);

            subscribe(
                base_url,
                &token,
                |event| {
                    match Formatter::format_json_output(&event, fmt) {
                        Ok(output) => println!("{}", output),
                        Err(e) => eprintln!("format error: {}", e),
                    }
                    true // keep streaming
                },
                |err| {
                    eprintln!("error: {}", err);
                    true // keep streaming on transient errors
                },
            )
            .await
        }
    }
}
