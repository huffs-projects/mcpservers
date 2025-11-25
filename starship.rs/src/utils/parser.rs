use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct StarshipConfig {
    #[serde(flatten)]
    pub modules: HashMap<String, toml::Value>,
    
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
}

impl StarshipConfig {
    pub fn from_str(contents: &str) -> Result<Self> {
        toml::from_str(contents)
            .context("Failed to parse TOML configuration")
    }

    pub fn validate_structure(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Validate top-level format field
        if let Some(format_val) = self.modules.get("format") {
            if !format_val.is_str() {
                errors.push("'format' must be a string".to_string());
            }
        }

        // Validate module configurations
        let known_modules = vec![
            "format", "add_newline", "scan_timeout", "command_timeout", "palette",
            "git_branch", "git_status", "git_commit", "git_state", "git_metrics",
            "directory", "nodejs", "python", "rust", "golang", "java", "php",
            "scala", "ruby", "swift", "elixir", "character", "username", "hostname",
            "cmd_duration", "jobs", "battery", "time", "status", "container",
            "shell", "os", "package", "docker_context", "aws", "gcloud", "openstack",
            "nix_shell", "conda", "memory_usage", "env_var", "custom", "sudo",
            "cmake", "cobol", "daml", "deno", "dotnet", "elm", "erlang",
            "guix_shell", "haskell", "helm", "julia", "kotlin", "gradle", "lua",
            "nim", "ocaml", "opa", "perl", "pulumi", "purescript", "raku",
            "rlang", "red", "terraform", "vlang", "vagrant", "crystal",
            "localip", "shlvl", "line_break",
        ];

        // Check for unknown top-level keys (warnings, not errors)
        for (key, _) in &self.modules {
            if !key.starts_with('$') && !known_modules.contains(&key.as_str()) {
                // Check if it's a nested module configuration
                if !self.is_nested_module(key) {
                    // This might be a valid module name we don't know about
                    // or it could be an error - we'll just note it
                }
            }
        }

        // Validate nested module structures
        for (key, value) in &self.modules {
            if let Some(module_name) = key.strip_suffix(".disabled") {
                if let Some(_module_val) = self.modules.get(module_name) {
                    if value.as_bool().is_none() {
                        errors.push(format!("'{}.disabled' must be a boolean", key));
                    }
                }
            }
        }

        errors
    }

    fn is_nested_module(&self, key: &str) -> bool {
        // Check if this key is part of a nested module configuration
        // e.g., "git_branch.symbol" is nested under "git_branch"
        if let Some((module_name, _)) = key.split_once('.') {
            return self.modules.contains_key(module_name) || 
                   known_module_prefixes().iter().any(|prefix| module_name.starts_with(prefix));
        }
        false
    }

    pub fn get_module(&self, name: &str) -> Option<&toml::Value> {
        self.modules.get(name)
    }

    #[allow(dead_code)]
    pub fn get_nested_value(&self, path: &str) -> Option<&toml::Value> {
        // Get nested value like "git_branch.symbol"
        if let Some((module, field)) = path.split_once('.') {
            if let Some(module_val) = self.modules.get(module) {
                if let Some(table) = module_val.as_table() {
                    return table.get(field);
                }
            }
        }
        None
    }

    pub fn merge_patch(&mut self, patch: &str) -> Result<()> {
        let patch_config: toml::Value = toml::from_str(patch)
            .context("Failed to parse patch TOML")?;

        match patch_config {
            toml::Value::Table(table) => {
                for (key, value) in table {
                    self.merge_value(&key, value);
                }
            }
            _ => {
                return Err(anyhow::anyhow!("Patch must be a TOML table"));
            }
        }
        
        Ok(())
    }

    fn merge_value(&mut self, key: &str, value: toml::Value) {
        if let Some((module, field)) = key.split_once('.') {
            // Nested module configuration
            if let Some(existing) = self.modules.get_mut(module) {
                if let Some(table) = existing.as_table_mut() {
                    table.insert(field.to_string(), value);
                    return;
                }
            }
            // Module doesn't exist, create it
            let mut new_table = toml::value::Table::new();
            new_table.insert(field.to_string(), value);
            self.modules.insert(module.to_string(), toml::Value::Table(new_table));
        } else {
            // Top-level configuration
            self.modules.insert(key.to_string(), value);
        }
    }

    pub fn to_string(&self) -> Result<String> {
        toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML")
    }

    pub fn get_all_module_names(&self) -> Vec<String> {
        let mut modules = Vec::new();
        
        for key in self.modules.keys() {
            if let Some((module, _)) = key.split_once('.') {
                if !modules.contains(&module.to_string()) {
                    modules.push(module.to_string());
                }
            } else if !key.starts_with('$') {
                modules.push(key.clone());
            }
        }
        
        modules.sort();
        modules
    }
}

fn known_module_prefixes() -> Vec<&'static str> {
    vec![
        "git_", "nodejs", "python", "rust", "golang", "java", "php",
        "scala", "ruby", "swift", "elixir", "directory", "character",
        "username", "hostname", "cmd_", "jobs", "battery", "time",
        "status", "container", "shell", "os", "package", "docker_",
        "aws", "gcloud", "openstack", "nix_", "conda", "memory_",
        "env_", "custom", "sudo", "cmake", "cobol", "daml", "deno",
        "dotnet", "elm", "erlang", "guix_", "haskell", "helm", "julia",
        "kotlin", "gradle", "lua", "nim", "ocaml", "opa", "perl",
        "pulumi", "purescript", "raku", "rlang", "red", "terraform",
        "vlang", "vagrant", "crystal", "localip", "shlvl", "line_",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_toml() {
        let toml = r#"
format = "$all"
add_newline = true

[git_branch]
symbol = " "
style = "bold purple"
"#;
        assert!(StarshipConfig::from_str(toml).is_ok());
    }

    #[test]
    fn test_parse_invalid_toml() {
        let toml = "invalid = [unclosed";
        assert!(StarshipConfig::from_str(toml).is_err());
    }

    #[test]
    fn test_merge_patch() {
        let mut config = StarshipConfig::from_str("format = \"$all\"").unwrap();
        let patch = "[git_branch]\nsymbol = \" \"";
        assert!(config.merge_patch(patch).is_ok());
        assert!(config.get_module("git_branch").is_some());
    }

    #[test]
    fn test_get_nested_value() {
        let config = StarshipConfig::from_str(
            "[git_branch]\nsymbol = \" \"\nstyle = \"bold purple\""
        ).unwrap();
        assert!(config.get_nested_value("git_branch.symbol").is_some());
    }

    #[test]
    fn test_validate_structure() {
        let config = StarshipConfig::from_str("format = \"$all\"").unwrap();
        let errors = config.validate_structure();
        assert!(errors.is_empty());
    }
}
