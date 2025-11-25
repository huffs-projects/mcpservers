use serde::{Deserialize, Serialize};
use warp::Reply;
use anyhow::Result;
use crate::models::FlakeOutput;
use crate::utils::NixCommand;

#[derive(Debug, Deserialize)]
pub struct FlakeOutputsRequest {
    pub flake_path: String,
    #[serde(default)]
    pub filter: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FlakeOutputsResponse {
    pub outputs: Vec<FlakeOutput>,
}

pub async fn handle_flake_outputs(req: FlakeOutputsRequest) -> Result<impl Reply, warp::Rejection> {
    let show_output = NixCommand::flake_show(&req.flake_path)
        .map_err(|e| warp::reject::custom(EndpointError::NixError(e.to_string())))?;

    let mut outputs = Vec::new();

    fn extract_outputs(
        value: &serde_json::Value,
        prefix: &str,
        outputs: &mut Vec<FlakeOutput>,
        filter: &Option<String>,
    ) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let attr_path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };

                    if let Some(filter) = filter {
                        if !attr_path.contains(filter) {
                            continue;
                        }
                    }

                    if val.is_object() {
                        if let Some(type_str) = val.get("type").and_then(|v| v.as_str()) {
                            let output_type = match type_str {
                                "derivation" => crate::models::flake_output::OutputType::Package,
                                "app" => crate::models::flake_output::OutputType::App,
                                "nixosModule" => crate::models::flake_output::OutputType::Module,
                                _ => continue,
                            };

                            let drv_path = val.get("drvPath")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            let doc_url = Some(format!("https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html#flake-outputs"));

                            outputs.push(FlakeOutput {
                                attribute: attr_path.clone(),
                                drv_path,
                                r#type: output_type,
                                documentation_url: doc_url,
                            });
                        } else {
                            extract_outputs(val, &attr_path, outputs, filter);
                        }
                    } else {
                        extract_outputs(val, &attr_path, outputs, filter);
                    }
                }
            }
            _ => {}
        }
    }

    extract_outputs(&show_output, "", &mut outputs, &req.filter);

    let response = FlakeOutputsResponse { outputs };
    Ok(warp::reply::json(&response))
}

#[derive(Debug)]
pub enum EndpointError {
    NixError(String),
}

impl warp::reject::Reject for EndpointError {}

