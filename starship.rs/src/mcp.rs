use crate::endpoints::{
    starship_apply::{ApplyEndpoint, ApplyRequest},
    starship_options::{OptionsEndpoint, OptionsQuery},
    starship_presets::{PresetsEndpoint, PresetsQuery},
    starship_templates::{TemplatesEndpoint, TemplatesQuery},
    starship_validate::{ValidateEndpoint, ValidateRequest},
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[derive(Debug, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum MCPRequest {
    Initialize {
        params: InitializeParams,
    },
    ToolsList,
    ToolsCall {
        params: ToolsCallParams,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: Value,
    pub client_info: Value,
}

#[derive(Debug, Deserialize)]
pub struct ToolsCallParams {
    pub name: String,
    pub arguments: Value,
}

#[derive(Debug, Serialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: Value,  // Required, never null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

#[derive(Debug, Serialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

pub async fn run_stdio_server() -> Result<()> {
    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = tokio::io::BufReader::new(stdin);
    let mut line = String::new();
    let mut initialized = false;

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await
            .context("Failed to read line from stdin")?;
        
        if bytes_read == 0 {
            break; // EOF
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let request: Value = serde_json::from_str(trimmed)
            .context("Failed to parse JSON request")?;

        let id = request.get("id").cloned();
        let method = request
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or("");

        // Skip notifications (requests without id) - they don't need responses
        if id.is_none() && !method.is_empty() {
            log::debug!("Skipping notification: {}", method);
            continue;
        }

        // Ensure id is never null - use 0 as default if missing
        let response_id = match id {
            Some(Value::Null) => Value::Number(serde_json::Number::from(0)),
            Some(v) => v,
            None => Value::Number(serde_json::Number::from(0)),
        };

        let response = match method {
            "initialize" => {
                if initialized {
                    continue;
                }
                initialized = true;
                handle_initialize(&request, response_id).await
            }
            "tools/list" => handle_tools_list(response_id).await,
            "tools/call" => {
                let params = serde_json::from_value(
                    request
                        .get("params")
                        .cloned()
                        .unwrap_or(Value::Null),
                )
                .context("Failed to parse tools/call params")?;
                handle_tools_call(params, response_id).await
            }
            _ => MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: response_id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: format!("Method not found: {}", method),
                    data: None,
                }),
            },
        };

        let response_json = serde_json::to_string(&response)
            .context("Failed to serialize response")?;
        stdout.write_all(response_json.as_bytes()).await
            .context("Failed to write response")?;
        stdout.write_all(b"\n").await
            .context("Failed to write newline")?;
        stdout.flush().await
            .context("Failed to flush stdout")?;
    }

    Ok(())
}

async fn handle_initialize(_request: &Value, id: Value) -> MCPResponse {
    MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "starship-mcp-server",
                "version": "1.1.0"
            }
        })),
        error: None,
    }
}

async fn handle_tools_list(id: Value) -> MCPResponse {
    let tools = vec![
        Tool {
            name: "starship_options".to_string(),
            description: "Query Starship configuration options".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "search_term": {"type": "string"},
                    "category": {"type": "string"}
                }
            }),
        },
        Tool {
            name: "starship_presets".to_string(),
            description: "Query available Starship presets".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "preset_name": {"type": "string"}
                }
            }),
        },
        Tool {
            name: "starship_templates".to_string(),
            description: "Generate Starship configuration templates".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "category": {"type": "string"},
                    "use_case": {"type": "string"}
                }
            }),
        },
        Tool {
            name: "starship_validate".to_string(),
            description: "Validate a Starship configuration file".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "config_path": {"type": "string"}
                },
                "required": ["config_path"]
            }),
        },
        Tool {
            name: "starship_apply".to_string(),
            description: "Apply configuration changes to a Starship config file".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "config_path": {"type": "string"},
                    "patch": {"type": "string"},
                    "dry_run": {"type": "boolean"},
                    "backup_path": {"type": "string"}
                },
                "required": ["config_path", "patch"]
            }),
        },
    ];

    MCPResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(serde_json::json!({
            "tools": tools
        })),
        error: None,
    }
}

async fn handle_tools_call(params: ToolsCallParams, id: Value) -> MCPResponse {
    let result = match params.name.as_str() {
        "starship_options" => {
            match serde_json::from_value::<OptionsQuery>(params.arguments) {
                Ok(query) => match OptionsEndpoint::query(query).await {
                    Ok(result) => Ok(serde_json::to_value(result).unwrap_or(Value::Null)),
                    Err(e) => Err(MCPError {
                        code: -32603,
                        message: format!("Internal error: {}", e),
                        data: None,
                    }),
                },
                Err(e) => Err(MCPError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                    data: None,
                }),
            }
        }
        "starship_presets" => {
            match serde_json::from_value::<PresetsQuery>(params.arguments) {
                Ok(query) => match PresetsEndpoint::query(query).await {
                    Ok(result) => Ok(serde_json::to_value(result).unwrap_or(Value::Null)),
                    Err(e) => Err(MCPError {
                        code: -32603,
                        message: format!("Internal error: {}", e),
                        data: None,
                    }),
                },
                Err(e) => Err(MCPError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                    data: None,
                }),
            }
        }
        "starship_templates" => {
            match serde_json::from_value::<TemplatesQuery>(params.arguments) {
                Ok(query) => match TemplatesEndpoint::query(query).await {
                    Ok(result) => Ok(serde_json::to_value(result).unwrap_or(Value::Null)),
                    Err(e) => Err(MCPError {
                        code: -32603,
                        message: format!("Internal error: {}", e),
                        data: None,
                    }),
                },
                Err(e) => Err(MCPError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                    data: None,
                }),
            }
        }
        "starship_validate" => {
            match serde_json::from_value::<ValidateRequest>(params.arguments) {
                Ok(request) => match ValidateEndpoint::execute(request).await {
                    Ok(result) => Ok(serde_json::to_value(result).unwrap_or(Value::Null)),
                    Err(e) => Err(MCPError {
                        code: -32603,
                        message: format!("Internal error: {}", e),
                        data: None,
                    }),
                },
                Err(e) => Err(MCPError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                    data: None,
                }),
            }
        }
        "starship_apply" => {
            match serde_json::from_value::<ApplyRequest>(params.arguments) {
                Ok(request) => match ApplyEndpoint::execute(request).await {
                    Ok(result) => Ok(serde_json::to_value(result).unwrap_or(Value::Null)),
                    Err(e) => Err(MCPError {
                        code: -32603,
                        message: format!("Internal error: {}", e),
                        data: None,
                    }),
                },
                Err(e) => Err(MCPError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                    data: None,
                }),
            }
        }
        _ => Err(MCPError {
            code: -32601,
            message: format!("Unknown tool: {}", params.name),
            data: None,
        }),
    };

    match result {
        Ok(value) => MCPResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": serde_json::to_string_pretty(&value).unwrap_or_default()
                }]
            })),
            error: None,
        },
        Err(error) => MCPResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        },
    }
}

