use serde::{Deserialize, Serialize};
use warp::Reply;
use anyhow::Result;
use crate::models::FlakeInput;
use crate::utils::NixCommand;

#[derive(Debug, Deserialize)]
pub struct FlakeInputsRequest {
    pub flake_path: String,
    #[serde(default)]
    pub filter: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FlakeInputsResponse {
    pub inputs: Vec<FlakeInput>,
}

pub async fn handle_flake_inputs(req: FlakeInputsRequest) -> Result<impl Reply, warp::Rejection> {
    let metadata = NixCommand::flake_metadata(&req.flake_path)
        .map_err(|e| warp::reject::custom(EndpointError::NixError(e.to_string())))?;

    let mut inputs = Vec::new();

    if let Some(locked) = metadata.get("locked") {
        if let Some(locked_inputs) = locked.get("nodes") {
            for (name, node) in locked_inputs.as_object().unwrap_or(&serde_json::Map::new()) {
                if let Some(filter) = &req.filter {
                    if !name.contains(filter) {
                        continue;
                    }
                }

                let url = node.get("original")
                    .and_then(|v| v.get("url"))
                    .or_else(|| node.get("url"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let revision = node.get("locked")
                    .and_then(|v| v.get("rev"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let input_type = if url.starts_with("git+") || url.starts_with("https://") || url.starts_with("git://") {
                    crate::models::flake_input::InputType::Git
                } else if url.starts_with("/") || url.starts_with(".") {
                    crate::models::flake_input::InputType::Path
                } else {
                    crate::models::flake_input::InputType::Url
                };

                let doc_url = Some(format!("https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html#flake-inputs"));

                inputs.push(FlakeInput {
                    name: name.clone(),
                    url,
                    revision,
                    r#type: input_type,
                    documentation_url: doc_url,
                });
            }
        }
    }

    let response = FlakeInputsResponse { inputs };
    Ok(warp::reply::json(&response))
}

#[derive(Debug)]
pub enum EndpointError {
    NixError(String),
}

impl warp::reject::Reject for EndpointError {}

