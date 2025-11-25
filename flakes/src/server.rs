use warp::Filter;
use serde_json::json;
use crate::endpoints::flake_inputs::{FlakeInputsRequest, FlakeInputsResponse};
use crate::endpoints::flake_outputs::{FlakeOutputsRequest, FlakeOutputsResponse};
use crate::endpoints::flake_eval::{FlakeEvalRequest, FlakeEvalResponse};
use crate::endpoints::flake_build::{FlakeBuildRequest, FlakeBuildResponse};
use crate::endpoints::flake_scaffold::{FlakeScaffoldRequest, FlakeScaffoldResponse};
use crate::utils::NixCommand;
use crate::models::{FlakeInput, FlakeOutput, EvalResult, BuildResult};

#[derive(serde::Deserialize)]
pub struct MCPRequest {
    pub method: String,
    pub params: Option<serde_json::Value>,
    pub id: Option<serde_json::Value>,
}

#[derive(serde::Serialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
    pub id: Option<serde_json::Value>, // Always present in responses (matches request id)
}

#[derive(serde::Serialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
}

pub async fn handle_mcp_stdio_request(line: &str) -> Result<Option<MCPResponse>, anyhow::Error> {
    // #region agent log
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
        let _ = writeln!(f, r#"{{"id":"log_server_001","timestamp":{},"location":"server.rs:32","message":"INCOMING REQUEST","data":{{"raw_line":{:?},"line_len":{}}},"sessionId":"debug-session","runId":"run3","hypothesisId":"A"}}"#,
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            line,
            line.len()
        );
    }
    // #endregion
    let req: MCPRequest = match serde_json::from_str::<MCPRequest>(line) {
        Ok(r) => {
            // #region agent log
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
                let _ = writeln!(f, r#"{{"id":"log_server_002","timestamp":{},"location":"server.rs:40","message":"PARSED REQUEST","data":{{"method":{:?},"has_params":{},"has_id":{}}},"sessionId":"debug-session","runId":"run3","hypothesisId":"A"}}"#,
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                    r.method,
                    r.params.is_some(),
                    r.id.is_some()
                );
            }
            // #endregion
            r
        },
        Err(e) => {
            // #region agent log
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
                let _ = writeln!(f, r#"{{"id":"log_server_003","timestamp":{},"location":"server.rs:50","message":"PARSE ERROR","data":{{"error":{:?}}},"sessionId":"debug-session","runId":"run3","hypothesisId":"A"}}"#,
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                    e.to_string()
                );
            }
            // #endregion
            return Err(e.into());
        }
    };
    let id = req.id.clone();
    let response = handle_mcp_request_internal(req).await;
    // #region agent log
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
        let response_json = serde_json::to_string(&response).unwrap_or_default();
        let _ = writeln!(f, r#"{{"id":"log_server_004","timestamp":{},"location":"server.rs:60","message":"OUTGOING RESPONSE","data":{{"has_result":{},"has_error":{},"has_id":{},"response_preview":{:?}}},"sessionId":"debug-session","runId":"run3","hypothesisId":"A"}}"#,
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            response.result.is_some(),
            response.error.is_some(),
            response.id.is_some(),
            if response_json.len() > 200 { format!("{}...", &response_json[..200]) } else { response_json }
        );
    }
    // #endregion
    // Return None for notifications (no id), Some for requests
    Ok(id.map(|_| response))
}

