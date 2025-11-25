use serde::{Deserialize, Serialize};
use warp::Reply;
use anyhow::Result;
use crate::models::BuildResult;
use crate::utils::NixCommand;

#[derive(Debug, Deserialize)]
pub struct FlakeBuildRequest {
    pub flake_path: String,
    pub outputs: Vec<String>,
    #[serde(default = "default_dry_run")]
    pub dry_run: bool,
}

fn default_dry_run() -> bool {
    true
}

#[derive(Debug, Serialize)]
pub struct FlakeBuildResponse {
    pub result: BuildResult,
}

pub async fn handle_flake_build(req: FlakeBuildRequest) -> Result<impl Reply, warp::Rejection> {
    let (success, logs, errors, built_paths) = NixCommand::build(
        &req.flake_path,
        &req.outputs,
        req.dry_run,
    )
    .map_err(|e| warp::reject::custom(EndpointError::NixError(e.to_string())))?;

    let result = BuildResult {
        success,
        logs,
        errors,
        built_paths,
    };

    let response = FlakeBuildResponse { result };
    Ok(warp::reply::json(&response))
}

#[derive(Debug)]
pub enum EndpointError {
    NixError(String),
}

impl warp::reject::Reject for EndpointError {}

