mod config;
mod constants;
mod error;
mod modules;
mod prompts;
mod resources;
mod schema;
mod tools;

use crate::error::McpServerError;
use rmcp::{
    model::*, ServerHandler, ServiceExt,
};
use rmcp::service::RequestContext;
use rmcp::transport::stdio;
use std::sync::Arc;

/// Fastfetch MCP Server implementation
/// 
/// This implementation follows the MCP protocol fixes described in FIXES.md:
/// - Uses stdio transport (not HTTP) for JSON-RPC 2.0 communication
/// - Ensures inputSchema is always present (not Option) for all tools
/// - The rmcp library handles JSON-RPC protocol compliance including:
///   - Proper id field handling (never null)
///   - Field name serialization (camelCase conversion for inputSchema)
pub struct FastfetchServer;

/// Helper function to convert JSON schema Value to Arc<Map>
/// The input_schema field expects Arc<Map<String, Value>> (not Option)
/// This ensures inputSchema is always present in the serialized output, as required by MCP protocol.
fn schema_to_map(schema: serde_json::Value) -> Arc<serde_json::Map<String, serde_json::Value>> {
    // If it's an object, use it directly. Otherwise create an empty map.
    if let Some(obj) = schema.as_object() {
        Arc::new(obj.clone())
    } else {
        // Fallback to empty map if schema is not an object
        Arc::new(serde_json::Map::new())
    }
}

