mod models;
mod modules;
mod utils;
mod mcp;

use anyhow::Result;

fn main() -> Result<()> {
    // MCP servers must log to stderr, not stdout
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Run the MCP stdio server
    mcp::run_stdio_server()
}
