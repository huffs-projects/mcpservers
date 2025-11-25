mod models;
mod endpoints;
mod utils;
mod mcp;
mod tools;
mod error;

#[tokio::main]
async fn main() {
    // Log to stderr (MCP requirement - stdout is for JSON-RPC)
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();
    
    mcp::run_stdio_server().await;
}