impl ServerHandler for FastfetchServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
            server_info: Implementation {
                name: "fastfetch-mcp-server".to_string(),
                version: "0.1.0".to_string(),
                icons: None,
                title: None,
                website_url: None,
            },
            instructions: Some(
                "A Model Context Protocol server for configuring fastfetch. \
                 Provides tools to read, write, validate, and generate fastfetch configuration files."
                    .to_string(),
            ),
        }
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let name = request.name.to_string();
        // Arguments from CallToolRequestParam are Map<String, Value>, convert to Value
        let arguments = request.arguments
            .map(|m| serde_json::Value::Object(m))
            .unwrap_or_else(|| serde_json::json!({}));
        
        let result = match name.as_str() {
            "read_fastfetch_config" => tools::read_fastfetch_config(arguments).await,
            "write_fastfetch_config" => tools::write_fastfetch_config(arguments).await,
            "validate_fastfetch_config" => tools::validate_fastfetch_config(arguments).await,
            "list_fastfetch_modules" => tools::list_fastfetch_modules(arguments).await,
            "list_fastfetch_logos" => tools::list_fastfetch_logos(arguments).await,
            "generate_fastfetch_config" => tools::generate_fastfetch_config(arguments).await,
            "fastfetch_format_help" => tools::fastfetch_format_help(arguments).await,
            _ => Err(McpServerError::UnknownTool { tool_name: name }),
        };

        match result {
            Ok(value) => {
                // Convert the Value to a string for text content
                let text = serde_json::to_string(&value)
                    .unwrap_or_else(|_| format!("{{\"error\": \"Failed to serialize result\"}}"));
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(e) => {
                // Provide detailed error information using the Display implementation
                let error_msg = format!("Error: {}", e);
                Err(rmcp::ErrorData::internal_error(error_msg, None))
            }
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        let tools = vec![
            Tool {
                name: "read_fastfetch_config".into(),
                title: None,
                description: Some("Read and parse a fastfetch configuration file (JSONC format)".into()),
                input_schema: schema_to_map(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to config file (optional, defaults to ~/.config/fastfetch/config.jsonc)"
                        }
                    }
                })),
                annotations: None,
                icons: None,
                output_schema: None,
            },
            Tool {
                name: "write_fastfetch_config".into(),
                title: None,
                description: Some("Write a fastfetch configuration to file".into()),
                input_schema: schema_to_map(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "config": {
                            "type": "object",
                            "description": "The fastfetch configuration object to write"
                        },
                        "path": {
                            "type": "string",
                            "description": "Path to config file (optional, defaults to ~/.config/fastfetch/config.jsonc)"
                        }
                    },
                    "required": ["config"]
                })),
                annotations: None,
                icons: None,
                output_schema: None,
            },
            Tool {
                name: "validate_fastfetch_config".into(),
                title: None,
                description: Some("Validate a fastfetch configuration against the JSON schema".into()),
                input_schema: schema_to_map(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "config": {
                            "type": "object",
                            "description": "The config object to validate (optional, if not provided will read from file)"
                        },
                        "path": {
                            "type": "string",
                            "description": "Path to config file (optional, used if config not provided)"
                        }
                    }
                })),
                annotations: None,
                icons: None,
                output_schema: None,
            },
            Tool {
                name: "list_fastfetch_modules".into(),
                title: None,
                description: Some("List all available fastfetch modules".into()),
                input_schema: schema_to_map(serde_json::json!({
                    "type": "object",
                    "properties": {}
                })),
                annotations: None,
                icons: None,
                output_schema: None,
            },
            Tool {
                name: "list_fastfetch_logos".into(),
                title: None,
                description: Some("List all available fastfetch logos".into()),
                input_schema: schema_to_map(serde_json::json!({
                    "type": "object",
                    "properties": {}
                })),
                annotations: None,
                icons: None,
                output_schema: None,
            },
            Tool {
                name: "generate_fastfetch_config".into(),
                title: None,
                description: Some("Generate a new fastfetch configuration file (minimal or full)".into()),
                input_schema: schema_to_map(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "full": {
                            "type": "boolean",
                            "description": "Generate full config with all defaults (default: false)"
                        },
                        "path": {
                            "type": "string",
                            "description": "Path to write config file (optional)"
                        }
                    }
                })),
                annotations: None,
                icons: None,
                output_schema: None,
            },
            Tool {
                name: "fastfetch_format_help".into(),
                title: None,
                description: Some("Get help with fastfetch format strings and color specifications".into()),
                input_schema: schema_to_map(serde_json::json!({
                    "type": "object",
                    "properties": {}
                })),
                annotations: None,
                icons: None,
                output_schema: None,
            },
        ];
        
        Ok(ListToolsResult::with_all_items(tools))
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::ErrorData> {
        let resources: Vec<Resource> = resources::list_resources()
            .into_iter()
            .map(|r| {
                Resource::new(
                    RawResource {
                        uri: r.uri,
                        name: r.name,
                        title: None,
                        description: r.description,
                        mime_type: r.mime_type,
                        size: None,
                        icons: None,
                    },
                    None,
                )
            })
            .collect();
        
        Ok(ListResourcesResult::with_all_items(resources))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ReadResourceResult, rmcp::ErrorData> {
        match resources::read_resource(&request.uri).await {
            Ok(result) => {
                let contents: Vec<ResourceContents> = result.contents
                    .into_iter()
                    .map(|c| ResourceContents::text(c.text, request.uri.clone()))
                    .collect();
                Ok(ReadResourceResult { contents })
            }
            Err(e) => {
                Err(rmcp::ErrorData::internal_error(
                    format!("Resource read error: {}", e),
                    None,
                ))
            }
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListPromptsResult, rmcp::ErrorData> {
        let prompts: Vec<Prompt> = prompts::list_prompts()
            .into_iter()
            .map(|p| {
                let arguments = p.arguments.map(|args| {
                    args.into_iter()
                        .map(|a| PromptArgument {
                            name: a.name,
                            title: None,
                            description: Some(a.description),
                            required: a.required,
                        })
                        .collect()
                });
                
                Prompt {
                    name: p.name,
                    description: Some(p.description),
                    arguments,
                    title: None,
                    icons: None,
                }
            })
            .collect();
        
        Ok(ListPromptsResult::with_all_items(prompts))
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, rmcp::ErrorData> {
        let arguments = request.arguments
            .map(|m| serde_json::Value::Object(m))
            .or_else(|| Some(serde_json::json!({})));
        
        match prompts::get_prompt(&request.name, arguments).await {
            Ok(result) => {
                let messages: Vec<PromptMessage> = result.messages
                    .into_iter()
                    .map(|m| {
                        let content = match m.content {
                            prompts::PromptMessageContent::Text(text) => {
                                PromptMessageContent::text(text)
                            }
                            prompts::PromptMessageContent::Parts(parts) => {
                                // For parts, we'll combine them into a single text content
                                let combined_text = parts
                                    .into_iter()
                                    .map(|p| p.text)
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                PromptMessageContent::text(combined_text)
                            }
                        };
                        
                        PromptMessage {
                            role: match m.role.as_str() {
                                "user" => PromptMessageRole::User,
                                "assistant" => PromptMessageRole::Assistant,
                                _ => PromptMessageRole::User,
                            },
                            content,
                        }
                    })
                    .collect();
                
                Ok(GetPromptResult {
                    description: result.description,
                    messages,
                })
            }
            Err(e) => {
                Err(rmcp::ErrorData::internal_error(
                    format!("Prompt get error: {}", e),
                    None,
                ))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = FastfetchServer;
    let transport = stdio();
    let service = server.serve(transport).await?;
    
    // Wait for the service to finish
    service.waiting().await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_to_map_with_object() {
        let schema = json!({
            "type": "object",
            "properties": {
                "key": {
                    "type": "string"
                }
            }
        });
        
        let map = schema_to_map(schema);
        assert_eq!(map.len(), 2);
        assert!(map.contains_key("type"));
        assert!(map.contains_key("properties"));
    }

    #[test]
    fn test_schema_to_map_with_non_object() {
        // Test with array (should return empty map)
        let schema = json!([1, 2, 3]);
        let map = schema_to_map(schema);
        assert_eq!(map.len(), 0);
        
        // Test with string (should return empty map)
        let schema = json!("string");
        let map = schema_to_map(schema);
        assert_eq!(map.len(), 0);
        
        // Test with number (should return empty map)
        let schema = json!(42);
        let map = schema_to_map(schema);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_schema_to_map_with_empty_object() {
        let schema = json!({});
        let map = schema_to_map(schema);
        assert_eq!(map.len(), 0);
    }
}
