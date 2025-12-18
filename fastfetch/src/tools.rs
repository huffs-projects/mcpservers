use crate::config::{read_config, write_config, default_config_path};
use crate::constants::{fastfetch_args, FASTFETCH_BINARY, FASTFETCH_COMMAND_TIMEOUT_SECS};
use crate::error::{FastfetchError, McpResult, McpServerError};
use crate::modules::{list_logos, list_modules};
use crate::schema::validate_config_summary;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// MCP tool implementations for fastfetch configuration management.
/// 
/// This module contains the implementation of all MCP tools exposed by the server.

/// Extract an optional string parameter from tool arguments.
/// 
/// # Parameters
/// 
/// * `args` - The arguments Value object
/// * `key` - The parameter key to extract
/// 
/// # Returns
/// 
/// * `Option<String>` - The string value if present and valid, None otherwise
fn get_optional_string(args: &Value, key: &str) -> Option<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract an optional boolean parameter from tool arguments.
/// 
/// # Parameters
/// 
/// * `args` - The arguments Value object
/// * `key` - The parameter key to extract
/// * `default` - The default value if the parameter is not present or invalid
/// 
/// # Returns
/// 
/// * `bool` - The boolean value or the default
fn get_optional_bool(args: &Value, key: &str, default: bool) -> bool {
    args.get(key)
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

/// Read fastfetch config tool.
/// 
/// Reads and parses a fastfetch configuration file.
/// 
/// # Parameters (via args)
/// 
/// * `path` (optional) - Path to config file. Defaults to `~/.config/fastfetch/config.jsonc`
/// 
/// # Returns
/// 
/// JSON object with:
/// * `config` - The parsed configuration object
/// * `path` - The path where the config was read from
pub async fn read_fastfetch_config(args: Value) -> McpResult<Value> {
    let path: Option<String> = get_optional_string(&args, "path");

    let config_path = path.map(PathBuf::from);
    let config = read_config(config_path.clone())
        .map_err(McpServerError::from)?;

    Ok(json!({
        "config": config,
        "path": config_path
            .or_else(|| default_config_path().ok())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }))
}

/// Write fastfetch config tool.
/// 
/// Writes a fastfetch configuration object to a file.
/// 
/// # Parameters (via args)
/// 
/// * `config` (required) - The configuration object to write
/// * `path` (optional) - Path to config file. Defaults to `~/.config/fastfetch/config.jsonc`
/// 
/// # Returns
/// 
/// JSON object with:
/// * `success` - Boolean indicating success
/// * `path` - The path where the config was written
pub async fn write_fastfetch_config(args: Value) -> McpResult<Value> {
    let config = args.get("config")
        .ok_or_else(|| McpServerError::MissingParameter {
            param: "config".to_string(),
        })?;

    let path: Option<String> = get_optional_string(&args, "path");

    let config_path = path.map(PathBuf::from);
    let path_for_result = config_path.clone();
    write_config(config, config_path)
        .map_err(McpServerError::from)?;

    Ok(json!({
        "success": true,
        "path": path_for_result
            .or_else(|| default_config_path().ok())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }))
}

/// Validate fastfetch config tool.
/// 
/// Validates a fastfetch configuration against the JSON schema.
/// 
/// # Parameters (via args)
/// 
/// * `config` (optional) - The configuration object to validate. If not provided, reads from file.
/// * `path` (optional) - Path to config file (used if config not provided)
/// 
/// # Returns
/// 
/// JSON object with:
/// * `valid` - Boolean indicating if config is valid
/// * `summary` - Human-readable validation summary
pub async fn validate_fastfetch_config(args: Value) -> McpResult<Value> {
    let config = if let Some(c) = args.get("config") {
        c.clone()
    } else {
        // If no config provided, read from file
        let path: Option<String> = get_optional_string(&args, "path");

        let config_path = path.map(PathBuf::from);
        read_config(config_path.clone())
            .map_err(McpServerError::from)?
    };

    let summary = validate_config_summary(&config).await
        .map_err(McpServerError::from)?;
    let is_valid = summary.contains("valid");

    Ok(json!({
        "valid": is_valid,
        "summary": summary
    }))
}

/// List fastfetch modules tool.
/// 
/// Lists all available fastfetch modules.
/// 
/// # Returns
/// 
/// JSON object with:
/// * `modules` - Array of module names
/// * `count` - Number of modules
pub async fn list_fastfetch_modules(_args: Value) -> McpResult<Value> {
    let modules = list_modules().await?;

    Ok(json!({
        "modules": modules,
        "count": modules.len()
    }))
}

/// List fastfetch logos tool.
/// 
/// Lists all available fastfetch logos.
/// 
/// # Returns
/// 
/// JSON object with:
/// * `logos` - Array of logo names
/// * `count` - Number of logos
pub async fn list_fastfetch_logos(_args: Value) -> McpResult<Value> {
    let logos = list_logos().await?;

    Ok(json!({
        "logos": logos,
        "count": logos.len()
    }))
}

/// Generate fastfetch config tool.
/// 
/// Generates a new fastfetch configuration file using the fastfetch CLI.
/// 
/// # Parameters (via args)
/// 
/// * `full` (optional) - Generate full config with all defaults (default: false)
/// * `path` (optional) - Path to write config file. If provided, copies generated config to this location.
/// 
/// # Returns
/// 
/// JSON object with:
/// * `success` - Boolean indicating success
/// * `path` - The path where the config was written
/// * `full` - Whether full config was generated
pub async fn generate_fastfetch_config(args: Value) -> McpResult<Value> {
    let full = args.get("full")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let path: Option<String> = get_optional_string(&args, "path");

    let target_path = path.map(PathBuf::from);

    // Execute fastfetch to generate config to default location
    let args = if full {
        vec![fastfetch_args::GEN_CONFIG_FULL]
    } else {
        vec![fastfetch_args::GEN_CONFIG]
    };

    let timeout_duration = Duration::from_secs(FASTFETCH_COMMAND_TIMEOUT_SECS);
    
    let output_result = timeout(
        timeout_duration,
        Command::new(FASTFETCH_BINARY)
            .args(&args)
            .kill_on_drop(true)
            .output()
    ).await;
    
    let output = match output_result {
        Ok(Ok(output)) => output,
        Ok(Err(source)) => {
            // Check if the error is "command not found"
            return Err(if source.kind() == std::io::ErrorKind::NotFound {
                McpServerError::Fastfetch(FastfetchError::CommandNotFound)
            } else {
                McpServerError::Fastfetch(FastfetchError::ExecutionError { source })
            });
        }
        Err(_) => {
            return Err(McpServerError::Fastfetch(FastfetchError::CommandFailed {
                stderr: format!("Command timed out after {} seconds", FASTFETCH_COMMAND_TIMEOUT_SECS),
            }));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(McpServerError::Fastfetch(FastfetchError::CommandFailed { stderr }));
    }

    // Determine the final path
    let final_path = if let Some(ref target) = target_path {
        // If a path was specified, read the generated config and write it to the target
        let default_path = default_config_path()
            .map_err(McpServerError::from)?;
        
        // Read the config that fastfetch generated
        let generated_config = read_config(Some(default_path.clone()))
            .map_err(McpServerError::from)?;
        
        // Write it to the target path
        write_config(&generated_config, Some(target.clone()))
            .map_err(McpServerError::from)?;
        
        target.clone()
    } else {
        // Use the default location where fastfetch wrote it
        default_config_path()
            .map_err(McpServerError::from)?
    };

    Ok(json!({
        "success": true,
        "path": final_path.to_string_lossy().to_string(),
        "full": full
    }))
}

/// Fastfetch format string help tool.
/// 
/// Returns help text explaining fastfetch format strings and color specifications.
/// 
/// # Returns
/// 
/// JSON object with:
/// * `help` - Help text string
pub async fn fastfetch_format_help(_args: Value) -> McpResult<Value> {
    let help_text = r#"
Fastfetch Format String Guide:

Format strings are used to customize the output of fastfetch modules. They support:

1. **Color Format Specification:**
   - `@{<color>}` - Set foreground color
   - `@{<color>:<bgcolor>}` - Set foreground and background colors
   - Colors can be: black, red, green, yellow, blue, magenta, cyan, white
   - Or use hex codes: `@{#RRGGBB}` or `@{#RRGGBB:#RRGGBB}`

2. **Common Format Codes:**
   - `@b` - Bold
   - `@u` - Underline
   - `@r` - Reset formatting
   - `@n` - Newline
   - `@t` - Tab

3. **Module-Specific Variables:**
   Each module has its own variables. For example:
   - OS module: `@name`, `@version`, `@codename`
   - CPU module: `@name`, `@cores`, `@frequency`
   - Memory module: `@used`, `@total`, `@percent`

4. **Examples:**
   - `@{cyan}OS: @{white}@name` - Cyan "OS: " followed by white OS name
   - `@{green}CPU: @{yellow}@name @{@cores} cores` - Green "CPU: ", yellow CPU name, cores count
   - `@{red}Memory: @{@used} / @{@total} (@{@percent}%)` - Memory usage display

For detailed information, see:
- Format String Guide: https://github.com/fastfetch-cli/fastfetch/wiki/Format-String-Guide
- Color Format Specification: https://github.com/fastfetch-cli/fastfetch/wiki/Color-Format-Specification
"#;

    Ok(json!({
        "help": help_text.trim()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_read_fastfetch_config_with_path() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.jsonc");
        
        let config = json!({
            "logo": "arch",
            "modules": ["os", "cpu"]
        });
        
        // Write config first
        write_config(&config, Some(config_path.clone())).unwrap();
        
        // Read it back
        let args = json!({
            "path": config_path.to_string_lossy().to_string()
        });
        
        let result = read_fastfetch_config(args).await.unwrap();
        assert_eq!(result["config"]["logo"], "arch");
        assert_eq!(result["config"]["modules"][0], "os");
        assert!(result["path"].as_str().unwrap().contains("test_config.jsonc"));
    }

    #[tokio::test]
    async fn test_read_fastfetch_config_missing_file() {
        let args = json!({
            "path": "/nonexistent/path/config.jsonc"
        });
        
        let result = read_fastfetch_config(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_write_fastfetch_config_with_path() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.jsonc");
        
        let config = json!({
            "logo": "ubuntu",
            "modules": ["os"]
        });
        
        let args = json!({
            "config": config,
            "path": config_path.to_string_lossy().to_string()
        });
        
        let result = write_fastfetch_config(args).await.unwrap();
        assert_eq!(result["success"], true);
        assert!(result["path"].as_str().unwrap().contains("test_config.jsonc"));
        
        // Verify it was written
        let read_back = read_config(Some(config_path)).unwrap();
        assert_eq!(read_back["logo"], "ubuntu");
    }

    #[tokio::test]
    async fn test_write_fastfetch_config_missing_config() {
        let args = json!({});
        
        let result = write_fastfetch_config(args).await;
        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                McpServerError::MissingParameter { param } => {
                    assert_eq!(param, "config");
                }
                _ => panic!("Expected MissingParameter error"),
            }
        }
    }

    #[tokio::test]
    async fn test_validate_fastfetch_config_with_config() {
        let config = json!({
            "logo": "arch",
            "modules": []
        });
        
        let args = json!({
            "config": config
        });
        
        let result = validate_fastfetch_config(args).await.unwrap();
        assert!(result.get("valid").is_some());
        assert!(result.get("summary").is_some());
    }

    #[tokio::test]
    async fn test_validate_fastfetch_config_with_path() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.jsonc");
        
        let config = json!({
            "logo": "arch",
            "modules": []
        });
        
        write_config(&config, Some(config_path.clone())).unwrap();
        
        let args = json!({
            "path": config_path.to_string_lossy().to_string()
        });
        
        let result = validate_fastfetch_config(args).await.unwrap();
        assert!(result.get("valid").is_some());
        assert!(result.get("summary").is_some());
    }

    #[tokio::test]
    async fn test_list_fastfetch_modules() {
        let args = json!({});
        let result = list_fastfetch_modules(args).await.unwrap();
        
        assert!(result.get("modules").is_some());
        assert!(result.get("count").is_some());
        assert!(result["count"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_list_fastfetch_logos() {
        let args = json!({});
        let result = list_fastfetch_logos(args).await.unwrap();
        
        assert!(result.get("logos").is_some());
        assert!(result.get("count").is_some());
        assert!(result["count"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_fastfetch_format_help() {
        let args = json!({});
        let result = fastfetch_format_help(args).await.unwrap();
        
        assert!(result.get("help").is_some());
        let help_text = result["help"].as_str().unwrap();
        assert!(help_text.contains("Fastfetch Format String Guide"));
        assert!(help_text.contains("Color Format Specification"));
        assert!(help_text.contains("@b"));
        assert!(help_text.contains("@name"));
    }

    #[tokio::test]
    async fn test_write_fastfetch_config_invalid_path() {
        // Test with an invalid path (parent directory doesn't exist and can't be created)
        // This is hard to test portably, so we'll test with a valid but unusual path
        let config = json!({
            "logo": "test"
        });
        
        let args = json!({
            "config": config,
            "path": "/tmp/fastfetch-test-write/config.jsonc"
        });
        
        // This should succeed (creates directory)
        let result = write_fastfetch_config(args).await;
        // Clean up if it succeeded
        if let Ok(r) = &result {
            let path_str = r["path"].as_str().unwrap();
            let _ = std::fs::remove_file(path_str);
            let _ = std::fs::remove_dir("/tmp/fastfetch-test-write");
        }
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_fastfetch_config_invalid_jsonc() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid_config.jsonc");
        
        // Write invalid JSONC
        fs::write(&config_path, "{ invalid json }").unwrap();
        
        let args = json!({
            "path": config_path.to_string_lossy().to_string()
        });
        
        let result = read_fastfetch_config(args).await;
        assert!(result.is_err(), "Should fail to read invalid JSONC");
    }

    #[tokio::test]
    async fn test_validate_fastfetch_config_invalid_jsonc() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid_config.jsonc");
        
        // Write invalid JSONC
        fs::write(&config_path, "{ invalid json }").unwrap();
        
        let args = json!({
            "path": config_path.to_string_lossy().to_string()
        });
        
        let result = validate_fastfetch_config(args).await;
        assert!(result.is_err(), "Should fail to validate invalid JSONC");
    }

    #[test]
    fn test_get_optional_string() {
        let args = json!({
            "path": "/test/path",
            "other": 123
        });
        
        assert_eq!(get_optional_string(&args, "path"), Some("/test/path".to_string()));
        assert_eq!(get_optional_string(&args, "nonexistent"), None);
        assert_eq!(get_optional_string(&args, "other"), None); // Not a string
    }

    #[test]
    fn test_get_optional_bool() {
        let args = json!({
            "full": true,
            "other": "not bool"
        });
        
        assert_eq!(get_optional_bool(&args, "full", false), true);
        assert_eq!(get_optional_bool(&args, "nonexistent", false), false);
        assert_eq!(get_optional_bool(&args, "other", false), false); // Not a bool
    }
}
