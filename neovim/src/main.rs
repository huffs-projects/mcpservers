use neovim_mcp_server::mcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    mcp::run_stdio_server().await
}
