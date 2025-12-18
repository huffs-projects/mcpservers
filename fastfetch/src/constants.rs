/// Constants used throughout the fastfetch MCP server.
/// 
/// This module contains all constant values used across the codebase,
/// including URLs, file names, and command-line arguments.

/// The official fastfetch JSON schema URL.
/// 
/// This URL points to the JSON schema file in the fastfetch repository
/// that defines the structure of valid configuration files.
pub const FASTFETCH_SCHEMA_URL: &str = "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json";

/// Default config file name.
pub const CONFIG_FILE_NAME: &str = "config.jsonc";

/// Default config directory name (relative to user config directory).
pub const CONFIG_DIR_NAME: &str = "fastfetch";

/// Fastfetch binary name.
pub const FASTFETCH_BINARY: &str = "fastfetch";

/// Fastfetch command-line arguments.
/// 
/// This module contains constants for all fastfetch CLI flags used by the server.
pub mod fastfetch_args {
    /// Generate minimal config
    pub const GEN_CONFIG: &str = "--gen-config";
    
    /// Generate full config
    pub const GEN_CONFIG_FULL: &str = "--gen-config-full";
    
    /// List available modules
    pub const LIST_MODULES: &str = "--list-modules";
    
    /// List available logos
    pub const LIST_LOGOS: &str = "--list-logos";
}

/// Timeout for fastfetch command execution (30 seconds)
pub const FASTFETCH_COMMAND_TIMEOUT_SECS: u64 = 30;
