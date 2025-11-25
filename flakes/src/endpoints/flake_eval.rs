use serde::{Deserialize, Serialize};
use warp::Reply;
use anyhow::Result;
use crate::models::EvalResult;
use crate::utils::NixCommand;

#[derive(Debug, Deserialize)]
pub struct FlakeEvalRequest {
    pub flake_path: String,
    pub expression: String,
    #[serde(default = "default_json_output")]
    pub json_output: bool,
}

fn default_json_output() -> bool {
    true
}

#[derive(Debug, Serialize)]
pub struct FlakeEvalResponse {
    pub result: EvalResult,
}

pub async fn handle_flake_eval(req: FlakeEvalRequest) -> Result<impl Reply, warp::Rejection> {
    let (stdout, stderr) = NixCommand::eval(&req.flake_path, &req.expression, req.json_output)
        .map_err(|e| warp::reject::custom(EndpointError::NixError(e.to_string())))?;

    let result = EvalResult {
        result: stdout.trim().to_string(),
        success: true,
        logs: stderr,
    };

    let response = FlakeEvalResponse { result };
    Ok(warp::reply::json(&response))
}

#[derive(Debug)]
pub enum EndpointError {
    NixError(String),
}

impl warp::reject::Reject for EndpointError {}

