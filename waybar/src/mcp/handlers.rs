use crate::endpoints::*;
use crate::mcp::protocol::Response;
use anyhow::Result;
use serde_json::Value;

/// Handle MCP initialize request
///
/// Returns server capabilities and information according to MCP protocol.
///
/// # Arguments
/// * `response_id` - The request ID from the initialize request
///
/// # Returns
/// A Response containing protocol version, capabilities, and server info
pub fn handle_initialize(response_id: Value) -> Response {
    Response::success(
        response_id,
        serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "waybar-rust-mcp",
                "version": "1.0.0"
            }
        }),
    )
}

/// Handle tools/list request
///
/// Returns the list of all available tools with their schemas.
///
/// # Arguments
/// * `response_id` - The request ID
/// * `tool_registry` - The tool registry containing all available tools
///
/// # Returns
/// A Response containing the list of tools
pub fn handle_tools_list(response_id: Value, tool_registry: &crate::mcp::tools::ToolRegistry) -> Response {
    Response::success(
        response_id,
        serde_json::json!({
            "tools": tool_registry.get_tools()
        }),
    )
}

/// Handle tools/call request
///
/// Executes a tool by name with the provided arguments.
///
/// # Arguments
/// * `response_id` - The request ID
/// * `name` - The name of the tool to execute
/// * `arguments` - The tool arguments as a JSON value
///
/// # Returns
/// A Response containing the tool execution result or an error
pub async fn handle_tools_call(
    response_id: Value,
    name: &str,
    arguments: &Value,
) -> Result<Response> {
    let result = execute_tool(name, arguments).await?;

    Ok(Response::success(
        response_id,
        serde_json::json!({
            "content": [
                {
                    "type": "text",
                    "text": serde_json::to_string_pretty(&result)?
                }
            ]
        }),
    ))
}

/// Execute a tool by name with given arguments
async fn execute_tool(name: &str, arguments: &Value) -> Result<Value> {
    match name {
        "waybar_modules" => {
            let filter = arguments
                .get("filter_module")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let modules = query_modules(filter);
            Ok(serde_json::to_value(modules)?)
        }
        "waybar_scripts" => {
            let config_path = arguments
                .get("config_path")
                .and_then(|v| v.as_str());
            let filter = arguments
                .get("filter_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let scripts = waybar_scripts::query_scripts(config_path, filter)?;
            Ok(serde_json::to_value(scripts)?)
        }
        "waybar_style" => {
            let selector = arguments
                .get("selector")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let styles = query_styles(selector);
            Ok(serde_json::to_value(styles)?)
        }
        "waybar_templates" => {
            let use_case = arguments
                .get("use_case")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let templates = query_templates(use_case);
            Ok(serde_json::to_value(templates)?)
        }
        "waybar_validate" => {
            let config_path = arguments
                .get("config_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing config_path"))?;
            let css_path = arguments
                .get("css_path")
                .and_then(|v| v.as_str());
            let result = waybar_validate::validate_config(config_path, css_path)?;
            Ok(serde_json::to_value(result)?)
        }
        "waybar_apply" => {
            let config_path = arguments
                .get("config_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing config_path"))?;
            let css_path = arguments
                .get("css_path")
                .and_then(|v| v.as_str());
            let patch_json = arguments
                .get("patch_json")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing patch_json"))?;
            let patch_css = arguments
                .get("patch_css")
                .and_then(|v| v.as_str());
            let dry_run = arguments
                .get("dry_run")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let backup_path = arguments
                .get("backup_path")
                .and_then(|v| v.as_str());
            let result = waybar_apply::apply_patches(
                config_path,
                css_path,
                patch_json,
                patch_css,
                dry_run,
                backup_path,
            )?;
            Ok(serde_json::to_value(result)?)
        }
        _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
    }
}

