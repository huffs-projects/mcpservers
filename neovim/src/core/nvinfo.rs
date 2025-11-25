use serde_json::Value;
use std::process::Command;

/// Integration with Neovim's api_info() for runtime option discovery
pub struct NvimInfo {
    api_info: Option<Value>,
}

impl NvimInfo {
    pub fn new() -> Self {
        Self { api_info: None }
    }

    /// Query Neovim for API information
    /// This requires a running Neovim instance with --headless
    pub fn query_api_info(&mut self) -> Result<Value, String> {
        let output = Command::new("nvim")
            .args(&["--headless", "--cmd", "lua print(vim.json.encode(vim.api.nvim_get_api_info()))", "--cmd", "qa"])
            .output()
            .map_err(|e| format!("Failed to execute nvim: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Neovim command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let api_info: Value = serde_json::from_str(&stdout)
            .map_err(|e| format!("Failed to parse API info: {}", e))?;

        self.api_info = Some(api_info.clone());
        Ok(api_info)
    }

    /// Extract option information from API info
    pub fn extract_options(&self) -> Vec<String> {
        let mut options = Vec::new();
        
        if let Some(ref api_info) = self.api_info {
            // Navigate through API info structure
            if let Some(functions) = api_info.get("functions").and_then(|f| f.as_array()) {
                for func in functions {
                    if let Some(name) = func.get("name").and_then(|n| n.as_str()) {
                        // Look for nvim_get_option and nvim_set_option related functions
                        if name.contains("option") {
                            options.push(name.to_string());
                        }
                    }
                }
            }
        }
        
        options
    }

    /// Get cached API info
    pub fn get_api_info(&self) -> Option<&Value> {
        self.api_info.as_ref()
    }
}

impl Default for NvimInfo {
    fn default() -> Self {
        Self::new()
    }
}

