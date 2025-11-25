use crate::core::model::NvimOption;
use std::collections::HashMap;
use std::path::PathBuf;

/// Neovim runtime documentation and option resolver
pub struct NeovimRuntime {
    options: HashMap<String, NvimOption>,
    runtime_paths: Vec<PathBuf>,
}

impl NeovimRuntime {
    pub fn new() -> Self {
        let mut runtime = Self {
            options: HashMap::new(),
            runtime_paths: Vec::new(),
        };
        
        // Initialize with common Neovim options
        runtime.initialize_options();
        runtime.discover_runtime_paths();
        
        runtime
    }

    /// Discover Neovim runtime paths
    fn discover_runtime_paths(&mut self) {
        // Standard Neovim runtime paths
        if let Some(home) = dirs::home_dir() {
            let config_nvim = home.join(".config/nvim");
            if config_nvim.exists() {
                self.runtime_paths.push(config_nvim);
            }
        }

        // XDG config directory
        if let Some(config_dir) = dirs::config_dir() {
            let xdg_nvim = config_dir.join("nvim");
            if xdg_nvim.exists() {
                self.runtime_paths.push(xdg_nvim);
            }
        }

        // System runtime (usually /usr/share/nvim/runtime on Linux)
        let system_runtimes = vec![
            PathBuf::from("/usr/share/nvim/runtime"),
            PathBuf::from("/usr/local/share/nvim/runtime"),
        ];

        for path in system_runtimes {
            if path.exists() {
                self.runtime_paths.push(path);
            }
        }
    }

    /// Initialize built-in Neovim options
    fn initialize_options(&mut self) {
        // Common vim.opt options
        let common_options = vec![
            ("tabstop", "number", "global", "Number of spaces that a <Tab> in the file counts for"),
            ("shiftwidth", "number", "global", "Number of spaces to use for each step of (auto)indent"),
            ("expandtab", "boolean", "global", "In Insert mode: Use the appropriate number of spaces to insert a <Tab>"),
            ("number", "boolean", "window", "Print the line number in front of each line"),
            ("relativenumber", "boolean", "window", "Show the line number relative to the line with the cursor"),
            ("wrap", "boolean", "window", "Long lines wrap"),
            ("linebreak", "boolean", "window", "Wrap long lines at a character in 'breakat'"),
            ("cursorline", "boolean", "window", "Highlight the screen line of the cursor"),
            ("colorcolumn", "string", "window", "Comma-separated list of screen columns that are highlighted"),
            ("scrolloff", "number", "global", "Minimal number of screen lines to keep above and below the cursor"),
            ("sidescrolloff", "number", "global", "Minimal number of screen columns to keep to the left and right of the cursor"),
            ("mouse", "string", "global", "Enable mouse support"),
            ("clipboard", "string", "global", "Use the clipboard as the unnamed register"),
            ("undofile", "boolean", "global", "Automatically save and restore undo history"),
            ("backup", "boolean", "global", "Make a backup before overwriting a file"),
            ("writebackup", "boolean", "global", "Make a backup before overwriting a file"),
            ("swapfile", "boolean", "global", "Use a swapfile for the buffer"),
            ("ignorecase", "boolean", "global", "Ignore case in search patterns"),
            ("smartcase", "boolean", "global", "Override 'ignorecase' if the search pattern contains upper case characters"),
            ("incsearch", "boolean", "global", "While typing a search command, show where the pattern matches"),
            ("hlsearch", "boolean", "global", "When there is a previous search pattern, highlight all its matches"),
        ];

        for (name, opt_type, scope, doc) in common_options {
            let option = NvimOption {
                name: name.to_string(),
                scope: scope.to_string(),
                option_type: opt_type.to_string(),
                default: None,
                current: None,
                documentation: doc.to_string(),
                help_tag: format!("'{}'", name),
                documentation_url: format!("https://neovim.io/doc/user/options.html#'{}'", name),
                valid_values: None,
                since_api: None,
                deprecated: false,
            };
            self.options.insert(name.to_string(), option);
        }
    }

    /// Get an option by name
    pub fn get_option(&self, name: &str) -> Option<&NvimOption> {
        self.options.get(name)
    }

    /// Search options by name or description
    pub fn search_options(&self, query: &str) -> Vec<&NvimOption> {
        let query_lower = query.to_lowercase();
        self.options
            .values()
            .filter(|opt| {
                opt.name.to_lowercase().contains(&query_lower)
                    || opt.documentation.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get all options, optionally filtered by scope
    pub fn get_all_options(&self, scope_filter: Option<&str>) -> Vec<&NvimOption> {
        self.options
            .values()
            .filter(|opt| {
                scope_filter
                    .map(|s| opt.scope == s)
                    .unwrap_or(true)
            })
            .collect()
    }

    /// Get runtime paths
    pub fn get_runtime_paths(&self) -> &[PathBuf] {
        &self.runtime_paths
    }

    /// Validate that a runtime path exists
    pub fn validate_runtime_path(&self, path: &PathBuf) -> bool {
        self.runtime_paths.iter().any(|p| p == path) || path.exists()
    }
}

impl Default for NeovimRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_option() {
        let runtime = NeovimRuntime::new();
        let option = runtime.get_option("tabstop");
        assert!(option.is_some(), "Should find tabstop option");
        assert_eq!(option.unwrap().name, "tabstop");
    }

    #[test]
    fn test_search_options() {
        let runtime = NeovimRuntime::new();
        let results = runtime.search_options("tab");
        assert!(!results.is_empty(), "Should find options matching 'tab'");
    }

    #[test]
    fn test_get_all_options() {
        let runtime = NeovimRuntime::new();
        let all_options = runtime.get_all_options(None);
        assert!(!all_options.is_empty(), "Should have some options");
    }

    #[test]
    fn test_get_options_by_scope() {
        let runtime = NeovimRuntime::new();
        let window_options = runtime.get_all_options(Some("window"));
        assert!(!window_options.is_empty(), "Should have window-scoped options");
    }
}

