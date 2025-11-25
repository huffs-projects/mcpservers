use crate::models::*;
use crate::modules::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use anyhow::Result;
use tracing::{debug, error, info, instrument, span, trace, warn, Level};

#[derive(Debug, Serialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: Value, // Required, never null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

#[derive(Debug, Serialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value, // Required, always present
}

#[derive(Debug, Deserialize)]
struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    protocol_version: String,
    capabilities: Value,
    #[serde(rename = "clientInfo")]
    client_info: Value,
}

#[derive(Debug, Deserialize)]
struct ToolsCallParams {
    pub name: String,
    pub arguments: Value,
}

/// Convert id to Value, never null
fn normalize_id(id: Option<&Value>) -> Value {
    match id {
        Some(Value::Null) => Value::Number(serde_json::Number::from(0)),
        Some(v) => v.clone(),
        None => Value::Number(serde_json::Number::from(0)),
    }
}

/// Handle initialize request
fn handle_initialize(id: Value) -> MCPResponse {
    let result = serde_json::json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": "wofi-rust-mcp",
            "version": "1.1.0"
        }
    });

    MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(result),
        error: None,
    }
}

/// Handle tools/list request
fn handle_tools_list(id: Value) -> MCPResponse {
    let tools = vec![
        Tool {
            name: "wofi_config_locations".to_string(),
            description: "Returns Wofi config search paths in priority order".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        Tool {
            name: "wofi_options".to_string(),
            description: "Get all Wofi runtime options with optional filtering".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "filter": {
                        "type": "string",
                        "description": "Filter options by name, description, or type"
                    }
                },
                "required": []
            }),
        },
        Tool {
            name: "wofi_templates".to_string(),
            description: "Get configuration templates for common setups".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "useCase": {
                        "type": "string",
                        "description": "Filter templates by use case"
                    }
                },
                "required": []
            }),
        },
        Tool {
            name: "wofi_styles".to_string(),
            description: "Get CSS style rules and selectors".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "selector": {
                        "type": "string",
                        "description": "Filter by CSS selector"
                    }
                },
                "required": []
            }),
        },
        Tool {
            name: "wofi_modes".to_string(),
            description: "Get available Wofi modes (builtin and custom)".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "filter": {
                        "type": "string",
                        "description": "Filter modes by name or type"
                    }
                },
                "required": []
            }),
        },
        Tool {
            name: "wofi_validate".to_string(),
            description: "Validate Wofi config and CSS files".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "configPath": {
                        "type": "string",
                        "description": "Path to config file"
                    },
                    "cssPath": {
                        "type": "string",
                        "description": "Path to CSS file (optional)"
                    }
                },
                "required": ["configPath"]
            }),
        },
        Tool {
            name: "wofi_apply".to_string(),
            description: "Apply patches to config and CSS files with atomic writes".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "configPath": {
                        "type": "string",
                        "description": "Path to config file"
                    },
                    "cssPath": {
                        "type": "string",
                        "description": "Path to CSS file (optional)"
                    },
                    "patchConfig": {
                        "type": "string",
                        "description": "New config content"
                    },
                    "patchCss": {
                        "type": "string",
                        "description": "New CSS content (optional)"
                    },
                    "dryRun": {
                        "type": "boolean",
                        "description": "If true, only show diff without applying (default: true)"
                    }
                },
                "required": ["configPath", "patchConfig"]
            }),
        },
        Tool {
            name: "wofi_docs".to_string(),
            description: "Get documentation links for a keyword".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "keyword": {
                        "type": "string",
                        "description": "Configuration key, mode, or selector name"
                    }
                },
                "required": ["keyword"]
            }),
        },
    ];

    let result = serde_json::json!({
        "tools": tools
    });

    MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(result),
        error: None,
    }
}

/// Handle tools/call request
#[instrument(skip(params), fields(tool_name = params.name.as_str()))]
fn handle_tools_call(id: Value, params: ToolsCallParams) -> Result<MCPResponse> {
    let result = match params.name.as_str() {
        "wofi_config_locations" => {
            let locations = wofi_config_locations::get_config_locations();
            serde_json::to_value(locations)?
        }
        "wofi_options" => {
            let filter = params.arguments.get("filter")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let options = wofi_options::get_options(filter.as_deref());
            serde_json::to_value(options)?
        }
        "wofi_templates" => {
            let use_case = params.arguments.get("useCase")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let templates = wofi_templates::get_templates(use_case.as_deref());
            serde_json::to_value(templates)?
        }
        "wofi_styles" => {
            let selector = params.arguments.get("selector")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let styles = wofi_styles::get_styles(selector.as_deref());
            serde_json::to_value(styles)?
        }
        "wofi_modes" => {
            let filter = params.arguments.get("filter")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let modes = wofi_modes::get_modes(filter.as_deref());
            serde_json::to_value(modes)?
        }
        "wofi_validate" => {
            let config_path = params.arguments.get("configPath")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("configPath is required"))?;
            let css_path = params.arguments.get("cssPath")
                .and_then(|v| v.as_str())
                .map(PathBuf::from);
            let config_path = PathBuf::from(config_path);
            let result = wofi_validate::validate(&config_path, css_path.as_deref());
            serde_json::to_value(result)?
        }
        "wofi_apply" => {
            let config_path = params.arguments.get("configPath")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("configPath is required"))?;
            let css_path = params.arguments.get("cssPath")
                .and_then(|v| v.as_str())
                .map(PathBuf::from);
            let patch_config = params.arguments.get("patchConfig")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("patchConfig is required"))?;
            let patch_css = params.arguments.get("patchCss")
                .and_then(|v| v.as_str());
            let dry_run = params.arguments.get("dryRun")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            
            let config_path = PathBuf::from(config_path);
            let result = wofi_apply::apply(
                &config_path,
                css_path.as_deref(),
                patch_config,
                patch_css,
                dry_run,
            )?;
            serde_json::to_value(result)?
        }
        "wofi_docs" => {
            let keyword = params.arguments.get("keyword")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("keyword is required"))?;
            let docs = wofi_docs::get_docs(keyword);
            serde_json::json!(docs)
        }
        _ => {
            return Ok(MCPResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: format!("Unknown tool: {}", params.name),
                }),
            });
        }
    };

    // MCP tools/call response format
    let content = serde_json::json!([{
        "type": "text",
        "text": serde_json::to_string(&result)?
    }]);

    Ok(MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(serde_json::json!({
            "content": content
        })),
        error: None,
    })
}

