use crate::core::model::LazyVimPlugin;
use std::collections::{HashMap, HashSet};

/// Plugin registry for tracking installed and available plugins
pub struct PluginRegistry {
    plugins: HashMap<String, LazyVimPlugin>,
    dependencies: HashMap<String, Vec<String>>, // plugin -> dependencies
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Register a plugin
    pub fn register(&mut self, plugin: LazyVimPlugin) {
        let deps: Vec<String> = plugin.dependencies
            .iter()
            .map(|d| d.name.clone())
            .collect();
        
        self.dependencies.insert(plugin.name.clone(), deps);
        self.plugins.insert(plugin.name.clone(), plugin);
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&LazyVimPlugin> {
        self.plugins.get(name)
    }

    /// Check if a plugin is registered
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    /// Get all registered plugins
    pub fn get_all_plugins(&self) -> Vec<&LazyVimPlugin> {
        self.plugins.values().collect()
    }

    /// Get dependencies for a plugin
    pub fn get_dependencies(&self, plugin_name: &str) -> Vec<String> {
        self.dependencies
            .get(plugin_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all dependencies recursively
    pub fn get_all_dependencies(&self, plugin_name: &str) -> HashSet<String> {
        let mut deps = HashSet::new();
        self.collect_dependencies(plugin_name, &mut deps);
        deps
    }

    fn collect_dependencies(&self, plugin_name: &str, deps: &mut HashSet<String>) {
        if let Some(direct_deps) = self.dependencies.get(plugin_name) {
            for dep in direct_deps {
                if deps.insert(dep.clone()) {
                    self.collect_dependencies(dep, deps);
                }
            }
        }
    }

    /// Find missing dependencies
    pub fn find_missing_dependencies(&self, plugin_name: &str) -> Vec<String> {
        let all_deps = self.get_all_dependencies(plugin_name);
        all_deps
            .iter()
            .filter(|dep| !self.has_plugin(dep))
            .cloned()
            .collect()
    }

    /// Validate plugin event triggers
    pub fn validate_event_trigger(&self, event: &str) -> bool {
        let valid_events = vec![
            "VeryLazy", "Lazy", "BufRead", "BufNewFile", "BufReadPre",
            "BufReadPost", "BufWrite", "BufWritePre", "BufWritePost",
            "InsertEnter", "InsertLeave", "VimEnter", "FileType",
            "CmdlineEnter", "CmdlineLeave", "UIEnter", "FocusGained",
            "FocusLost", "WinEnter", "WinLeave",
        ];
        valid_events.contains(&event)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

