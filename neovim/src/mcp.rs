use serde_json::{json, Value};
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info, instrument, span, warn, Level};
use crate::endpoints::*;

/// MCP JSON-RPC request
#[derive(Debug, serde::Deserialize)]
struct MCPRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// MCP JSON-RPC response
#[derive(Debug, serde::Serialize)]
struct MCPResponse {
    jsonrpc: String,
    id: Value, // Required, never null
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<MCPError>,
}

/// MCP error object
#[derive(Debug, serde::Serialize)]
struct MCPError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// MCP Tool definition
#[derive(Debug, serde::Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value, // Required, always present
}

/// Run the MCP stdio server
#[instrument]
pub async fn run_stdio_server() -> Result<(), Box<dyn std::error::Error>> {
    // Log to stderr (MCP requirement)
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting Neovim MCP server");

    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin);
    let mut stdout = tokio::io::stdout();
    let mut line = String::new();

    // Initialize endpoints with shared instances (Arc/Mutex for thread safety)
    let options_endpoint = std::sync::Arc::new(OptionsEndpoint::new());
    let templates_endpoint = std::sync::Arc::new(TemplatesEndpoint::new());
    let validate_endpoint = std::sync::Arc::new(tokio::sync::Mutex::new(ValidateEndpoint::new()));
    let apply_endpoint = std::sync::Arc::new(tokio::sync::Mutex::new(ApplyEndpoint::new()));
    let discover_endpoint = std::sync::Arc::new(DiscoverEndpoint::new());

    loop {
        line.clear();
        let bytes_read = stdin_reader.read_line(&mut line).await?;
        
        if bytes_read == 0 {
            info!("EOF received, shutting down");
            break; // EOF
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let start_time = Instant::now();
        let request_id = extract_request_id(trimmed);

        // Parse JSON-RPC request
        let request: MCPRequest = match serde_json::from_str::<MCPRequest>(trimmed) {
            Ok(req) => {
                debug!(
                    request_id = ?request_id,
                    method = %req.method,
                    "Received request"
                );
                req
            }
            Err(e) => {
                error!(
                    request_id = ?request_id,
                    error = %e,
                    "Failed to parse JSON-RPC request"
                );
                let error_response = MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id: json!(0),
                    result: None,
                    error: Some(MCPError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                        data: Some(json!({
                            "parse_error": e.to_string(),
                            "input_length": trimmed.len()
                        })),
                    }),
                };
                let response_json = serde_json::to_string(&error_response)?;
                stdout.write_all(response_json.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                continue;
            }
        };

        // Validate request structure
        if let Err(validation_error) = validate_request(&request) {
            error!(
                request_id = ?request_id,
                method = %request.method,
                error_code = validation_error.code,
                "Request validation failed"
            );
            let error_response = MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id.unwrap_or_else(|| json!(0)),
                result: None,
                error: Some(validation_error),
            };
            let response_json = serde_json::to_string(&error_response)?;
            stdout.write_all(response_json.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
            continue;
        }

        // Handle notifications (requests without id) and special notification methods
        if request.method == "initialized" {
            debug!("Received initialized notification");
            continue;
        }

        let response_id = match request.id {
            Some(Value::Null) => Value::Number(serde_json::Number::from(0)),
            Some(v) => v.clone(),
            None => {
                debug!(method = %request.method, "Notification received, no response needed");
                continue;
            }
        };

        let method_span = span!(
            Level::DEBUG,
            "handle_request",
            method = %request.method,
            request_id = ?response_id
        );
        let _enter = method_span.enter();

        // Route to appropriate handler
        let result = match request.method.as_str() {
            "initialize" => {
                info!("Handling initialize request");
                handle_initialize(request.params)
            }
            "tools/list" => {
                info!("Handling tools/list request");
                handle_tools_list()
            }
            "tools/call" => {
                handle_tools_call(
                    request.params,
                    options_endpoint.clone(),
                    templates_endpoint.clone(),
                    validate_endpoint.clone(),
                    apply_endpoint.clone(),
                    discover_endpoint.clone(),
                ).await
            }
            _ => {
                warn!(method = %request.method, "Unknown method requested");
                Err(MCPError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: Some(json!({
                        "available_methods": ["initialize", "tools/list", "tools/call"]
                    })),
                })
            }
        };

        let elapsed = start_time.elapsed();

        // Build response
        let response = match result {
            Ok(value) => {
                info!(
                    method = %request.method,
                    duration_ms = elapsed.as_millis(),
                    "Request completed successfully"
                );
                MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id: response_id,
                    result: Some(value),
                    error: None,
                }
            }
            Err(error) => {
                error!(
                    method = %request.method,
                    error_code = error.code,
                    error_message = %error.message,
                    duration_ms = elapsed.as_millis(),
                    "Request failed"
                );
                MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id: response_id,
                    result: None,
                    error: Some(error),
                }
            }
        };

        // Write response
        let response_json = serde_json::to_string(&response)?;
        stdout.write_all(response_json.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        debug!(
            method = %request.method,
            response_size = response_json.len(),
            "Response sent"
        );
    }

    Ok(())
}

