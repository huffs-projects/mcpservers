use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use crate::tools::ToolRegistry;
use crate::error::MCPError as ServerError;
use std::sync::Arc;
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize)]
struct MCPRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct MCPResponse {
    jsonrpc: String,
    id: Value, // Required, never null
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<MCPError>,
}

#[derive(Debug, Serialize)]
struct MCPError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

// Global tool registry (initialized once)
static TOOL_REGISTRY: Lazy<Arc<ToolRegistry>> = Lazy::new(|| Arc::new(ToolRegistry::new()));

pub async fn run_stdio_server() {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    let mut line = String::new();

    // Log to stderr (MCP requirement - stdout is for JSON-RPC)
    eprintln!("Kitty MCP Server starting (stdio mode)");

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                // Parse JSON-RPC request
                let request: MCPRequest = match serde_json::from_str(trimmed) {
                    Ok(req) => req,
                    Err(e) => {
                        // Try to extract id from malformed JSON for better error response
                        let error_id = serde_json::from_str::<serde_json::Value>(trimmed)
                            .ok()
                            .and_then(|v| v.get("id").cloned())
                            .unwrap_or_else(|| Value::Number(serde_json::Number::from(0)));
                        
                        let error_response = MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            id: error_id,
                            result: None,
                            error: Some(MCPError {
                                code: -32700,
                                message: format!("Parse error: {}", e),
                                data: None,
                            }),
                        };
                        send_response(&error_response).await;
                        continue;
                    }
                };

                // Handle request
                if let Some(response) = handle_request(request).await {
                    send_response(&response).await;
                }
                // For notifications (no id), don't send a response
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }
}

async fn send_response(response: &MCPResponse) {
    let json = match serde_json::to_string(response) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Error serializing response: {}", e);
            return;
        }
    };
    let mut stdout = io::stdout();
    if let Err(e) = stdout.write_all(json.as_bytes()).await {
        eprintln!("Error writing to stdout: {}", e);
    }
    if let Err(e) = stdout.write_all(b"\n").await {
        eprintln!("Error writing newline to stdout: {}", e);
    }
    if let Err(e) = stdout.flush().await {
        eprintln!("Error flushing stdout: {}", e);
    }
}

async fn handle_request(request: MCPRequest) -> Option<MCPResponse> {
    // Convert id to Value (never null)
    let response_id = match &request.id {
        Some(Value::Null) => Value::Number(serde_json::Number::from(0)),
        Some(v) => v.clone(),
        None => Value::Number(serde_json::Number::from(0)),
    };

    // Handle notifications (requests without id) - don't send response
    if request.id.is_none() {
        return None;
    }

    Some(match request.method.as_str() {
        "initialize" => handle_initialize(request.params, response_id),
        "tools/list" => handle_tools_list(response_id),
        "tools/call" => handle_tools_call(request.params, response_id).await,
        _ => MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: response_id,
            result: None,
            error: Some(MCPError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
        },
    })
}

fn handle_initialize(_params: Value, id: Value) -> MCPResponse {
    MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "kitty-mcp-server",
                "version": "rust-1.1"
            }
        })),
        error: None,
    }
}

fn handle_tools_list(id: Value) -> MCPResponse {
    let tools = TOOL_REGISTRY.list_tools();

    MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(json!({
            "tools": tools
        })),
        error: None,
    }
}

async fn handle_tools_call(params: Value, id: Value) -> MCPResponse {
    let tool_name = params.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let arguments = params.get("arguments")
        .cloned()
        .unwrap_or(json!({}));

    // Get tool from registry
    let tool = match TOOL_REGISTRY.get(tool_name) {
        Some(t) => t,
        None => {
            return MCPResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: format!("Unknown tool: {}", tool_name),
                    data: None,
                }),
            };
        }
    };

    // Execute tool
    match tool.execute(arguments).await {
        Ok(result) => {
            // Wrap result in MCP content format
            MCPResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string(&result)
                            .unwrap_or_else(|_| "{}".to_string())
                    }]
                })),
                error: None,
            }
        }
        Err(e) => {
            let (code, message, data) = ServerError::ToolExecution(e).to_mcp_error();
            MCPResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(MCPError {
                    code,
                    message,
                    data,
                }),
            }
        }
    }
}

