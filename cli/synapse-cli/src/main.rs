use clap::{Parser, Subcommand};
use synapse_cli::{CliConfig, OutputFormat};

mod handlers {
    use super::{GraphqlCmd, SettlementsCmd, TransactionsCmd};
    use synapse_cli::{CliConfig, Formatter, OutputFormat, SynapseCliClient};

    pub async fn handle_transactions(
        command: TransactionsCmd,
        config: &CliConfig,
        _output_format: OutputFormat,
    ) -> anyhow::Result<()> {
        match command {
            TransactionsCmd::Export {
                format,
                from,
                to,
                status,
                asset_code,
                output,
            } => {
                let client = SynapseCliClient::new(&config.base_url);

                let mut query_params: Vec<(&str, String)> = Vec::new();
                query_params.push(("format", format.clone()));

                let from_owned;
                if let Some(ref f) = from {
                    from_owned = f.clone();
                    query_params.push(("from", from_owned.clone()));
                }

                let to_owned;
                if let Some(ref t) = to {
                    to_owned = t.clone();
                    query_params.push(("to", to_owned.clone()));
                }

                let status_owned;
                if let Some(ref s) = status {
                    status_owned = s.clone();
                    query_params.push(("status", status_owned.clone()));
                }

                let asset_code_owned;
                if let Some(ref ac) = asset_code {
                    asset_code_owned = ac.clone();
                    query_params.push(("asset_code", asset_code_owned.clone()));
                }

                let query_refs: Vec<(&str, &str)> = query_params
                    .iter()
                    .map(|(k, v)| (*k, v.as_str()))
                    .collect();

                let bytes = client.get_bytes("/export", &query_refs).await?;

                if let Some(output_path) = output {
                    std::fs::write(&output_path, &bytes)?;
                    println!("✓ Exported to {}", output_path);
                } else {
                    let output = String::from_utf8(bytes)?;
                    println!("{}", output);
                }

                Ok(())
            }
        }
    }

    pub async fn handle_settlements(
        command: SettlementsCmd,
        config: &CliConfig,
        _output_format: OutputFormat,
    ) -> anyhow::Result<()> {
        let client = SynapseCliClient::new(&config.base_url);

        match command {
            SettlementsCmd::List {
                cursor,
                limit,
                direction,
                format,
            } => {
                let mut query_params: Vec<(&str, String)> = Vec::new();
                query_params.push(("limit", limit.to_string()));
                query_params.push(("direction", direction.clone()));

                let cursor_owned;
                if let Some(ref c) = cursor {
                    cursor_owned = c.clone();
                    query_params.push(("cursor", cursor_owned.clone()));
                }

                let query_refs: Vec<(&str, &str)> = query_params
                    .iter()
                    .map(|(k, v)| (*k, v.as_str()))
                    .collect();

                let fmt = OutputFormat::from_str(&format);
                let response: serde_json::Value =
                    client.get_with_query("/settlements", &query_refs).await?;

                let output = Formatter::format_json_output(&response, fmt)?;
                println!("{}", output);

                Ok(())
            }

            SettlementsCmd::Get {
                settlement_id,
                format,
            } => {
                let fmt = OutputFormat::from_str(&format);
                let path = format!("/settlements/{}", settlement_id);
                let response: serde_json::Value = client.get_json(&path).await?;

                let output = Formatter::format_json_output(&response, fmt)?;
                println!("{}", output);

                Ok(())
            }
        }
    }

    pub async fn handle_graphql(
        command: GraphqlCmd,
        config: &CliConfig,
    ) -> anyhow::Result<()> {
        let client = SynapseCliClient::new(&config.base_url);

        match command {
            GraphqlCmd::Query { query, format } => {
                let body = serde_json::json!({ "query": query, "variables": null });
                let response: serde_json::Value = client.post_json("/graphql", &body).await?;

                let fmt = OutputFormat::from_str(&format);

                // Surface application-level GraphQL errors (HTTP 200 + errors array)
                if let Some(errors) = response.get("errors") {
                    if let Some(arr) = errors.as_array() {
                        if !arr.is_empty() {
                            let msg = arr
                                .iter()
                                .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                                .collect::<Vec<_>>()
                                .join("; ");
                            anyhow::bail!("graphql error: {}", msg);
                        }
                    }
                }

                let output = Formatter::format_json_output(&response, fmt)?;
                println!("{}", output);

                Ok(())
            }
        }
    }
}

