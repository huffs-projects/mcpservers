//! Request handlers for MCP protocol methods
//!
//! This module contains handlers for all MCP protocol methods:
//! - `initialize` - Protocol initialization
//! - `tools/list` - List available tools
//! - `tools/call` - Execute tool calls

use crate::config;
use crate::endpoints::{mako_apply, mako_options, mako_templates, mako_validate};
use crate::mcp::errors::create_error_response;
use crate::mcp::protocol::{self, error_codes, InitializeResult, MCPResponse, ToolCallParams};
use crate::mcp::tools::get_all_tools;
use serde::Deserialize;
use serde_json::{json, Value};

/// Typed arguments for mako_options tool
///
/// Used for type-safe deserialization of tool arguments.
#[derive(Debug, Deserialize)]
pub struct MakoOptionsArgs {
    pub search_term: Option<String>,
}

/// Typed arguments for mako_templates tool
#[derive(Debug, Deserialize)]
pub struct MakoTemplatesArgs {
    pub use_case: Option<String>,
}

/// Typed arguments for mako_validate tool
#[derive(Debug, Deserialize)]
pub struct MakoValidateArgs {
    pub config_path: String,
}

/// Typed arguments for mako_apply tool
#[derive(Debug, Deserialize)]
pub struct MakoApplyArgs {
    pub config_path: String,
    pub patch: String,
    #[serde(default = "default_true")]
    pub dry_run: bool,
    pub backup_path: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Handle initialize request
///
/// Returns server capabilities and information including protocol version,
/// server name, and version.
///
/// # Arguments
///
/// * `params` - Initialize parameters (currently unused)
/// * `id` - Request ID for the response
///
/// # Returns
///
/// MCP response with server information and capabilities
pub fn handle_initialize(params: &Option<Value>, id: Value) -> MCPResponse {
    let _ = params; // Initialize params are not used in this implementation

    let result = InitializeResult {
        protocol_version: config::PROTOCOL_VERSION.to_string(),
        capabilities: protocol::Capabilities {
            tools: Value::Object(serde_json::Map::new()),
        },
        server_info: protocol::ServerInfo {
            name: config::SERVER_NAME.to_string(),
            version: config::SERVER_VERSION.to_string(),
        },
    };

    match serde_json::to_value(result) {
        Ok(value) => MCPResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(value),
            error: None,
        },
        Err(e) => create_error_response(
            id,
            error_codes::INTERNAL_ERROR,
            "Internal error".to_string(),
            Some(Value::String(format!("Failed to serialize initialize result: {}", e))),
        ),
    }
}

/// Handle tools/list request
///
/// Returns a list of all available tools with their names, descriptions,
/// and input schemas.
///
/// # Arguments
///
/// * `id` - Request ID for the response
///
/// # Returns
///
/// MCP response containing array of tool definitions
pub fn handle_tools_list(id: Value) -> MCPResponse {
    let tools = get_all_tools();

    MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(json!({ "tools": tools })),
        error: None,
    }
}