/// Write response to stdout
fn write_response(mut stdout: impl Write, response: MCPResponse) -> Result<()> {
    let json = serde_json::to_string(&response)?;
    writeln!(stdout, "{}", json)?;
    stdout.flush()?;
    Ok(())
}

/// Main MCP stdio server loop
#[instrument]
pub fn run_stdio_server() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdin.lock();
    
    info!("Starting MCP stdio server");
    
    let mut request_counter = 0u64;
    
    loop {
        let mut buffer = String::new();
        let bytes_read = handle.read_line(&mut buffer)?;
        
        if bytes_read == 0 {
            info!("EOF reached, shutting down");
            break;
        }
        
        request_counter += 1;
        let request_id = request_counter;
        
        let span = span!(
            Level::DEBUG,
            "mcp_request",
            request_id = request_id
        );
        let _enter = span.enter();
        
        trace!("Received raw request: {}", buffer.trim());
        
        // Parse JSON-RPC request
        match serde_json::from_str::<Value>(&buffer) {
            Ok(json) => {
                let method = json.get("method").and_then(|m| m.as_str());
                let params = json.get("params");
                let id = normalize_id(json.get("id"));
                
                debug!(
                    method = method,
                    jsonrpc_id = ?id,
                    "Parsed JSON-RPC request"
                );
                
                // Handle notifications (requests without id) by skipping
                if json.get("id").is_none() && method.is_some() {
                    debug!("Skipping notification (no id)");
                    continue;
                }
                
                let response = match method {
                    Some("initialize") => {
                        debug!("Handling initialize request");
                        let _params: InitializeParams = serde_json::from_value(
                            params.cloned().unwrap_or(serde_json::json!({}))
                        )?;
                        handle_initialize(id)
                    }
                    Some("tools/list") => {
                        debug!("Handling tools/list request");
                        handle_tools_list(id)
                    }
                    Some("tools/call") => {
                        let call_params_result: Result<ToolsCallParams, _> = serde_json::from_value(
                            params.cloned().ok_or_else(|| anyhow::anyhow!("params required"))?
                        );
                        
                        match call_params_result {
                            Ok(call_params) => {
                                debug!(tool_name = call_params.name.as_str(), "Handling tools/call request");
                                let id_clone = id.clone();
                                match handle_tools_call(id, call_params) {
                                    Ok(resp) => {
                                        debug!("Tool call succeeded");
                                        resp
                                    }
                                    Err(e) => {
                                        error!(error = %e, "Tool call failed");
                                        MCPResponse {
                                            jsonrpc: "2.0".to_string(),
                                            id: id_clone,
                                            result: None,
                                            error: Some(MCPError {
                                                code: -32603,
                                                message: format!("Internal error: {}", e),
                                            }),
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                warn!(error = %e, "Invalid params for tools/call");
                                MCPResponse {
                                    jsonrpc: "2.0".to_string(),
                                    id,
                                    result: None,
                                    error: Some(MCPError {
                                        code: -32602,
                                        message: format!("Invalid params: {}", e),
                                    }),
                                }
                            }
                        }
                    }
                    _ => {
                        warn!(method = method, "Unknown method requested");
                        MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            id,
                            result: None,
                            error: Some(MCPError {
                                code: -32601,
                                message: format!("Unknown method: {}", method.unwrap_or("null")),
                            }),
                        }
                    }
                };
                
                trace!("Sending response");
                write_response(stdout.lock(), response)?;
            }
            Err(e) => {
                // Log parse errors to stderr (MCP requirement)
                error!(
                    error = %e,
                    request = buffer.trim(),
                    "Failed to parse JSON-RPC request"
                );
                
                // Try to send an error response if we can extract an id
                if let Ok(json) = serde_json::from_str::<Value>(&buffer) {
                    let id = normalize_id(json.get("id"));
                    let error_response = MCPResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(MCPError {
                            code: -32700,
                            message: format!("Parse error: {}", e),
                        }),
                    };
                    if let Err(write_err) = write_response(stdout.lock(), error_response) {
                        error!(error = %write_err, "Failed to write error response");
                    }
                }
            }
        }
    }
    
    info!("MCP server shutdown complete");
    Ok(())
}