#[derive(Parser)]
#[command(name = "synapse")]
#[command(about = "Synapse CLI - Transaction, Settlement, and GraphQL management")]
#[command(version)]
struct Cli {
    /// Base URL for the Synapse API
    #[arg(long, env = "SYNAPSE_URL")]
    url: Option<String>,

    /// API key for authentication
    #[arg(long, env = "SYNAPSE_API_KEY")]
    api_key: Option<String>,

    /// Output format (table or json)
    #[arg(long, default_value = "table", global = true)]
    format: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Transaction management commands
    Transactions {
        #[command(subcommand)]
        command: TransactionsCmd,
    },

    /// Settlement management commands
    Settlements {
        #[command(subcommand)]
        command: SettlementsCmd,
    },

    /// Send a raw GraphQL query to the Synapse API
    Graphql {
        #[command(subcommand)]
        command: GraphqlCmd,
    },
}

#[derive(Subcommand)]
enum TransactionsCmd {
    /// Export transactions to CSV or JSON format with optional filters.
    Export {
        /// Export format: 'csv' (default) or 'json'
        #[arg(long, default_value = "csv")]
        format: String,

        /// Start date filter (inclusive). Format: YYYY-MM-DD.
        #[arg(long)]
        from: Option<String>,

        /// End date filter (inclusive). Format: YYYY-MM-DD.
        #[arg(long)]
        to: Option<String>,

        /// Filter by transaction status (e.g., pending, completed).
        #[arg(long)]
        status: Option<String>,

        /// Filter by asset code (e.g., USD, EUR, USDC).
        #[arg(long)]
        asset_code: Option<String>,

        /// Output file path. Default: stdout.
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum SettlementsCmd {
    /// List settlements with cursor-based pagination.
    List {
        /// Pagination cursor from a previous response.
        #[arg(long)]
        cursor: Option<String>,

        /// Number of results per page (1-100, default 10).
        #[arg(long, default_value = "10")]
        limit: i64,

        /// Pagination direction: 'forward' (default) or 'backward'.
        #[arg(long, default_value = "forward")]
        direction: String,

        /// Output format: 'table' (default) or 'json'.
        #[arg(long, default_value = "table")]
        format: String,
    },

    /// Get a specific settlement by ID.
    Get {
        /// Settlement UUID.
        settlement_id: String,

        /// Output format: 'table' (default) or 'json'.
        #[arg(long, default_value = "table")]
        format: String,
    },
}

/// GraphQL subcommand — send raw queries to `POST /graphql`.
///
/// Exit codes:
///   0 – success
///   1 – GraphQL application error (HTTP 200 with `errors` array) or network/HTTP error
///
/// Output formats:
///   table – human-readable key/value output (default)
///   json  – pretty-printed JSON response
#[derive(Subcommand)]
enum GraphqlCmd {
    #[command(
        about = "Send a raw GraphQL query and print the response",
        long_about = "Send a raw GraphQL query to POST /graphql and print the result.\n\n\
                      Exit codes:\n  \
                      0 - Success\n  \
                      1 - GraphQL application error or network/HTTP failure\n\n\
                      Output formats:\n  \
                      table - Human-readable output (default)\n  \
                      json  - Pretty-printed JSON"
    )]
    Query {
        /// The GraphQL query string (e.g. \"{ transactions { id status } }\")
        #[arg(long)]
        query: String,

        /// Output format: 'table' (default) or 'json'
        #[arg(long, default_value = "table")]
        format: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("synapse_cli=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    let mut config = CliConfig::from_env()?;
    if let Some(url) = cli.url {
        config.base_url = url;
    }
    if let Some(api_key) = cli.api_key {
        config.api_key = Some(api_key);
    }

    let output_format = OutputFormat::from_str(&cli.format);

    match cli.command {
        Commands::Transactions { command } => {
            handlers::handle_transactions(command, &config, output_format).await?
        }
        Commands::Settlements { command } => {
            handlers::handle_settlements(command, &config, output_format).await?
        }
        Commands::Graphql { command } => {
            if let Err(e) = handlers::handle_graphql(command, &config).await {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
