use crate::mcp::handlers::*;
use crate::mcp::protocol::{error_codes, Response};
use crate::mcp::tools::ToolRegistry;
use anyhow::Result;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader as TokioBufReader};

/// MCP Server for handling Waybar configuration management
///
/// The server communicates via stdio using JSON-RPC 2.0 protocol.
/// It handles initialization, tool listing, and tool execution requests.
pub struct McpServer {
    initialized: bool,
    tool_registry: ToolRegistry,
}

impl McpServer {
    /// Create a new MCP server instance
    ///
    /// Initializes the server with an empty tool registry that will
    /// be populated with all available Waybar tools.
    pub fn new() -> Self {
        Self {
            initialized: false,
            tool_registry: ToolRegistry::new(),
        }
    }

    /// Handle a JSON-RPC request
    ///
    /// Routes the request to the appropriate handler based on the method.
    /// Supports: `initialize`, `tools/list`, and `tools/call`.
    ///
    /// # Arguments
    /// * `request` - The JSON-RPC request as a Value
    ///
    /// # Returns
    /// A Response appropriate for the request, or an error if the request is invalid
    pub async fn handle_request(&mut self, request: &Value) -> Result<Response> {
        let response_id = Response::extract_id(request);

        // Handle initialization
        if let Some(method) = request.get("method").and_then(|m| m.as_str()) {
            if method == "initialize" {
                self.initialized = true;
                return Ok(handle_initialize(response_id));
            }

            if method == "tools/list" {
                return Ok(handle_tools_list(response_id, &self.tool_registry));
            }

            if method == "tools/call" {
                let params = request.get("params")
                    .ok_or_else(|| anyhow::anyhow!("Missing params"))?;
                
                let name = params
                    .get("name")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing tool name"))?;

                let arguments = params
                    .get("arguments")
                    .cloned()
                    .unwrap_or(serde_json::json!({}));

                match handle_tools_call(response_id.clone(), name, &arguments).await {
                    Ok(response) => return Ok(response),
                    Err(e) => {
                        return Ok(Response::error(
                            response_id,
                            error_codes::INVALID_PARAMS,
                            format!("Tool execution failed: {}", e),
                            Some(serde_json::json!({
                                "tool": name,
                                "arguments": arguments,
                                "error": e.to_string()
                            })),
                        ));
                    }
                }
            }
        }

        Ok(Response::error(
            response_id,
            error_codes::METHOD_NOT_FOUND,
            "Method not found".to_string(),
            None,
        ))
    }

    /// Run the MCP server loop (stdio-based)
    ///
    /// Reads JSON-RPC requests from stdin and writes responses to stdout.
    /// Logs to stderr to avoid interfering with protocol communication.
    /// Continues until stdin is closed (EOF).
    ///
    /// # Returns
    /// Ok(()) on normal shutdown, or an error if something goes wrong
    pub async fn run(&mut self) -> Result<()> {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        let mut stdin_reader = TokioBufReader::new(stdin);
        let mut stdout_writer = stdout;

        loop {
            let mut line = String::new();
            let bytes_read = stdin_reader.read_line(&mut line).await?;

            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let request: Value = match serde_json::from_str(trimmed) {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("Failed to parse request: {}", e);
                    // Return proper JSON-RPC parse error response
                    let parse_error_id = Response::extract_id(&serde_json::json!({}));
                    let error_response = Response::error(
                        parse_error_id,
                        error_codes::PARSE_ERROR,
                        format!("Parse error: {}", e),
                        Some(serde_json::json!({
                            "original_request": trimmed
                        })),
                    );
                    let response_json = serde_json::to_string(&error_response)?;
                    stdout_writer.write_all(response_json.as_bytes()).await?;
                    stdout_writer.write_all(b"\n").await?;
                    stdout_writer.flush().await?;
                    continue;
                }
            };

            // Handle notifications (requests without id) - skip response
            let is_notification = !request.get("id").is_some();
            
            let response = match self.handle_request(&request).await {
                Ok(r) => r,
                Err(e) => {
                    // Convert anyhow error to JSON-RPC internal error
                    let response_id = Response::extract_id(&request);
                    Response::error(
                        response_id,
                        error_codes::INTERNAL_ERROR,
                        format!("Internal error: {}", e),
                        Some(serde_json::json!({
                            "error_chain": e.chain().map(|e| e.to_string()).collect::<Vec<_>>()
                        })),
                    )
                }
            };

            // Only send response if this is not a notification
            if !is_notification {
                let response_json = serde_json::to_string(&response)?;
                stdout_writer.write_all(response_json.as_bytes()).await?;
                stdout_writer.write_all(b"\n").await?;
                stdout_writer.flush().await?;
            }
        }

        Ok(())
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

