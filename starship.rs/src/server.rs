mod handler;

use crate::config::Config;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use warp::Filter;

pub use handler::handle_mcp_request;

#[derive(Debug, Deserialize)]
pub struct MCPRequest {
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct MCPResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<MCPError>,
}

#[derive(Debug, Serialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
}

pub async fn start_server(config: Config) -> Result<()> {
    // Configure CORS
    let cors = if config.cors_allowed_origins.is_empty() {
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"])
    } else {
        warp::cors()
            .allow_origins(
                config
                    .cors_allowed_origins
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>(),
            )
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"])
    };

    let routes = mcp_route()
        .or(health_route())
        .with(cors)
        .with(warp::log("starship_mcp_server"));

    log::info!("Starting Starship MCP Server on port {}", config.port);
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
    Ok(())
}

fn health_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})))
}

fn mcp_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("mcp")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_mcp_request)
}

