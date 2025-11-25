use super::protocol::Tool;
use serde_json::{json, Value};

/// Create a tool definition with name, description, and input schema
pub fn create_tool(name: &str, description: &str, input_schema: Value) -> Tool {
    Tool {
        name: name.to_string(),
        description: description.to_string(),
        input_schema,
    }
}

/// Get all available tools with their schemas
pub fn get_all_tools() -> Vec<Tool> {
    vec![
        create_tool(
            "mako_options",
            "List Mako configuration options with types, defaults, and valid values from source code.",
            json!({
                "type": "object",
                "properties": {
                    "search_term": {
                        "type": "string",
                        "description": "Optional search term to filter options"
                    }
                }
            }),
        ),
        create_tool(
            "mako_templates",
            "Generate Mako config snippets for common use cases.",
            json!({
                "type": "object",
                "properties": {
                    "use_case": {
                        "type": "string",
                        "description": "Optional use case name (e.g. 'minimal', 'persistent', 'colored', 'positional')"
                    }
                }
            }),
        ),
        create_tool(
            "mako_validate",
            "Validate the `mako/config` file for syntax and semantic correctness.",
            json!({
                "type": "object",
                "properties": {
                    "config_path": {
                        "type": "string",
                        "description": "Path to the Mako config file"
                    }
                },
                "required": ["config_path"]
            }),
        ),
        create_tool(
            "mako_apply",
            "Apply patch to Mako configuration safely, with dry-run and backup.",
            json!({
                "type": "object",
                "properties": {
                    "config_path": {
                        "type": "string",
                        "description": "Path to the Mako config file"
                    },
                    "patch": {
                        "type": "string",
                        "description": "INI-format patch to apply"
                    },
                    "dry_run": {
                        "type": "boolean",
                        "description": "If true, preview changes without applying (default: true)"
                    },
                    "backup_path": {
                        "type": "string",
                        "description": "Optional path for backup file"
                    }
                },
                "required": ["config_path", "patch"]
            }),
        ),
    ]
}

