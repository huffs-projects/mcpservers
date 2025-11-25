mod config;
mod endpoints;
mod error;
mod mcp;
mod models;
mod server;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger - use stderr for MCP stdio protocol
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .format_module_path(false)
        .target(env_logger::Target::Stderr)
        .init();

    // Run MCP stdio server
    mcp::run_stdio_server().await?;
    Ok(())
}

