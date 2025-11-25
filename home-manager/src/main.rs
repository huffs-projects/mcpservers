mod config;
mod endpoints;
mod error;
mod metrics;
mod models;
mod server;
mod utils;

use server::Server;
use utils::logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_logger();

    let server = Server::new();
    server.run().await?;

    Ok(())
}

