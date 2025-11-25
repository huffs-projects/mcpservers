use anyhow::Result;
use serde_json::Value;
use std::fs;

pub struct WaybarParser;

impl WaybarParser {
    pub fn parse_json(path: &str) -> Result<Value> {
        let content = fs::read_to_string(path)?;
        let json: Value = serde_json::from_str(&content)?;
        Ok(json)
    }

    pub fn parse_css(path: &str) -> Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn extract_modules(config: &Value) -> Vec<String> {
        let mut modules = Vec::new();
        
        if let Some(modules_array) = config.get("modules-left")
            .or_else(|| config.get("modules-center"))
            .or_else(|| config.get("modules-right"))
        {
            if let Some(arr) = modules_array.as_array() {
                for module in arr {
                    if let Some(name) = module.as_str() {
                        modules.push(name.to_string());
                    }
                }
            }
        }

        // Also check for module definitions
        use crate::utils::TOP_LEVEL_KEYS;
        if let Some(module_defs) = config.as_object() {
            for (key, _) in module_defs {
                if !TOP_LEVEL_KEYS.contains(&key.as_str()) {
                    modules.push(key.clone());
                }
            }
        }

        modules
    }

    pub fn extract_custom_scripts(config: &Value) -> Vec<(String, String)> {
        let mut scripts = Vec::new();
        
        if let Some(obj) = config.as_object() {
            for (key, value) in obj {
                if let Some(module) = value.as_object() {
                    if let Some(exec) = module.get("exec") {
                        if let Some(cmd) = exec.as_str() {
                            scripts.push((key.clone(), cmd.to_string()));
                        }
                    }
                }
            }
        }

        scripts
    }
}

