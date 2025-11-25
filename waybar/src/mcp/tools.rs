use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Tool definition for MCP protocol
///
/// Each tool has a name, description, and JSON schema for its input parameters.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,  // Always present, serialized as "inputSchema"
}

/// Tool registry for managing available Waybar tools
///
/// Maintains a list of all available tools that can be executed by the MCP server.
pub struct ToolRegistry {
    tools: Vec<Tool>,
}

impl ToolRegistry {
    /// Create a new tool registry with all Waybar tools
    ///
    /// Initializes the registry with all 6 Waybar management tools:
    /// - waybar_modules
    /// - waybar_scripts
    /// - waybar_style
    /// - waybar_templates
    /// - waybar_validate
    /// - waybar_apply
    pub fn new() -> Self {
        Self {
            tools: Self::get_all_tools(),
        }
    }

    /// Get all registered tools
    ///
    /// # Returns
    /// A slice of all registered tools
    pub fn get_tools(&self) -> &[Tool] {
        &self.tools
    }

    /// Find a tool by name
    ///
    /// # Arguments
    /// * `name` - The name of the tool to find
    ///
    /// # Returns
    /// Some(Tool) if found, None otherwise
    pub fn find_tool(&self, name: &str) -> Option<&Tool> {
        self.tools.iter().find(|t| t.name == name)
    }

    /// Get all tool definitions
    fn get_all_tools() -> Vec<Tool> {
        vec![
            Tool {
                name: "waybar_modules".to_string(),
                description: "List built-in Waybar modules and all configuration options".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "filter_module": {
                            "type": "string",
                            "description": "Optional module name to filter by"
                        }
                    }
                }),
            },
            Tool {
                name: "waybar_scripts".to_string(),
                description: "Inspect custom script blocks ('custom' and 'exec' modules)".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "config_path": {
                            "type": "string",
                            "description": "Path to Waybar config file"
                        },
                        "filter_name": {
                            "type": "string",
                            "description": "Optional script name to filter by"
                        }
                    }
                }),
            },
            Tool {
                name: "waybar_style".to_string(),
                description: "Return CSS style rules for bars, modules, blocks, and fonts".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "selector": {
                            "type": "string",
                            "description": "Optional CSS selector to filter by"
                        }
                    }
                }),
            },
            Tool {
                name: "waybar_templates".to_string(),
                description: "Generate Waybar JSON + CSS templates for common use-cases".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "use_case": {
                            "type": "string",
                            "description": "Use case name (e.g., 'hyprland-default', 'battery', 'network', 'cpu')"
                        }
                    }
                }),
            },
            Tool {
                name: "waybar_validate".to_string(),
                description: "Validate Waybar JSON + CSS files: syntax, required keys, style correctness, script validity".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "required": ["config_path"],
                    "properties": {
                        "config_path": {
                            "type": "string",
                            "description": "Path to Waybar JSON config file"
                        },
                        "css_path": {
                            "type": "string",
                            "description": "Optional path to CSS file"
                        }
                    }
                }),
            },
            Tool {
                name: "waybar_apply".to_string(),
                description: "Apply patches to JSON and CSS safely: backup, diff, dry-run".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "required": ["config_path", "patch_json"],
                    "properties": {
                        "config_path": {
                            "type": "string",
                            "description": "Path to Waybar JSON config file"
                        },
                        "css_path": {
                            "type": "string",
                            "description": "Optional path to CSS file"
                        },
                        "patch_json": {
                            "type": "string",
                            "description": "JSON patch to apply"
                        },
                        "patch_css": {
                            "type": "string",
                            "description": "Optional CSS patch to apply"
                        },
                        "dry_run": {
                            "type": "boolean",
                            "description": "If true, show diff without applying",
                            "default": true
                        },
                        "backup_path": {
                            "type": "string",
                            "description": "Optional directory for backups"
                        }
                    }
                }),
            },
        ]
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

