use crate::core::model::{LazyVimPlugin, PluginDependency, PluginEvent};
use crate::core::ast::LuaAst;
use regex;
use std::collections::HashMap;
use std::path::Path;

/// LazyVim plugin structure and convention analyzer
pub struct LazyVimAnalyzer {
    ast: LuaAst,
}

impl LazyVimAnalyzer {
    pub fn new() -> Self {
        Self {
            ast: LuaAst::new(),
        }
    }

    /// Parse a LazyVim plugin file
    pub fn parse_plugin_file<P: AsRef<Path>>(&mut self, path: P) -> Result<LazyVimPlugin, String> {
        let (tree, source) = self.ast.parse_file(path)?;
        
        // Extract plugin information from AST
        // LazyVim plugins typically return a table with plugin spec
        let plugin = self.extract_plugin_spec(&tree, &source)?;
        
        Ok(plugin)
    }

    /// Extract plugin specification from AST
    fn extract_plugin_spec(&self, _tree: &tree_sitter::Tree, source: &str) -> Result<LazyVimPlugin, String> {
        // This is a simplified parser - full implementation would traverse the AST
        // and extract the return statement with the plugin table
        
        // For now, create a basic plugin structure
        // Full implementation would parse the actual Lua table structure
        let mut plugin = LazyVimPlugin {
            name: "unknown".to_string(),
            spec: HashMap::new(),
            dependencies: Vec::new(),
            events: Vec::new(),
            config: None,
            enabled: true,
        };

        // Try to extract plugin name from source
        if let Some(name_match) = regex::Regex::new(r#""([^"]+)/([^"]+)""#)
            .ok()
            .and_then(|re| re.captures(source))
        {
            if let (Some(user), Some(repo)) = (name_match.get(1), name_match.get(2)) {
                plugin.name = format!("{}/{}", user.as_str(), repo.as_str());
            }
        }

        // Extract event triggers
        if let Some(event_match) = regex::Regex::new(r#"event\s*=\s*"([^"]+)""#)
            .ok()
            .and_then(|re| re.captures(source))
        {
            if let Some(event) = event_match.get(1) {
                plugin.events.push(PluginEvent {
                    event: event.as_str().to_string(),
                    pattern: None,
                });
            }
        }

        // Extract dependencies
        if source.contains("dependencies") {
            // Simplified - would need proper AST traversal
            let deps_re = regex::Regex::new(r#""([^"]+)/([^"]+)""#).unwrap();
            for cap in deps_re.captures_iter(source) {
                if let (Some(user), Some(repo)) = (cap.get(1), cap.get(2)) {
                    plugin.dependencies.push(PluginDependency {
                        name: format!("{}/{}", user.as_str(), repo.as_str()),
                        version: None,
                        optional: false,
                    });
                }
            }
        }

        // Extract config block
        if let Some(config_match) = regex::Regex::new(r#"config\s*=\s*function\(\)\s*(.*?)\s*end"#)
            .ok()
            .and_then(|re| re.captures(source))
        {
            if let Some(config_code) = config_match.get(1) {
                plugin.config = Some(config_code.as_str().to_string());
            }
        }

        Ok(plugin)
    }

    /// Validate LazyVim plugin structure
    pub fn validate_plugin(&self, plugin: &LazyVimPlugin) -> Vec<String> {
        let mut errors = Vec::new();

        if plugin.name == "unknown" {
            errors.push("Plugin name is missing or invalid".to_string());
        }

        if plugin.events.is_empty() && !plugin.spec.contains_key("cmd") {
            errors.push("Plugin must have either an event trigger or cmd".to_string());
        }

        // Validate event names
        let valid_events = vec![
            "VeryLazy", "Lazy", "BufRead", "BufNewFile", "InsertEnter",
            "VimEnter", "FileType", "CmdlineEnter", "UIEnter",
        ];

        for event in &plugin.events {
            if !valid_events.contains(&event.event.as_str()) {
                errors.push(format!("Unknown event: {}", event.event));
            }
        }

        errors
    }

    /// Get LazyVim standard directory structure
    pub fn get_standard_directories() -> Vec<&'static str> {
        vec![
            "lua/plugins",
            "lua/config",
            "lua/keymaps",
            "lua/options",
            "after/plugin",
            "after/ftplugin",
        ]
    }

    /// Check if a path follows LazyVim conventions
    pub fn is_lazyvim_path<P: AsRef<Path>>(path: P) -> bool {
        let path_str = path.as_ref().to_string_lossy();
        Self::get_standard_directories()
            .iter()
            .any(|dir| path_str.contains(dir))
    }
}

impl Default for LazyVimAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

