use std::path::Path;

/// Discover endpoint handler
#[derive(Clone)]
pub struct DiscoverEndpoint;

impl DiscoverEndpoint {
    pub fn new() -> Self {
        Self
    }

    /// Handle discover query
    pub async fn handle_query(&self) -> Result<Vec<String>, String> {
        let mut config_roots = Vec::new();

        // Check XDG config directory
        if let Some(config_dir) = dirs::config_dir() {
            let nvim_config = config_dir.join("nvim");
            if nvim_config.exists() {
                config_roots.push(nvim_config.to_string_lossy().to_string());
            }
        }

        // Check ~/.config/nvim
        if let Some(home) = dirs::home_dir() {
            let nvim_config = home.join(".config/nvim");
            if nvim_config.exists() {
                let path_str = nvim_config.to_string_lossy().to_string();
                if !config_roots.contains(&path_str) {
                    config_roots.push(path_str);
                }
            }
        }

        // Discover config structure
        let mut discovered_paths = Vec::new();
        for root in &config_roots {
            let root_path = Path::new(root);
            
            // Check for init.lua
            if root_path.join("init.lua").exists() {
                discovered_paths.push(root_path.join("init.lua").to_string_lossy().to_string());
            }

            // Check for lua/ directory
            let lua_dir = root_path.join("lua");
            if lua_dir.exists() {
                discovered_paths.push(lua_dir.to_string_lossy().to_string());
                
                // Check for lua/plugins (LazyVim convention)
                let plugins_dir = lua_dir.join("plugins");
                if plugins_dir.exists() {
                    discovered_paths.push(plugins_dir.to_string_lossy().to_string());
                }
            }

            // Check for plugin/ directory
            let plugin_dir = root_path.join("plugin");
            if plugin_dir.exists() {
                discovered_paths.push(plugin_dir.to_string_lossy().to_string());
            }

            // Check for after/ directory
            let after_dir = root_path.join("after");
            if after_dir.exists() {
                discovered_paths.push(after_dir.to_string_lossy().to_string());
            }
        }

        if discovered_paths.is_empty() {
            discovered_paths = config_roots;
        }

        Ok(discovered_paths)
    }
}

impl Default for DiscoverEndpoint {
    fn default() -> Self {
        Self::new()
    }
}