/// Extract request ID from JSON string for logging purposes
fn extract_request_id(json_str: &str) -> Option<Value> {
    serde_json::from_str::<Value>(json_str)
        .ok()
        .and_then(|v| v.get("id").cloned())
}

/// Validate JSON-RPC request structure
fn validate_request(request: &MCPRequest) -> Result<(), MCPError> {
    // Validate jsonrpc version
    if request.jsonrpc != "2.0" {
        return Err(MCPError {
            code: -32600,
            message: format!("Invalid jsonrpc version: {}. Expected '2.0'", request.jsonrpc),
            data: Some(json!({
                "received_version": request.jsonrpc,
                "expected_version": "2.0"
            })),
        });
    }

    // Validate method is not empty
    if request.method.is_empty() {
        return Err(MCPError {
            code: -32600,
            message: "Method cannot be empty".to_string(),
            data: Some(json!({
                "field": "method"
            })),
        });
    }

    // Validate method name format (should not start with $ unless it's a notification)
    if request.method.starts_with('$') && request.id.is_some() {
        return Err(MCPError {
            code: -32600,
            message: format!("Method '{}' is a notification and cannot have an id", request.method),
            data: Some(json!({
                "method": request.method,
                "is_notification": true
            })),
        });
    }

    // Validate params structure if present
    if let Some(ref params) = request.params {
        if !params.is_object() && !params.is_array() && !params.is_null() {
            return Err(MCPError {
                code: -32602,
                message: "Params must be an object, array, or null".to_string(),
                data: Some(json!({
                    "received_type": format!("{:?}", params),
                    "expected_types": ["object", "array", "null"]
                })),
            });
        }
    }

    Ok(())
}

/// Validate tool call parameters
fn validate_tool_call_params(params: &Value) -> Result<(), MCPError> {
    let params_obj = params.as_object().ok_or_else(|| MCPError {
        code: -32602,
        message: "Params must be an object for tools/call".to_string(),
        data: Some(json!({
            "expected_type": "object"
        })),
    })?;

    // Check for required 'name' field
    if !params_obj.contains_key("name") {
        return Err(MCPError {
            code: -32602,
            message: "Missing required field 'name' in params".to_string(),
            data: Some(json!({
                "required_fields": ["name"],
                "received_fields": params_obj.keys().collect::<Vec<_>>()
            })),
        });
    }

    // Validate name is a string
    let name = params_obj.get("name").unwrap();
    if !name.is_string() {
        return Err(MCPError {
            code: -32602,
            message: "Field 'name' must be a string".to_string(),
            data: Some(json!({
                "field": "name",
                "received_type": format!("{:?}", name)
            })),
        });
    }

    // Validate arguments if present
    if let Some(arguments) = params_obj.get("arguments") {
        if !arguments.is_object() {
            return Err(MCPError {
                code: -32602,
                message: "Field 'arguments' must be an object".to_string(),
                data: Some(json!({
                    "field": "arguments",
                    "received_type": format!("{:?}", arguments)
                })),
            });
        }
    }

    Ok(())
}

/// Handle initialize request
fn handle_initialize(_params: Option<Value>) -> Result<Value, MCPError> {
    Ok(json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": "neovim-mcp-server",
            "version": "1.0.0"
        }
    }))
}

