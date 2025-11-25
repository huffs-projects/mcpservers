//! MCP (Model Context Protocol) server implementation.
//! 
//! This module provides the stdio-based JSON-RPC 2.0 server that communicates
//! with MCP clients via standard input/output.

use crate::endpoints::{zsh_options, zsh_templates, zsh_validate, zsh_apply};
use crate::error::{MCPError, Result};
use crate::models::{ValidationResult, ApplyResult};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Mutex;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

#[derive(Debug, Deserialize)]
struct JSONRPCRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JSONRPCResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<crate::error::JSONRPCError>,
}

#[derive(Debug, Serialize)]
struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    protocol_version: String,
    capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    server_info: ServerInfo,
}

#[derive(Debug, Serialize)]
struct ServerCapabilities {
    tools: ToolsCapability,
}

#[derive(Debug, Serialize)]
struct ToolsCapability {}

#[derive(Debug, Serialize)]
struct ServerInfo {
    name: String,
    version: String,
}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[derive(Debug, Serialize)]
struct ToolCallResult {
    content: Vec<ContentItem>,
}

#[derive(Debug, Serialize)]
struct ContentItem {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

/// Cache for tools/list response (rarely changes, so we cache it)
static TOOLS_LIST_CACHE: Lazy<Mutex<Option<Value>>> = Lazy::new(|| Mutex::new(None));

/// Cache for initialize response (static, so we cache it)
static INITIALIZE_CACHE: Lazy<Mutex<Option<Value>>> = Lazy::new(|| Mutex::new(None));

/// Runs the MCP stdio server.
/// 
/// This function reads JSON-RPC 2.0 requests from stdin and writes responses to stdout.
/// It uses async I/O with buffering for optimal performance.
/// 
/// # Errors
/// 
/// Returns an error if there's an I/O error or JSON parsing error.
/// 
/// # Examples
/// 
/// ```no_run
/// use zsh_mcp_server::mcp;
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     mcp::run_stdio_server().await?;
///     Ok(())
/// }
/// ```
pub async fn run_stdio_server() -> Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let mut reader = BufReader::with_capacity(8192, stdin);
    let mut writer = BufWriter::with_capacity(8192, stdout);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let request: JSONRPCRequest = match serde_json::from_str(trimmed) {
                    Ok(req) => req,
                    Err(e) => {
                        let error_response = JSONRPCResponse {
                            jsonrpc: "2.0".to_string(),
                            id: Value::Number(serde_json::Number::from(0)),
                            result: None,
                            error: Some(
                                MCPError::ParseError(e.to_string()).to_jsonrpc_error(),
                            ),
                        };
                        let response_json = serde_json::to_string(&error_response)?;
                        writer.write_all(response_json.as_bytes()).await?;
                        writer.write_all(b"\n").await?;
                        writer.flush().await?;
                        continue;
                    }
                };

                if request.jsonrpc != "2.0" {
                    let error_response = JSONRPCResponse {
                        jsonrpc: "2.0".to_string(),
                        id: Value::Number(serde_json::Number::from(0)),
                        result: None,
                        error: Some(
                            MCPError::InvalidRequest("jsonrpc must be '2.0'".to_string())
                                .to_jsonrpc_error(),
                        ),
                    };
                    let response_json = serde_json::to_string(&error_response)?;
                    writer.write_all(response_json.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                    writer.flush().await?;
                    continue;
                }

                let response_id = match request.id {
                    Some(Value::Null) => Value::Number(serde_json::Number::from(0)),
                    Some(v) => v,
                    None => {
                        continue;
                    }
                };

                let method = request.method.clone();
                let params = request.params.clone();
                let response = match handle_request(method, params).await {
                    Ok(result) => JSONRPCResponse {
                        jsonrpc: "2.0".to_string(),
                        id: response_id,
                        result: Some(result),
                        error: None,
                    },
                    Err(e) => JSONRPCResponse {
                        jsonrpc: "2.0".to_string(),
                        id: response_id,
                        result: None,
                        error: Some(e.to_jsonrpc_error()),
                    },
                };

                let response_json = serde_json::to_string(&response)?;
                writer.write_all(response_json.as_bytes()).await?;
                writer.write_all(b"\n").await?;
                writer.flush().await?;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                }
                return Err(MCPError::FileError(e));
            }
        }
    }

    Ok(())
}

/// Handles incoming JSON-RPC requests by routing to appropriate handlers.
/// 
/// # Arguments
/// 
/// * `method` - The JSON-RPC method name
/// * `params` - Optional parameters for the method
/// 
/// # Errors
/// 
/// Returns `MethodNotFound` if the method is not recognized.
async fn handle_request(method: String, params: Option<Value>) -> Result<Value> {
    match method.as_str() {
        "initialize" => handle_initialize(params).await,
        "tools/list" => handle_tools_list().await,
        "tools/call" => handle_tools_call(params).await,
        _ => Err(MCPError::MethodNotFound(method)),
    }
}

/// Handles the `initialize` method.
/// 
/// Returns server capabilities and information. The response is cached
/// since it never changes during the server's lifetime.
async fn handle_initialize(_params: Option<Value>) -> Result<Value> {
    let mut cache = INITIALIZE_CACHE.lock().unwrap();
    if let Some(cached) = cache.as_ref() {
        return Ok(cached.clone());
    }

    let result = InitializeResult {
        protocol_version: "2024-11-05".to_string(),
        capabilities: ServerCapabilities {
            tools: ToolsCapability {},
        },
        server_info: ServerInfo {
            name: "zsh-mcp-server".to_string(),
            version: "1.0.0".to_string(),
        },
    };
    let json_str = serde_json::to_string(&result)?;
    let value: Value = serde_json::from_str(&json_str)?;
    *cache = Some(value.clone());
    Ok(value)
}

