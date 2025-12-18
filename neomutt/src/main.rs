use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write};

mod error;
mod handlers;
mod models;
mod parser;
mod utils;
mod prompts;
mod resources;

use handlers::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value, // Required, never null for responses
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct McpTool {
    name: String,
    description: String,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct McpResource {
    uri: String,
    name: String,
    description: Option<String>,
    mime_type: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Initialize handlers
    let docs_handler = docs::DocsHandler::new();
    let config_gen_handler = config_gen::ConfigGenHandler::new();
    let config_validate_handler = config_validate::ConfigValidateHandler::new();
    let interactive_handler = interactive::InteractiveHandler::new();

    loop {
        buffer.clear();
        match stdin_lock.read_line(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let request: JsonRpcRequest = match serde_json::from_str(&buffer) {
                    Ok(req) => req,
                    Err(_) => continue,
                };

                // Skip notifications (requests without id) - they don't get responses
                if request.id.is_none() {
                    continue;
                }

                let response = handle_request(
                    &request,
                    &docs_handler,
                    &config_gen_handler,
                    &config_validate_handler,
                    &interactive_handler,
                );

                let response_json = serde_json::to_string(&response)?;
                writeln!(stdout, "{}", response_json)?;
                stdout.flush()?;
            }
            Err(_) => break,
        }
    }

    Ok(())
}