/// Handle tools/call request
///
/// Executes a tool call by deserializing arguments into typed structs,
/// calling the appropriate endpoint function, and wrapping the result
/// in MCP content format.
///
/// Supported tools:
/// - `mako_options` - List configuration options
/// - `mako_templates` - Generate config templates
/// - `mako_validate` - Validate config file
/// - `mako_apply` - Apply config patch
///
/// # Arguments
///
/// * `params` - Tool call parameters containing tool name and arguments
/// * `id` - Request ID for the response
///
/// # Returns
///
/// MCP response with tool execution result or error
pub fn handle_tools_call(params: &Option<Value>, id: Value) -> MCPResponse {
    let params = match params {
        Some(p) => p,
        None => {
            return create_error_response(
                id,
                error_codes::INVALID_PARAMS,
                "Invalid params".to_string(),
                Some(Value::String("Missing params".to_string())),
            );
        }
    };

    let tool_params: ToolCallParams = match serde_json::from_value(params.clone()) {
        Ok(p) => p,
        Err(e) => {
            return create_error_response(
                id,
                error_codes::INVALID_PARAMS,
                "Invalid params".to_string(),
                Some(Value::String(format!("Invalid params: {}", e))),
            );
        }
    };

    let result = match tool_params.name.as_str() {
        "mako_options" => {
            let args: MakoOptionsArgs = match serde_json::from_value(tool_params.arguments) {
                Ok(a) => a,
                Err(e) => {
                    return create_error_response(
                        id,
                        error_codes::INVALID_PARAMS,
                        "Invalid params".to_string(),
                        Some(Value::String(format!("Invalid mako_options arguments: {}", e))),
                    );
                }
            };
            let options = mako_options::get_mako_options(args.search_term.as_deref());
            match serde_json::to_value(options) {
                Ok(v) => v,
                Err(e) => {
                    return create_error_response(
                        id,
                        error_codes::INTERNAL_ERROR,
                        "Internal error".to_string(),
                        Some(Value::String(format!("Failed to serialize options: {}", e))),
                    );
                }
            }
        }
        "mako_templates" => {
            let args: MakoTemplatesArgs = match serde_json::from_value(tool_params.arguments) {
                Ok(a) => a,
                Err(e) => {
                    return create_error_response(
                        id,
                        error_codes::INVALID_PARAMS,
                        "Invalid params".to_string(),
                        Some(Value::String(format!("Invalid mako_templates arguments: {}", e))),
                    );
                }
            };
            let templates = mako_templates::get_templates(args.use_case.as_deref());
            match serde_json::to_value(templates) {
                Ok(v) => v,
                Err(e) => {
                    return create_error_response(
                        id,
                        error_codes::INTERNAL_ERROR,
                        "Internal error".to_string(),
                        Some(Value::String(format!("Failed to serialize templates: {}", e))),
                    );
                }
            }
        }
        "mako_validate" => {
            let args: MakoValidateArgs = match serde_json::from_value(tool_params.arguments) {
                Ok(a) => a,
                Err(e) => {
                    return create_error_response(
                        id,
                        error_codes::INVALID_PARAMS,
                        "Invalid params".to_string(),
                        Some(Value::String(format!("Invalid mako_validate arguments: {}", e))),
                    );
                }
            };

            match mako_validate::validate_config(&args.config_path) {
                Ok(result) => match serde_json::to_value(result) {
                    Ok(v) => v,
                    Err(e) => {
                        return create_error_response(
                            id,
                            error_codes::INTERNAL_ERROR,
                            "Internal error".to_string(),
                            Some(Value::String(format!("Failed to serialize validation result: {}", e))),
                        );
                    }
                },
                Err(e) => {
                    return create_error_response(
                        id,
                        error_codes::SERVER_ERROR,
                        "Validation error".to_string(),
                        Some(Value::String(format!("{}", e))),
                    );
                }
            }
        }
        "mako_apply" => {
            let args: MakoApplyArgs = match serde_json::from_value(tool_params.arguments) {
                Ok(a) => a,
                Err(e) => {
                    return create_error_response(
                        id,
                        error_codes::INVALID_PARAMS,
                        "Invalid params".to_string(),
                        Some(Value::String(format!("Invalid mako_apply arguments: {}", e))),
                    );
                }
            };

            match mako_apply::apply_patch(
                &args.config_path,
                &args.patch,
                args.dry_run,
                args.backup_path.as_deref(),
            ) {
                Ok(result) => match serde_json::to_value(result) {
                    Ok(v) => v,
                    Err(e) => {
                        return create_error_response(
                            id,
                            error_codes::INTERNAL_ERROR,
                            "Internal error".to_string(),
                            Some(Value::String(format!("Failed to serialize apply result: {}", e))),
                        );
                    }
                },
                Err(e) => {
                    return create_error_response(
                        id,
                        error_codes::SERVER_ERROR,
                        "Apply error".to_string(),
                        Some(Value::String(format!("{}", e))),
                    );
                }
            }
        }
        _ => {
            return create_error_response(
                id,
                error_codes::METHOD_NOT_FOUND,
                "Method not found".to_string(),
                Some(Value::String(format!("Unknown tool: {}", tool_params.name))),
            );
        }
    };

    // Wrap result in MCP content format
    let text_content = match serde_json::to_string(&result) {
        Ok(s) => s,
        Err(e) => {
            return create_error_response(
                id,
                error_codes::INTERNAL_ERROR,
                "Internal error".to_string(),
                Some(Value::String(format!("Failed to serialize result: {}", e))),
            );
        }
    };

    let content = json!([{
        "type": "text",
        "text": text_content
    }]);

    MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(json!({ "content": content })),
        error: None,
    }
}