/// Handle tools/list request
fn handle_tools_list() -> Result<Value, MCPError> {
    let tools = vec![
        Tool {
            name: "nvim_options".to_string(),
            description: "Returns a full database of Neovim option definitions, derived from runtime documentation + API metadata.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "search": {
                        "type": "string",
                        "description": "Search options by name or description"
                    },
                    "scope": {
                        "type": "string",
                        "description": "Filter by scope (global, window, buffer)",
                        "enum": ["global", "window", "buffer"]
                    }
                }
            }),
        },
        Tool {
            name: "nvim_templates".to_string(),
            description: "Generate idiomatic Neovim config snippets. Supports LazyVim-specific templates.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "use_case": {
                        "type": "string",
                        "description": "Template use case (e.g., lazyvim_keymap, lazyvim_plugin, lsp_config)"
                    },
                    "parameters": {
                        "type": "object",
                        "description": "Template parameters",
                        "additionalProperties": true
                    }
                },
                "required": ["use_case"]
            }),
        },
        Tool {
            name: "nvim_validate".to_string(),
            description: "Perform multi-stage validation: syntax, semantic, LazyVim plugin tree validation, and runtime path validation.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "config_roots": {
                        "type": "array",
                        "items": {
                            "type": "string"
                        },
                        "description": "List of Neovim config root directories to validate"
                    }
                },
                "required": ["config_roots"]
            }),
        },
        Tool {
            name: "nvim_apply".to_string(),
            description: "Apply safe patches to any Neovim config file. Uses AST-based patch merging and unified diff generation with backup.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "Path to the config file to modify"
                    },
                    "patch": {
                        "type": "string",
                        "description": "Unified diff or AST patch instruction"
                    },
                    "dry_run": {
                        "type": "boolean",
                        "description": "If true, validate but don't apply changes",
                        "default": true
                    }
                },
                "required": ["file_path", "patch"]
            }),
        },
        Tool {
            name: "nvim_discover".to_string(),
            description: "Detect Neovim config root using XDG paths or ~/.config/nvim. Identify init.lua, lua/, plugin/, after/, and LazyVim plugin files.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
    ];

    Ok(json!({
        "tools": tools
    }))
}

