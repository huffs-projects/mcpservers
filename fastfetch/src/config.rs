use crate::constants::{CONFIG_DIR_NAME, CONFIG_FILE_NAME};
use crate::error::ConfigError;
use dirs;
use jsonc_parser::{parse_to_serde_value, ParseOptions};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Configuration file handling for fastfetch MCP server.
/// 
/// This module provides functions to read, write, and parse fastfetch configuration files
/// in JSONC format (JSON with comments).

/// Get the default fastfetch config path.
/// 
/// Returns the standard location for the fastfetch configuration file:
/// `~/.config/fastfetch/config.jsonc` (or platform-equivalent).
/// 
/// # Returns
/// 
/// * `Ok(PathBuf)` - The default config file path
/// * `Err` - If the config directory cannot be determined
/// 
/// # Example
/// 
/// ```no_run
/// use fastfetch_mcp_server::config::default_config_path;
/// 
/// let path = default_config_path()?;
/// println!("Config path: {}", path.display());
/// ```
pub fn default_config_path() -> Result<PathBuf, ConfigError> {
    let config_dir = dirs::config_dir()
        .ok_or(ConfigError::ConfigDirNotFound)?;
    Ok(config_dir.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
}

/// Resolve the config path, using default if None.
/// 
/// If a path is provided, it is returned as-is. Otherwise, the default
/// config path is used.
/// 
/// # Parameters
/// 
/// * `path` - Optional custom path to config file
/// 
/// # Returns
/// 
/// * `Ok(PathBuf)` - The resolved config file path
/// * `Err` - If the default path cannot be determined
/// 
/// # Example
/// 
/// ```no_run
/// use fastfetch_mcp_server::config::resolve_config_path;
/// use std::path::PathBuf;
/// 
/// // Use custom path
/// let path = resolve_config_path(Some("/custom/path.jsonc".into()))?;
/// 
/// // Use default path
/// let default_path = resolve_config_path(None)?;
/// ```
pub fn resolve_config_path(path: Option<PathBuf>) -> Result<PathBuf, ConfigError> {
    match path {
        Some(p) => Ok(p),
        None => default_config_path(),
    }
}

/// Parse JSONC content (JSON with comments).
/// 
/// Parses a string containing JSONC (JSON with C-style comments) into a
/// `serde_json::Value` object.
/// 
/// # Parameters
/// 
/// * `content` - The JSONC string to parse
/// * `path` - Optional path for error reporting (defaults to "config.jsonc" if None)
/// 
/// # Returns
/// 
/// * `Ok(Value)` - The parsed JSON value
/// * `Err` - If parsing fails
/// 
/// # Example
/// 
/// ```
/// use fastfetch_mcp_server::config::parse_jsonc;
/// 
/// let jsonc = r#"
/// {
///   // This is a comment
///   "key": "value"
/// }
/// "#;
/// 
/// let value = parse_jsonc(jsonc, None)?;
/// ```
pub fn parse_jsonc(content: &str, path: Option<PathBuf>) -> Result<Value, ConfigError> {
    let parse_options = ParseOptions::default();
    let path = path.unwrap_or_else(|| PathBuf::from("config.jsonc"));
    parse_to_serde_value(content, &parse_options)
        .map_err(|e| ConfigError::ParseError {
            path: path.clone(),
            message: format!("Failed to parse JSONC: {}", e),
        })?
        .ok_or_else(|| ConfigError::ParseError {
            path,
            message: "JSONC parsing returned None".to_string(),
        })
}

/// Read fastfetch config from file.
/// 
/// Reads and parses a fastfetch configuration file. If no path is provided,
/// the default config path is used.
/// 
/// # Parameters
/// 
/// * `path` - Optional path to config file. If `None`, uses default location.
/// 
/// # Returns
/// 
/// * `Ok(Value)` - The parsed configuration as a JSON value
/// * `Err` - If the file doesn't exist, cannot be read, or parsing fails
/// 
/// # Example
/// 
/// ```no_run
/// use fastfetch_mcp_server::config::read_config;
/// 
/// // Read from default location
/// let config = read_config(None)?;
/// 
/// // Read from custom location
/// let config = read_config(Some("/path/to/config.jsonc".into()))?;
/// ```
pub fn read_config(path: Option<PathBuf>) -> Result<Value, ConfigError> {
    let config_path = resolve_config_path(path)?;
    
    if !config_path.exists() {
        return Err(ConfigError::NotFound {
            path: config_path,
        });
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|source| ConfigError::ReadError {
            path: config_path.clone(),
            source,
        })?;

    parse_jsonc(&content, Some(config_path.clone()))
}

/// Write fastfetch config to file.
/// 
/// Writes a configuration object to a file. The parent directory is created
/// if it doesn't exist. The config is serialized as pretty-printed JSON.
/// 
/// # Parameters
/// 
/// * `config` - The configuration object to write
/// * `path` - Optional path to config file. If `None`, uses default location.
/// 
/// # Returns
/// 
/// * `Ok(())` - Success
/// * `Err` - If the directory cannot be created, serialization fails, or writing fails
/// 
/// # Example
/// 
/// ```no_run
/// use fastfetch_mcp_server::config::write_config;
/// use serde_json::json;
/// 
/// let config = json!({
///     "logo": "arch",
///     "modules": []
/// });
/// 
/// write_config(&config, None)?;
/// ```
pub fn write_config(config: &Value, path: Option<PathBuf>) -> Result<(), ConfigError> {
    let config_path = resolve_config_path(path)?;
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|source| ConfigError::DirectoryCreationError {
                path: parent.to_path_buf(),
                source,
            })?;
    }

    // Serialize with pretty printing
    let content = serde_json::to_string_pretty(config)
        .map_err(|source| ConfigError::SerializeError { source })?;

    fs::write(&config_path, content)
        .map_err(|source| ConfigError::WriteError {
            path: config_path,
            source,
        })?;

    Ok(())
}

