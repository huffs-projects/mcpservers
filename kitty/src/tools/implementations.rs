use crate::tools::traits::Tool;
use crate::endpoints::*;
use crate::utils::extract_args_mod as extract_args;
use serde_json::{json, Value};

pub struct KittyOptionsTool;

#[async_trait::async_trait]
impl Tool for KittyOptionsTool {
    fn name(&self) -> &str {
        "kitty_options"
    }
    
    fn description(&self) -> &str {
        "Query all known Kitty options (fonts, window behavior, layouts, mouse, performance, graphics)"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "search_term": {
                    "type": "string",
                    "description": "Search term to filter options"
                },
                "category": {
                    "type": "string",
                    "description": "Filter by category (Fonts, Window, Performance, Layouts, etc.)"
                }
            }
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, String> {
        let query = crate::endpoints::kitty_options::OptionsQuery {
            search_term: extract_args::extract_string(&arguments, "search_term"),
            category: extract_args::extract_string(&arguments, "category"),
        };
        
        let result = handle_kitty_options(query).await;
        serde_json::to_value(result)
            .map_err(|e| format!("Failed to serialize result: {}", e))
    }
}

pub struct KittyThemingTool;

#[async_trait::async_trait]
impl Tool for KittyThemingTool {
    fn name(&self) -> &str {
        "kitty_theming"
    }
    
    fn description(&self) -> &str {
        "Return themes, full color palettes, and template snippets"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "theme_name": {
                    "type": "string",
                    "description": "Filter by specific theme name"
                }
            }
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, String> {
        let query = crate::endpoints::kitty_theming::ThemingQuery {
            theme_name: extract_args::extract_string(&arguments, "theme_name"),
        };
        
        let result = handle_kitty_theming(query).await;
        serde_json::to_value(result)
            .map_err(|e| format!("Failed to serialize result: {}", e))
    }
}

pub struct KittyKeybindingsTool;

#[async_trait::async_trait]
impl Tool for KittyKeybindingsTool {
    fn name(&self) -> &str {
        "kitty_keybindings"
    }
    
    fn description(&self) -> &str {
        "Query keybinding actions (e.g., resize_window, new_tab, goto_layout, kitten)"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Filter by specific action name"
                }
            }
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, String> {
        let query = crate::endpoints::kitty_keybindings::KeybindingsQuery {
            action: extract_args::extract_string(&arguments, "action"),
        };
        
        let result = handle_kitty_keybindings(query).await;
        serde_json::to_value(result)
            .map_err(|e| format!("Failed to serialize result: {}", e))
    }
}

pub struct KittyTemplatesTool;

#[async_trait::async_trait]
impl Tool for KittyTemplatesTool {
    fn name(&self) -> &str {
        "kitty_templates"
    }
    
    fn description(&self) -> &str {
        "Generate templates for sections like fonts, performance tuning, layout management, kittens, keybindings, and window defaults"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "category": {
                    "type": "string",
                    "description": "Filter by template category"
                },
                "use_case": {
                    "type": "string",
                    "description": "Filter by use case description"
                }
            }
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, String> {
        let query = crate::endpoints::kitty_templates::TemplatesQuery {
            category: extract_args::extract_string(&arguments, "category"),
            use_case: extract_args::extract_string(&arguments, "use_case"),
        };
        
        let result = handle_kitty_templates(query).await;
        serde_json::to_value(result)
            .map_err(|e| format!("Failed to serialize result: {}", e))
    }
}

pub struct KittyValidateTool;

#[async_trait::async_trait]
impl Tool for KittyValidateTool {
    fn name(&self) -> &str {
        "kitty_validate"
    }
    
    fn description(&self) -> &str {
        "Validate kitty.conf using Kitty's official syntax rules, including lexical errors, unknown options, invalid values or units, broken bindings, and missing required parameters"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "config_path": {
                    "type": "string",
                    "description": "Path to kitty.conf file to validate"
                }
            },
            "required": ["config_path"]
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, String> {
        let config_path = extract_args::extract_string(&arguments, "config_path")
            .ok_or_else(|| "config_path is required".to_string())?;
        
        let req = crate::endpoints::kitty_validate::ValidateRequest {
            config_path,
        };
        
        let result = handle_kitty_validate(req).await;
        serde_json::to_value(result)
            .map_err(|e| format!("Failed to serialize result: {}", e))
    }
}

pub struct KittyApplyTool;

#[async_trait::async_trait]
impl Tool for KittyApplyTool {
    fn name(&self) -> &str {
        "kitty_apply"
    }
    
    fn description(&self) -> &str {
        "Safely apply patches to kitty.conf with atomic writes and automatic backups. Includes unified diff output for agent reasoning."
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "config_path": {
                    "type": "string",
                    "description": "Path to kitty.conf file"
                },
                "patch": {
                    "type": "string",
                    "description": "Configuration patch to apply"
                },
                "dry_run": {
                    "type": "boolean",
                    "description": "If true, only show diff without applying changes",
                    "default": true
                },
                "backup_path": {
                    "type": "string",
                    "description": "Optional path for backup file"
                }
            },
            "required": ["config_path", "patch"]
        })
    }
    
    async fn execute(&self, arguments: Value) -> Result<Value, String> {
        let config_path = extract_args::extract_string(&arguments, "config_path")
            .ok_or_else(|| "config_path is required".to_string())?;
        let patch = extract_args::extract_string(&arguments, "patch")
            .ok_or_else(|| "patch is required".to_string())?;
        let dry_run = extract_args::extract_bool(&arguments, "dry_run").unwrap_or(true);
        let backup_path = extract_args::extract_string(&arguments, "backup_path");
        
        let req = crate::endpoints::kitty_apply::ApplyRequest {
            config_path,
            patch,
            dry_run,
            backup_path,
        };
        
        let result = handle_kitty_apply(req).await;
        serde_json::to_value(result)
            .map_err(|e| format!("Failed to serialize result: {}", e))
    }
}

