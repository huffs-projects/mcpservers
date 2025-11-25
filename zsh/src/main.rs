mod models;
mod endpoints;
mod utils;
mod mcp;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();
    
    mcp::run_stdio_server().await.map_err(|e| anyhow::anyhow!("{}", e))
}