/// Check if config file exists.
/// 
/// # Parameters
/// 
/// * `path` - Optional path to config file. If `None`, checks default location.
/// 
/// # Returns
/// 
/// * `Ok(true)` - File exists
/// * `Ok(false)` - File does not exist
/// * `Err` - If the path cannot be resolved
/// 
/// # Example
/// 
/// ```no_run
/// use fastfetch_mcp_server::config::config_exists;
/// use std::path::PathBuf;
/// 
/// // Check default location
/// let exists = config_exists(None)?;
/// 
/// // Check custom location
/// let exists = config_exists(Some("/custom/path.jsonc".into()))?;
/// ```
pub fn config_exists(path: Option<PathBuf>) -> Result<bool, ConfigError> {
    let config_path = resolve_config_path(path)?;
    Ok(config_path.exists())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_jsonc() {
        // Test parsing JSONC with comments
        let jsonc = r#"
        {
            // This is a comment
            "key": "value",
            "number": 42
        }
        "#;
        
        let result = parse_jsonc(jsonc, None).unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
    }

    #[test]
    fn test_parse_jsonc_invalid() {
        // Test parsing invalid JSONC
        let invalid = "{ invalid json }";
        assert!(parse_jsonc(invalid, None).is_err());
    }

    #[test]
    fn test_resolve_config_path() {
        // Test with custom path
        let custom_path = PathBuf::from("/custom/path.jsonc");
        let resolved = resolve_config_path(Some(custom_path.clone())).unwrap();
        assert_eq!(resolved, custom_path);
        
        // Test with None (should use default)
        let default = resolve_config_path(None);
        assert!(default.is_ok());
    }

    #[test]
    fn test_read_write_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.jsonc");
        
        let config = json!({
            "logo": "arch",
            "modules": ["os", "cpu"]
        });
        
        // Write config
        write_config(&config, Some(config_path.clone())).unwrap();
        
        // Read config back
        let read_back = read_config(Some(config_path.clone())).unwrap();
        assert_eq!(read_back["logo"], "arch");
        assert_eq!(read_back["modules"][0], "os");
    }

    #[test]
    fn test_config_exists() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.jsonc");
        
        // Should not exist initially
        assert!(!config_exists(Some(config_path.clone())).unwrap());
        
        // Create file
        fs::write(&config_path, "{}").unwrap();
        
        // Should exist now
        assert!(config_exists(Some(config_path)).unwrap());
    }

    #[test]
    fn test_parse_jsonc_with_multiline_comments() {
        let jsonc = r#"
        {
            // Single line comment
            "key": "value",
            /* Multi-line
               comment */
            "number": 42
        }
        "#;
        
        let result = parse_jsonc(jsonc, None).unwrap();
        assert_eq!(result["key"], "value");
        assert_eq!(result["number"], 42);
    }

    #[test]
    fn test_parse_jsonc_with_trailing_comma() {
        // JSONC allows trailing commas
        let jsonc = r#"
        {
            "key1": "value1",
            "key2": "value2",
        }
        "#;
        
        let result = parse_jsonc(jsonc, None).unwrap();
        assert_eq!(result["key1"], "value1");
        assert_eq!(result["key2"], "value2");
    }

    #[test]
    fn test_read_write_config_with_comments() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.jsonc");
        
        // Write config with comments
        let jsonc_content = r#"{
            // This is a comment
            "logo": "arch",
            "modules": ["os", "cpu"]
        }"#;
        
        fs::write(&config_path, jsonc_content).unwrap();
        
        // Read it back
        let read_back = read_config(Some(config_path.clone())).unwrap();
        assert_eq!(read_back["logo"], "arch");
        assert_eq!(read_back["modules"][0], "os");
    }

    #[test]
    fn test_write_config_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("subdir").join("config.jsonc");
        
        let config = json!({
            "logo": "test"
        });
        
        // Should create the subdirectory
        write_config(&config, Some(config_path.clone())).unwrap();
        
        // Verify file exists
        assert!(config_path.exists());
        
        // Verify content
        let read_back = read_config(Some(config_path)).unwrap();
        assert_eq!(read_back["logo"], "test");
    }

    #[test]
    fn test_default_config_path() {
        let path = default_config_path().unwrap();
        assert!(path.to_string_lossy().contains("fastfetch"));
        assert!(path.to_string_lossy().contains("config.jsonc"));
    }

    #[test]
    fn test_resolve_config_path_custom() {
        let custom = PathBuf::from("/custom/path.jsonc");
        let resolved = resolve_config_path(Some(custom.clone())).unwrap();
        assert_eq!(resolved, custom);
    }

    #[test]
    fn test_read_config_not_found() {
        let nonexistent = PathBuf::from("/nonexistent/path/config.jsonc");
        let result = read_config(Some(nonexistent));
        assert!(result.is_err());
        if let Err(ConfigError::NotFound { .. }) = result {
            // Expected error type
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[test]
    fn test_parse_jsonc_malformed() {
        // Test various malformed JSONC inputs
        let malformed_inputs = vec![
            "{ invalid }",
            "{ \"key\": }",
            "{ \"key\": \"value\" } extra",
            "not json at all",
        ];
        
        for input in malformed_inputs {
            let result = parse_jsonc(input, None);
            assert!(result.is_err(), "Should fail to parse: {}", input);
        }
    }

    #[test]
    fn test_write_config_invalid_path() {
        // Test writing to a path that cannot be created (e.g., root on Unix)
        // This is platform-specific, so we'll test with a very long path instead
        let config = json!({"logo": "test"});
        
        // On Unix, paths longer than PATH_MAX will fail
        // This is a best-effort test
        let very_long_path = PathBuf::from("/".repeat(1000));
        let result = write_config(&config, Some(very_long_path));
        // May succeed or fail depending on platform, but shouldn't panic
        let _ = result;
    }
}