fn handle_request(
    request: &JsonRpcRequest,
    docs_handler: &docs::DocsHandler,
    config_gen_handler: &config_gen::ConfigGenHandler,
    config_validate_handler: &config_validate::ConfigValidateHandler,
    interactive_handler: &interactive::InteractiveHandler,
) -> JsonRpcResponse {
    // JSON-RPC responses must have a non-null id
    // Use 0 as default if id is missing or null (shouldn't happen for requests, but be safe)
    let id = request.id.clone().unwrap_or_else(|| Value::Number(serde_json::Number::from(0)));

    match request.method.as_str() {
        "initialize" => {
            let result = serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "resources": {
                        "subscribe": false,
                        "listChanged": true
                    },
                    "prompts": {
                        "listChanged": true
                    }
                },
                "serverInfo": {
                    "name": "neomutt-mcp-server",
                    "version": "0.1.0"
                }
            });
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(result),
                error: None,
            }
        }
        "tools/list" => {
            let tools = vec![
                McpTool {
                    name: "search_docs".to_string(),
                    description: "Search NeoMutt documentation".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "Search query"
                            }
                        },
                        "required": ["query"]
                    }),
                },
                McpTool {
                    name: "get_config_option".to_string(),
                    description: "Get details about a specific NeoMutt configuration option".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "option": {
                                "type": "string",
                                "description": "Configuration option name"
                            }
                        },
                        "required": ["option"]
                    }),
                },
                McpTool {
                    name: "get_guide_section".to_string(),
                    description: "Retrieve a specific guide section from neomutt.org".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "section": {
                                "type": "string",
                                "description": "Guide section name or URL"
                            }
                        },
                        "required": ["section"]
                    }),
                },
                McpTool {
                    name: "generate_config".to_string(),
                    description: "Generate a NeoMutt configuration file based on requirements".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "requirements": {
                                "type": "string",
                                "description": "Description of configuration requirements"
                            }
                        },
                        "required": ["requirements"]
                    }),
                },
                McpTool {
                    name: "add_account".to_string(),
                    description: "Add an email account configuration to a muttrc file".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "email": {"type": "string"},
                            "imap_server": {"type": "string"},
                            "imap_port": {"type": "number"},
                            "smtp_server": {"type": "string"},
                            "smtp_port": {"type": "number"},
                            "use_ssl": {"type": "boolean"}
                        },
                        "required": ["email", "imap_server", "smtp_server"]
                    }),
                },
                McpTool {
                    name: "add_feature".to_string(),
                    description: "Enable/configure specific NeoMutt features".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "feature": {
                                "type": "string",
                                "description": "Feature name (encryption, sidebar, notmuch, threading, colors, etc.)"
                            },
                            "gpg_key": {
                                "type": "string",
                                "description": "GPG key ID (for encryption feature)"
                            },
                            "format": {
                                "type": "string",
                                "description": "Custom format string (for index_format feature)"
                            },
                            "options": {
                                "type": "object",
                                "description": "Additional feature-specific options"
                            }
                        },
                        "required": ["feature"]
                    }),
                },
                McpTool {
                    name: "validate_config".to_string(),
                    description: "Validate a NeoMutt configuration file".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "config": {
                                "type": "string",
                                "description": "Configuration file content or path"
                            }
                        },
                        "required": ["config"]
                    }),
                },
                McpTool {
                    name: "check_options".to_string(),
                    description: "Verify option names and values in a configuration".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "config": {
                                "type": "string",
                                "description": "Configuration file content"
                            }
                        },
                        "required": ["config"]
                    }),
                },
                McpTool {
                    name: "lint_config".to_string(),
                    description: "Find common mistakes and suggest fixes in a configuration".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "config": {
                                "type": "string",
                                "description": "Configuration file content"
                            }
                        },
                        "required": ["config"]
                    }),
                },
                McpTool {
                    name: "setup_wizard".to_string(),
                    description: "Guided setup process for NeoMutt configuration".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "step": {
                                "type": "string",
                                "description": "Current step in the wizard"
                            }
                        }
                    }),
                },
                McpTool {
                    name: "suggest_config".to_string(),
                    description: "Suggest configurations based on use case".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "use_case": {
                                "type": "string",
                                "description": "Description of the use case"
                            }
                        },
                        "required": ["use_case"]
                    }),
                },
                McpTool {
                    name: "troubleshoot".to_string(),
                    description: "Help diagnose configuration issues".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "error": {
                                "type": "string",
                                "description": "Error message or issue description"
                            },
                            "config": {
                                "type": "string",
                                "description": "Configuration file content (optional)"
                            }
                        },
                        "required": ["error"]
                    }),
                },
            ];
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({ "tools": tools })),
                error: None,
            }
        }
        "tools/call" => {
            let params = request.params.as_ref().and_then(|p| p.as_object());
            let tool_name = params
                .and_then(|p| p.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let arguments = params.and_then(|p| p.get("arguments"));

            let result = match tool_name {
                "search_docs" => docs_handler.search_docs(arguments),
                "get_config_option" => docs_handler.get_config_option(arguments),
                "get_guide_section" => docs_handler.get_guide_section(arguments),
                "generate_config" => config_gen_handler.generate_config(arguments),
                "add_account" => config_gen_handler.add_account(arguments),
                "add_feature" => config_gen_handler.add_feature(arguments),
                "validate_config" => config_validate_handler.validate_config(arguments),
                "check_options" => config_validate_handler.check_options(arguments),
                "lint_config" => config_validate_handler.lint_config(arguments),
                "setup_wizard" => interactive_handler.setup_wizard(arguments),
                "suggest_config" => interactive_handler.suggest_config(arguments),
                "troubleshoot" => interactive_handler.troubleshoot(arguments),
                _ => Err(crate::error::McpError::UnknownMethod {
                    method: tool_name.to_string(),
                }),
            };

            match result {
                Ok(value) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(value),
                    error: None,
                },
                Err(e) => {
                    let (code, message) = match e {
                        crate::error::McpError::ParameterError { .. } => (-32602, e.to_string()),
                        crate::error::McpError::UnknownMethod { .. } => (-32601, e.to_string()),
                        _ => (-32000, e.to_string()),
                    };
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(JsonRpcError {
                            code,
                            message,
                            data: None,
                        }),
                    }
                }
            }
        }
        "resources/list" => {
            let resources = crate::resources::list_resources();
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "resources": resources
                })),
                error: None,
            }
        }
        "resources/read" => {
            let params = request.params.as_ref().and_then(|p| p.as_object());
            let uri = params
                .and_then(|p| p.get("uri"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if uri.is_empty() {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing 'uri' parameter".to_string(),
                        data: None,
                    }),
                };
            }

            match futures::executor::block_on(crate::resources::read_resource(uri)) {
                Ok(result) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::json!(result)),
                    error: None,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32603,
                        message: format!("Resource read error: {}", e),
                        data: None,
                    }),
                },
            }
        }
        "prompts/list" => {
            let prompts = crate::prompts::list_prompts();
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "prompts": prompts
                })),
                error: None,
            }
        }
        "prompts/get" => {
            let params = request.params.as_ref().and_then(|p| p.as_object());
            let name = params
                .and_then(|p| p.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if name.is_empty() {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing 'name' parameter".to_string(),
                        data: None,
                    }),
                };
            }

            let arguments = params.and_then(|p| p.get("arguments")).cloned();

            match futures::executor::block_on(crate::prompts::get_prompt(name, arguments)) {
                Ok(result) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::json!(result)),
                    error: None,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32603,
                        message: format!("Prompt get error: {}", e),
                        data: None,
                    }),
                },
            }
        }
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
        },
    }
}
