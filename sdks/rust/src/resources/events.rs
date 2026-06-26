use crate::client::SynapseClient;
use crate::error::SynapseError;
use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

pub struct Events<'a> {
    pub(crate) client: &'a SynapseClient,
}

/// A real-time transaction status update pushed by the server over the
/// WebSocket connection.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionStatusUpdate {
    pub transaction_id: Uuid,
    pub tenant_id: Uuid,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub message: Option<String>,
}

impl<'a> Events<'a> {
    /// Subscribe to real-time transaction events via `GET /ws`.
    ///
    /// Connects to the server's WebSocket endpoint, forwarding each incoming
    /// event to `on_event` and any error to `on_error`.  Returns only when the
    /// connection is closed — either by the server or because `on_event` /
    /// `on_error` returns `false`.
    ///
    /// **Connection lifecycle**: the socket is closed cleanly before this
    /// function returns and no background task is left running.
    ///
    /// # Parameters
    /// - `on_event` – called for each [`TransactionStatusUpdate`] received.
    ///   Return `true` to continue, `false` to close the subscription.
    /// - `on_error` – called when a message cannot be parsed or a connection
    ///   error occurs. Return `true` to continue, `false` to close.
    ///
    /// # Errors
    /// Returns [`SynapseError::Http`] if the initial WebSocket handshake fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use synapse_sdk::{SynapseClient, SynapseError};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SynapseClient::new("https://api.example.com", "your-api-key");
    ///
    /// client
    ///     .events()
    ///     .subscribe(
    ///         |event| {
    ///             println!("tx {} -> {}", event.transaction_id, event.status);
    ///             true // keep listening
    ///         },
    ///         |err| {
    ///             eprintln!("error: {}", err);
    ///             true // keep listening despite bad messages
    ///         },
    ///     )
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn subscribe<F, E>(
        &self,
        mut on_event: F,
        mut on_error: E,
    ) -> Result<(), SynapseError>
    where
        F: FnMut(TransactionStatusUpdate) -> bool,
        E: FnMut(SynapseError) -> bool,
    {
        // Build the WebSocket URL: replace http(s) scheme with ws(s).
        let ws_url = {
            let base = self.client.base_url.trim_end_matches('/');
            let ws_base = if base.starts_with("https://") {
                base.replacen("https://", "wss://", 1)
            } else {
                base.replacen("http://", "ws://", 1)
            };
            format!("{}/ws?token={}", ws_base, self.client.api_key)
        };

        let (mut ws_stream, _) = connect_async(&ws_url).await.map_err(|e| SynapseError::Http {
            status: 0,
            body: e.to_string(),
        })?;

        loop {
            match ws_stream.next().await {
                Some(Ok(Message::Text(text))) => {
                    match serde_json::from_str::<TransactionStatusUpdate>(&text) {
                        Ok(event) => {
                            if !on_event(event) {
                                break;
                            }
                        }
                        Err(e) => {
                            let err = SynapseError::Http {
                                status: 0,
                                body: format!("parse error: {}", e),
                            };
                            if !on_error(err) {
                                break;
                            }
                        }
                    }
                }
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(_)) => {} // ignore ping/pong/binary
                Some(Err(e)) => {
                    let err = SynapseError::Http {
                        status: 0,
                        body: e.to_string(),
                    };
                    if !on_error(err) {
                        break;
                    }
                }
            }
        }

        // Close the socket cleanly — no dangling background task or thread.
        let _ = ws_stream.close(None).await;
        Ok(())
    }
}
