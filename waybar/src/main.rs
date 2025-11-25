mod endpoints;
mod models;
mod mcp;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // MCP servers must log to stderr, not stdout
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "waybar_mcp=info".into()),
        )
        .init();
    tracing::info!("Waybar MCP Server starting...");

    let mut server = mcp::McpServer::new();
    server.run().await
}