/// Handles the `tools/list` method.
/// 
/// Returns a list of all available tools with their input schemas.
/// The response is cached since tools don't change at runtime.
async fn handle_tools_list() -> Result<Value> {
    let mut cache = TOOLS_LIST_CACHE.lock().unwrap();
    if let Some(cached) = cache.as_ref() {
        return Ok(cached.clone());
    }

    let tools = vec![
        Tool {
            name: "zsh_options".to_string(),
            description: "List Zsh shell options, built-ins, and module options with metadata.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "search_term": {
                        "type": "string",
                        "description": "Filter by option name or description keyword"
                    },
                    "scope": {
                        "type": "string",
                        "description": "Filter by option scope (e.g. GLOBAL, BOURNEOPT, etc)"
                    }
                }
            }),
        },
        Tool {
            name: "zsh_templates".to_string(),
            description: "Generate snippet templates for Zsh configuration, including prompts, completions, module loading, and keybindings.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "use_case": {
                        "type": "string",
                        "description": "Use case, e.g. 'prompt', 'completion', 'vi-mode', 'keybindings'"
                    }
                }
            }),
        },
        Tool {
            name: "zsh_validate".to_string(),
            description: "Validate the current Zsh config file (`.zshrc` or related) for syntactic correctness and common misconfigurations.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["config_path"],
                "properties": {
                    "config_path": {
                        "type": "string",
                        "description": "Path to Zsh config file to validate"
                    }
                }
            }),
        },
        Tool {
            name: "zsh_apply".to_string(),
            description: "Apply configuration changes to Zsh config safely, supporting dry-run, diff preview, and backup.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["config_path", "patch"],
                "properties": {
                    "config_path": {
                        "type": "string",
                        "description": "Path to Zsh config file"
                    },
                    "patch": {
                        "type": "string",
                        "description": "Unified diff or structured patch content"
                    },
                    "dry_run": {
                        "type": "boolean",
                        "description": "Perform dry-run (default: true)",
                        "default": true
                    },
                    "backup_path": {
                        "type": "string",
                        "description": "Optional path for backup file"
                    }
                }
            }),
        },
    ];
    let result = serde_json::json!({ "tools": tools });
    *cache = Some(result.clone());
    Ok(result)
}

/// Handles the `tools/call` method.
/// 
/// Executes a tool with the provided arguments and returns the result
/// in MCP content format.
/// 
/// # Arguments
/// 
/// * `params` - Must contain `name` (tool name) and `arguments` (tool arguments)
/// 
/// # Errors
/// 
/// Returns `InvalidParams` if required parameters are missing.
/// Returns `ToolError` if the tool name is unknown.
async fn handle_tools_call(params: Option<Value>) -> Result<Value> {
    let params = params.ok_or_else(|| MCPError::InvalidParams("Missing params".to_string()))?;
    let params_obj = params
        .as_object()
        .ok_or_else(|| MCPError::InvalidParams("Params must be an object".to_string()))?;

    let name = params_obj
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MCPError::InvalidParams("Missing 'name' in params".to_string()))?;

    let empty_map = serde_json::Map::new();
    let arguments = params_obj
        .get("arguments")
        .and_then(|v| v.as_object())
        .unwrap_or(&empty_map);

    let result = match name {
        "zsh_options" => {
            let search_term = arguments
                .get("search_term")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let scope = arguments
                .get("scope")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let options = zsh_options::query_options(search_term, scope);
            serde_json::to_string(&options)?
        }
        "zsh_templates" => {
            let use_case = arguments
                .get("use_case")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let templates = zsh_templates::generate_templates(use_case);
            serde_json::to_string(&templates)?
        }
        "zsh_validate" => {
            let config_path = arguments
                .get("config_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| MCPError::InvalidParams("Missing 'config_path' in arguments".to_string()))?;
            let validation_result = match zsh_validate::validate_config(config_path) {
                Ok(result) => result,
                Err(e) => ValidationResult {
                    success: false,
                    errors: vec![e.to_string()],
                    warnings: vec![],
                    logs: format!("Error validating config: {}", e),
                },
            };
            serde_json::to_string(&validation_result)?
        }
        "zsh_apply" => {
            let config_path = arguments
                .get("config_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| MCPError::InvalidParams("Missing 'config_path' in arguments".to_string()))?;
            let patch = arguments
                .get("patch")
                .and_then(|v| v.as_str())
                .ok_or_else(|| MCPError::InvalidParams("Missing 'patch' in arguments".to_string()))?;
            let dry_run = arguments
                .get("dry_run")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let backup_path = arguments
                .get("backup_path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let apply_result = match zsh_apply::apply_patch(config_path, patch, dry_run, backup_path.as_deref()) {
                Ok(result) => result,
                Err(e) => {
                    tracing::error!("zsh_apply error: {}", e);
                    ApplyResult {
                        success: false,
                        diff_applied: String::new(),
                        backup_created: false,
                    }
                }
            };
            serde_json::to_string(&apply_result)?
        }
        _ => return Err(MCPError::ToolError(format!("Unknown tool: {}", name))),
    };

    let content = ToolCallResult {
        content: vec![ContentItem {
            content_type: "text".to_string(),
            text: result,
        }],
    };

    Ok(serde_json::to_value(content)?)
}