async fn handle_mcp_request_internal(req: MCPRequest) -> MCPResponse {
    // #region agent log
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
        let _ = writeln!(f, r#"{{"id":"log_server_005","timestamp":{},"location":"server.rs:65","message":"HANDLING METHOD","data":{{"method":{:?},"method_len":{}}},"sessionId":"debug-session","runId":"run3","hypothesisId":"B"}}"#,
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            req.method,
            req.method.len()
        );
    }
    // #endregion
    let response = match req.method.as_str() {
        "initialize" => {
            // #region agent log
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
                let _ = writeln!(f, r#"{{"id":"log_server_006","timestamp":{},"location":"server.rs:75","message":"HANDLING INITIALIZE","data":{{}},"sessionId":"debug-session","runId":"run3","hypothesisId":"C"}}"#,
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
                );
            }
            // #endregion
            MCPResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "nix-flakes-mcp-server",
                        "version": "2.0.0"
                    }
                })),
                error: None,
                id: req.id,
            }
        }
        "tools/list" => {
            // #region agent log
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
                let _ = writeln!(f, r#"{{"id":"log_server_007","timestamp":{},"location":"server.rs:95","message":"HANDLING TOOLS/LIST","data":{{}},"sessionId":"debug-session","runId":"run3","hypothesisId":"B"}}"#,
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
                );
            }
            // #endregion
            let tools = json!([
                {
                    "name": "flake_inputs",
                    "description": "List all inputs of a flake following canonical Nix flake structure.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "flake_path": {
                                "type": "string",
                                "description": "Path or URL of flake"
                            },
                            "filter": {
                                "type": "string",
                                "description": "Optional filter for input names"
                            }
                        },
                        "required": ["flake_path"]
                    }
                },
                {
                    "name": "flake_outputs",
                    "description": "List outputs of a flake according to canonical flake attributes and derivations.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "flake_path": {
                                "type": "string",
                                "description": "Path or URL of flake"
                            },
                            "filter": {
                                "type": "string",
                                "description": "Optional filter for output attributes"
                            }
                        },
                        "required": ["flake_path"]
                    }
                },
                {
                    "name": "flake_eval",
                    "description": "Evaluate arbitrary flake expressions safely, conforming to best practices from nix.dev and nixos-and-flakes-book.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "flake_path": {
                                "type": "string",
                                "description": "Path or URL of flake"
                            },
                            "expression": {
                                "type": "string",
                                "description": "Expression to evaluate"
                            },
                            "json_output": {
                                "type": "boolean",
                                "description": "Output as JSON",
                                "default": true
                            }
                        },
                        "required": ["flake_path", "expression"]
                    }
                },
                {
                    "name": "flake_build",
                    "description": "Build selected outputs from a flake using Nix CLI with dry-run by default, following authoritative flake conventions.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "flake_path": {
                                "type": "string",
                                "description": "Path or URL of flake"
                            },
                            "outputs": {
                                "type": "array",
                                "items": {
                                    "type": "string"
                                },
                                "description": "List of outputs to build"
                            },
                            "dry_run": {
                                "type": "boolean",
                                "description": "Perform dry-run build",
                                "default": true
                            }
                        },
                        "required": ["flake_path", "outputs"]
                    }
                },
                {
                    "name": "flake_scaffold",
                    "description": "Scaffold new flake projects, generate flake.nix files from templates, or add outputs to existing flakes.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "scaffold_type": {
                                "type": "string",
                                "enum": ["init", "generate", "addoutput", "addinput"],
                                "description": "Type of scaffolding operation"
                            },
                            "inputs": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "url": {"type": "string"}
                                    },
                                    "required": ["name", "url"]
                                },
                                "description": "Custom inputs to add (for addinput type or template customization)"
                            },
                            "version": {
                                "type": "string",
                                "description": "Version string for package (optional)"
                            },
                            "author": {
                                "type": "string",
                                "description": "Author name (optional)"
                            },
                            "template": {
                                "type": "string",
                                "enum": ["package", "devshell", "nixos", "multi"],
                                "description": "Template type to use"
                            },
                            "target_path": {
                                "type": "string",
                                "description": "Target directory or file path"
                            },
                            "name": {
                                "type": "string",
                                "description": "Project or package name (optional)"
                            },
                            "description": {
                                "type": "string",
                                "description": "Project description (optional)"
                            },
                            "overwrite": {
                                "type": "boolean",
                                "description": "Overwrite existing files (default: false)"
                            }
                        },
                        "required": ["scaffold_type", "target_path"]
                    }
                }
            ]);
            MCPResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(json!({ "tools": tools })),
                error: None,
                id: req.id,
            }
        }
        "tools/call" => {
            let params = match req.params {
                Some(p) => p,
                None => {
                    return MCPResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(MCPError {
                            code: -32602,
                            message: "Missing params".to_string(),
                        }),
                        id: req.id,
                    }
                }
            };

            let tool_name = match params.get("name").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return MCPResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(MCPError {
                            code: -32602,
                            message: "Missing tool name".to_string(),
                        }),
                        id: req.id,
                    };
                }
            };

            let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

            let result = match tool_name {
                "flake_inputs" => {
                    let request: FlakeInputsRequest = match serde_json::from_value(arguments) {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32602,
                                    message: format!("Invalid request: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    let response = match handle_flake_inputs_internal(request).await {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32000,
                                    message: format!("Nix error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    match serde_json::to_value(response) {
                        Ok(v) => v,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32603,
                                    message: format!("Serialization error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    }
                }
                "flake_outputs" => {
                    let request: FlakeOutputsRequest = match serde_json::from_value(arguments) {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32602,
                                    message: format!("Invalid request: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    let response = match handle_flake_outputs_internal(request).await {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32000,
                                    message: format!("Nix error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    match serde_json::to_value(response) {
                        Ok(v) => v,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32603,
                                    message: format!("Serialization error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    }
                }
                "flake_eval" => {
                    let request: FlakeEvalRequest = match serde_json::from_value(arguments) {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32602,
                                    message: format!("Invalid request: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    let response = match handle_flake_eval_internal(request).await {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32000,
                                    message: format!("Nix error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    match serde_json::to_value(response) {
                        Ok(v) => v,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32603,
                                    message: format!("Serialization error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    }
                }
                "flake_build" => {
                    let request: FlakeBuildRequest = match serde_json::from_value(arguments) {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32602,
                                    message: format!("Invalid request: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    let response = match handle_flake_build_internal(request).await {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32000,
                                    message: format!("Nix error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    match serde_json::to_value(response) {
                        Ok(v) => v,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32603,
                                    message: format!("Serialization error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    }
                }
                "flake_scaffold" => {
                    let request: FlakeScaffoldRequest = match serde_json::from_value(arguments) {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32602,
                                    message: format!("Invalid request: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    let response = match handle_flake_scaffold_internal(request).await {
                        Ok(r) => r,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32000,
                                    message: format!("Scaffold error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    };
                    match serde_json::to_value(response) {
                        Ok(v) => v,
                        Err(e) => {
                            return MCPResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(MCPError {
                                    code: -32603,
                                    message: format!("Serialization error: {}", e),
                                }),
                                id: req.id,
                            };
                        }
                    }
                }
                _ => {
                    return MCPResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(MCPError {
                            code: -32601,
                            message: format!("Unknown tool: {}", tool_name),
                        }),
                        id: req.id,
                    };
                }
            };

            MCPResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(result),
                error: None,
                id: req.id,
            }
        }
        _ => MCPResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(MCPError {
                code: -32601,
                message: format!("Unknown method: {}", req.method),
            }),
            id: req.id,
        }
    };

    response
}

pub async fn handle_mcp_request(req: MCPRequest) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&handle_mcp_request_internal(req).await))
}

async fn handle_flake_inputs_internal(req: FlakeInputsRequest) -> anyhow::Result<FlakeInputsResponse> {
    let metadata = NixCommand::flake_metadata(&req.flake_path)?;

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

    Ok(FlakeInputsResponse { inputs })
}

async fn handle_flake_outputs_internal(req: FlakeOutputsRequest) -> anyhow::Result<FlakeOutputsResponse> {
    let show_output = NixCommand::flake_show(&req.flake_path)?;

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

    Ok(FlakeOutputsResponse { outputs })
}

async fn handle_flake_eval_internal(req: FlakeEvalRequest) -> anyhow::Result<FlakeEvalResponse> {
    let (stdout, stderr) = NixCommand::eval(&req.flake_path, &req.expression, req.json_output)?;

    let result = EvalResult {
        result: stdout.trim().to_string(),
        success: true,
        logs: stderr,
    };

    Ok(FlakeEvalResponse { result })
}

async fn handle_flake_build_internal(req: FlakeBuildRequest) -> anyhow::Result<FlakeBuildResponse> {
    let (success, logs, errors, built_paths) = NixCommand::build(
        &req.flake_path,
        &req.outputs,
        req.dry_run,
    )?;

    let result = BuildResult {
        success,
        logs,
        errors,
        built_paths,
    };

    Ok(FlakeBuildResponse { result })
}

async fn handle_flake_scaffold_internal(req: FlakeScaffoldRequest) -> anyhow::Result<FlakeScaffoldResponse> {
    use crate::endpoints::flake_scaffold::handle_flake_scaffold_internal as scaffold_handler;
    
    let result = scaffold_handler(req).await?;
    Ok(FlakeScaffoldResponse { result })
}

#[derive(Debug)]
pub enum ServerError {
    InvalidParams(String),
    NixError(String),
    SerializationError(String),
}

impl warp::reject::Reject for ServerError {}

async fn handle_rejection(err: warp::Rejection) -> std::result::Result<impl warp::Reply, std::convert::Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = 404;
        message = "Not Found".to_string();
    } else if let Some(server_err) = err.find::<ServerError>() {
        match server_err {
            ServerError::InvalidParams(msg) => {
                code = 400;
                message = format!("Invalid parameters: {}", msg);
            }
            ServerError::NixError(msg) => {
                code = 500;
                message = format!("Nix error: {}", msg);
            }
            ServerError::SerializationError(msg) => {
                code = 500;
                message = format!("Serialization error: {}", msg);
            }
        }
    } else {
        code = 500;
        message = "Internal Server Error".to_string();
    }

    let json = warp::reply::json(&serde_json::json!({
        "error": {
            "code": code,
            "message": message
        }
    }));

    Ok(warp::reply::with_status(json, warp::http::StatusCode::from_u16(code).unwrap_or(warp::http::StatusCode::INTERNAL_SERVER_ERROR)))
}

pub fn create_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let mcp_route = warp::post()
        .and(warp::path("mcp"))
        .and(warp::body::json())
        .and_then(handle_mcp_request);

    let flake_inputs_route = warp::post()
        .and(warp::path("flake_inputs"))
        .and(warp::body::json())
        .and_then(|req: FlakeInputsRequest| async move {
            handle_flake_inputs_internal(req)
                .await
                .map(|r| warp::reply::json(&r))
                .map_err(|e| warp::reject::custom(ServerError::NixError(e.to_string())))
        });

    let flake_outputs_route = warp::post()
        .and(warp::path("flake_outputs"))
        .and(warp::body::json())
        .and_then(|req: FlakeOutputsRequest| async move {
            handle_flake_outputs_internal(req)
                .await
                .map(|r| warp::reply::json(&r))
                .map_err(|e| warp::reject::custom(ServerError::NixError(e.to_string())))
        });

    let flake_eval_route = warp::post()
        .and(warp::path("flake_eval"))
        .and(warp::body::json())
        .and_then(|req: FlakeEvalRequest| async move {
            handle_flake_eval_internal(req)
                .await
                .map(|r| warp::reply::json(&r))
                .map_err(|e| warp::reject::custom(ServerError::NixError(e.to_string())))
        });

    let flake_build_route = warp::post()
        .and(warp::path("flake_build"))
        .and(warp::body::json())
        .and_then(|req: FlakeBuildRequest| async move {
            handle_flake_build_internal(req)
                .await
                .map(|r| warp::reply::json(&r))
                .map_err(|e| warp::reject::custom(ServerError::NixError(e.to_string())))
        });

    let flake_scaffold_route = warp::post()
        .and(warp::path("flake_scaffold"))
        .and(warp::body::json())
        .and_then(|req: FlakeScaffoldRequest| async move {
            handle_flake_scaffold_internal(req)
                .await
                .map(|r| warp::reply::json(&r))
                .map_err(|e| warp::reject::custom(ServerError::NixError(e.to_string())))
        });

    mcp_route
        .or(flake_inputs_route)
        .or(flake_outputs_route)
        .or(flake_eval_route)
        .or(flake_build_route)
        .or(flake_scaffold_route)
}