/// Handle tools/call request
#[instrument(skip_all)]
async fn handle_tools_call(
    params: Option<Value>,
    options_endpoint: std::sync::Arc<OptionsEndpoint>,
    templates_endpoint: std::sync::Arc<TemplatesEndpoint>,
    validate_endpoint: std::sync::Arc<tokio::sync::Mutex<ValidateEndpoint>>,
    apply_endpoint: std::sync::Arc<tokio::sync::Mutex<ApplyEndpoint>>,
    discover_endpoint: std::sync::Arc<DiscoverEndpoint>,
) -> Result<Value, MCPError> {
    let params = params.ok_or_else(|| MCPError {
        code: -32602,
        message: "Missing params for tools/call".to_string(),
        data: Some(json!({
            "method": "tools/call",
            "required": "params"
        })),
    })?;

    // Validate tool call parameters
    validate_tool_call_params(&params)?;

    let params_obj = params.as_object().unwrap(); // Safe because validate_tool_call_params ensures it's an object
    let tool_name = params_obj
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap(); // Safe because validate_tool_call_params ensures it exists and is a string

    let arguments = params_obj.get("arguments").cloned().unwrap_or(json!({}));
    
    info!(tool_name = %tool_name, "Executing tool call");

    // Route to appropriate tool handler
    let result = match tool_name {
        "nvim_options" => {
            let query: OptionsQuery = serde_json::from_value(arguments)
                .map_err(|e| {
                    error!(tool_name = "nvim_options", error = %e, "Invalid arguments");
                    MCPError {
                        code: -32602,
                        message: format!("Invalid arguments: {}", e),
                        data: Some(json!({
                            "tool": "nvim_options",
                            "parse_error": e.to_string()
                        })),
                    }
                })?;
            
            debug!(tool_name = "nvim_options", "Calling endpoint");
            options_endpoint.handle_query(query).await
                .map(|options| json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string(&options).unwrap_or_default()
                    }]
                }))
                .map_err(|e| {
                    error!(tool_name = "nvim_options", error = %e, "Tool execution failed");
                    MCPError {
                        code: -32000,
                        message: e,
                        data: Some(json!({
                            "tool": "nvim_options"
                        })),
                    }
                })
        }
        "nvim_templates" => {
            let query: TemplatesQuery = serde_json::from_value(arguments)
                .map_err(|e| {
                    error!(tool_name = "nvim_templates", error = %e, "Invalid arguments");
                    MCPError {
                        code: -32602,
                        message: format!("Invalid arguments: {}", e),
                        data: Some(json!({
                            "tool": "nvim_templates",
                            "parse_error": e.to_string()
                        })),
                    }
                })?;
            
            debug!(tool_name = "nvim_templates", "Calling endpoint");
            templates_endpoint.handle_query(query).await
                .map(|templates| json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string(&templates).unwrap_or_default()
                    }]
                }))
                .map_err(|e| {
                    error!(tool_name = "nvim_templates", error = %e, "Tool execution failed");
                    MCPError {
                        code: -32000,
                        message: e,
                        data: Some(json!({
                            "tool": "nvim_templates"
                        })),
                    }
                })
        }
        "nvim_validate" => {
            let query: ValidateQuery = serde_json::from_value(arguments)
                .map_err(|e| {
                    error!(tool_name = "nvim_validate", error = %e, "Invalid arguments");
                    MCPError {
                        code: -32602,
                        message: format!("Invalid arguments: {}", e),
                        data: Some(json!({
                            "tool": "nvim_validate",
                            "parse_error": e.to_string()
                        })),
                    }
                })?;
            
            debug!(tool_name = "nvim_validate", "Calling endpoint");
            let mut endpoint = validate_endpoint.lock().await;
            endpoint.handle_query(query).await
                .map(|result| json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string(&result).unwrap_or_default()
                    }]
                }))
                .map_err(|e| {
                    error!(tool_name = "nvim_validate", error = %e, "Tool execution failed");
                    MCPError {
                        code: -32000,
                        message: e,
                        data: Some(json!({
                            "tool": "nvim_validate"
                        })),
                    }
                })
        }
        "nvim_apply" => {
            let query: ApplyQuery = serde_json::from_value(arguments)
                .map_err(|e| {
                    error!(tool_name = "nvim_apply", error = %e, "Invalid arguments");
                    MCPError {
                        code: -32602,
                        message: format!("Invalid arguments: {}", e),
                        data: Some(json!({
                            "tool": "nvim_apply",
                            "parse_error": e.to_string()
                        })),
                    }
                })?;
            
            debug!(tool_name = "nvim_apply", file_path = %query.file_path, "Calling endpoint");
            let mut endpoint = apply_endpoint.lock().await;
            endpoint.handle_query(query).await
                .map(|result| json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string(&result).unwrap_or_default()
                    }]
                }))
                .map_err(|e| {
                    error!(tool_name = "nvim_apply", error = %e, "Tool execution failed");
                    MCPError {
                        code: -32000,
                        message: e,
                        data: Some(json!({
                            "tool": "nvim_apply"
                        })),
                    }
                })
        }
        "nvim_discover" => {
            debug!(tool_name = "nvim_discover", "Calling endpoint");
            discover_endpoint.handle_query().await
                .map(|paths| json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string(&paths).unwrap_or_default()
                    }]
                }))
                .map_err(|e| {
                    error!(tool_name = "nvim_discover", error = %e, "Tool execution failed");
                    MCPError {
                        code: -32000,
                        message: e,
                        data: Some(json!({
                            "tool": "nvim_discover"
                        })),
                    }
                })
        }
        _ => {
            warn!(tool_name = %tool_name, "Unknown tool requested");
            Err(MCPError {
                code: -32601,
                message: format!("Unknown tool: {}", tool_name),
                data: Some(json!({
                    "available_tools": ["nvim_options", "nvim_templates", "nvim_validate", "nvim_apply", "nvim_discover"]
                })),
            })
        },
    };

    result
}

