use clap::Parser;

#[derive(Parser)]
#[command(name = "synapse")]
#[command(about = "Synapse CLI", version)]
struct Args {
    /// API base URL
    #[arg(long, env = "SYNAPSE_BASE_URL")]
    base_url: Option<String>,

    /// API key
    #[arg(long, env = "SYNAPSE_API_KEY")]
    api_key: Option<String>,
}

impl Args {
    fn resolve_base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    fn resolve_api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }
}

fn main() {
    let _args = Args::parse();
}
