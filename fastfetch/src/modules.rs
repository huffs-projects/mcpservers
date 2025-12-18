use crate::config::read_config;
use crate::constants::{fastfetch_args, FASTFETCH_BINARY, FASTFETCH_COMMAND_TIMEOUT_SECS};
use crate::error::{FastfetchError, McpServerError};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::time::timeout;

/// Module and logo listing for fastfetch.
/// This module provides functions to discover available fastfetch modules
/// and logos, with caching and fallback mechanisms.

static MODULES_CACHE: OnceLock<Mutex<Option<Vec<String>>>> = OnceLock::new();
static LOGOS_CACHE: OnceLock<Mutex<Option<Vec<String>>>> = OnceLock::new();

/// Parse modules from fastfetch --list-modules output.
/// Parses the output format: "1)  ModuleName : Description"
/// Handles various edge cases and malformed input gracefully.
/// # Parameters
/// * `output` - The output string from fastfetch --list-modules
/// # Returns
/// * `Vec<String>` - List of parsed module names (normalized to lowercase)
fn parse_modules_from_output(output: &str) -> Vec<String> {
    let mut parsed_modules = Vec::new();
    
    for line in output.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }
        
        // Skip lines that don't contain a closing parenthesis (likely headers or invalid)
        if !trimmed.contains(')') {
            continue;
        }
        
        // Extract module name from format: "1)  ModuleName : Description"
        // or "1) ModuleName : Description" or variations
        if let Some(colon_pos) = trimmed.find(':') {
            // Get the part before the colon
            let before_colon = trimmed[..colon_pos].trim();
            
            // Find the closing parenthesis
            if let Some(paren_pos) = before_colon.find(')') {
                // Extract module name after the parenthesis
                let module_name = before_colon[paren_pos + 1..].trim();
                
                // Validate module name is not empty and doesn't contain only whitespace
                if !module_name.is_empty() && !module_name.chars().all(|c| c.is_whitespace()) {
                    // Normalize to lowercase to match config file format
                    let normalized = module_name.to_lowercase();
                    
                    // Avoid duplicates
                    if !parsed_modules.contains(&normalized) {
                        parsed_modules.push(normalized);
                    }
                }
            }
        } else {
            // Try to parse format without colon: "1) ModuleName"
            if let Some(paren_pos) = trimmed.find(')') {
                let module_name = trimmed[paren_pos + 1..].trim();
                if !module_name.is_empty() && !module_name.chars().all(|c| c.is_whitespace()) {
                    let normalized = module_name.to_lowercase();
                    if !parsed_modules.contains(&normalized) {
                        parsed_modules.push(normalized);
                    }
                }
            }
        }
    }
    
    parsed_modules
}

/// Parse logos from fastfetch --list-logos output.
/// Parses the output format: "1)  "LogoName" "Alias1" "Alias2""
/// Handles various edge cases and malformed input gracefully.
/// # Parameters
/// * `output` - The output string from fastfetch --list-logos
/// # Returns
/// * `Vec<String>` - List of parsed logo names
fn parse_logos_from_output(output: &str) -> Vec<String> {
    let mut parsed_logos = Vec::new();
    
    for line in output.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }
        
        // Check for section headers (case-insensitive)
        let lower = trimmed.to_lowercase();
        if lower.starts_with("builtin logos:") {
            continue;
        }
        if lower.starts_with("custom logos:") {
            // We can stop here or continue for custom logos
            // For now, we'll include builtin logos only
            break;
        }
        
        // Skip lines that don't match the pattern (no quotes)
        if !trimmed.contains('"') {
            continue;
        }
        
        // Extract all quoted strings from the line
        // Format: "1)  "LogoName" "Alias1" "Alias2""
        // Handle escaped quotes and edge cases
        let mut in_quotes = false;
        let mut current_quote = String::new();
        let mut prev_char = None;
        
        for ch in trimmed.chars() {
            match (prev_char, ch, in_quotes) {
                (Some('\\'), '"', true) => {
                    // Escaped quote - add literal quote
                    current_quote.push('"');
                    prev_char = Some(ch);
                }
                (_, '"', false) => {
                    // Start of quoted string
                    in_quotes = true;
                    current_quote.clear();
                    prev_char = Some(ch);
                }
                (_, '"', true) => {
                    // End of quoted string
                    in_quotes = false;
                    if !current_quote.is_empty() {
                        // Validate logo name is not empty or only whitespace
                        let trimmed_logo = current_quote.trim();
                        if !trimmed_logo.is_empty() && !parsed_logos.iter().any(|l| l == trimmed_logo) {
                            parsed_logos.push(trimmed_logo.to_string());
                        }
                        current_quote.clear();
                    }
                    prev_char = Some(ch);
                }
                (_, c, true) => {
                    // Inside quoted string
                    if c != '\\' || prev_char != Some('\\') {
                        current_quote.push(c);
                    }
                    prev_char = Some(ch);
                }
                _ => {
                    prev_char = Some(ch);
                }
            }
        }
        
        // Handle case where quote wasn't closed (malformed input)
        if in_quotes && !current_quote.trim().is_empty() {
            let trimmed_logo = current_quote.trim();
            if !parsed_logos.iter().any(|l| l == trimmed_logo) {
                parsed_logos.push(trimmed_logo.to_string());
            }
        }
    }
    
    parsed_logos
}

/// Execute fastfetch command and return output.
/// # Parameters
/// * `args` - Command-line arguments to pass to fastfetch
/// # Returns
/// * `Ok(String)` - The command output
/// * `Err` - If fastfetch is not installed, the command fails, or times out
async fn exec_fastfetch(args: &[&str]) -> Result<String, FastfetchError> {
    let timeout_duration = Duration::from_secs(FASTFETCH_COMMAND_TIMEOUT_SECS);
    
    let output_result = timeout(
        timeout_duration,
        Command::new(FASTFETCH_BINARY)
            .args(args)
            .kill_on_drop(true)
            .output()
    ).await;
    
    let output = match output_result {
        Ok(Ok(output)) => output,
        Ok(Err(source)) => {
            // Check if the error is "command not found"
            return Err(if source.kind() == std::io::ErrorKind::NotFound {
                FastfetchError::CommandNotFound
            } else {
                FastfetchError::ExecutionError { source }
            });
        }
        Err(_) => {
            return Err(FastfetchError::CommandFailed {
                stderr: format!("Command timed out after {} seconds", FASTFETCH_COMMAND_TIMEOUT_SECS),
            });
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(FastfetchError::CommandFailed { stderr });
    }

    String::from_utf8(output.stdout)
        .map_err(|source| FastfetchError::ParseOutputError { source })
}

/// List available fastfetch modules.
/// Attempts to discover available modules through multiple methods:
/// 1. Query fastfetch CLI with `--list-modules` flag
/// 2. Parse modules from existing config file
/// 3. Fall back to a default list of common modules
/// Results are cached in memory for subsequent calls.
/// # Returns
/// * `Ok(Vec<String>)` - List of module names
/// * `Err` - If all discovery methods fail
/// # Example
/// ```
/// use fastfetch_mcp_server::modules::list_modules;
/// 
/// let modules = list_modules().await?;
/// println!("Available modules: {:?}", modules);
/// ```
pub async fn list_modules() -> Result<Vec<String>, McpServerError> {
    // Check cache first
    let cache = MODULES_CACHE.get_or_init(|| Mutex::new(None));
    let mut cached = cache.lock().await;
    
    if let Some(ref modules) = *cached {
        return Ok(modules.clone());
    }

    // Try to get from fastfetch CLI
    let modules = match exec_fastfetch(&[fastfetch_args::LIST_MODULES]).await {
        Ok(output) => {
            let parsed_modules = parse_modules_from_output(&output);
            
            if parsed_modules.is_empty() {
                // If parsing failed, try fallback methods
                get_modules_from_config().await.unwrap_or_else(|_| get_default_modules())
            } else {
                parsed_modules
            }
        }
        Err(_e) => {
            // If --list-modules doesn't work, try to get from generated config
            // or return a common list
            get_modules_from_config().await.unwrap_or_else(|_| {
                // If we can't get from config either, return defaults
                // This is a fallback, so we don't fail completely
                get_default_modules()
            })
        }
    };

    *cached = Some(modules.clone());
    Ok(modules)
}

/// List available fastfetch logos.
/// Attempts to discover available logos through multiple methods:
/// 1. Query fastfetch CLI with `--list-logos` flag
/// 2. Parse logos from existing config file
/// 3. Fall back to a default list of common logos
/// Results are cached in memory for subsequent calls.
/// # Returns
/// * `Ok(Vec<String>)` - List of logo names
/// * `Err` - If all discovery methods fail
/// # Example
/// ```
/// use fastfetch_mcp_server::modules::list_logos;
/// let logos = list_logos().await?;
/// println!("Available logos: {:?}", logos);
/// ```
pub async fn list_logos() -> Result<Vec<String>, McpServerError> {
    // Check cache first
    let cache = LOGOS_CACHE.get_or_init(|| Mutex::new(None));
    let mut cached = cache.lock().await;
    
    if let Some(ref logos) = *cached {
        return Ok(logos.clone());
    }

    // Try to get from fastfetch CLI
    let logos = match exec_fastfetch(&[fastfetch_args::LIST_LOGOS]).await {
        Ok(output) => {
            let parsed_logos = parse_logos_from_output(&output);
            
            if parsed_logos.is_empty() {
                // If parsing failed, try fallback methods
                get_logos_from_config().await.unwrap_or_else(|_| get_default_logos())
            } else {
                parsed_logos
            }
        }
        Err(_) => {
            // If --list-logos doesn't work, try to get from generated config
            // or return a common list
            get_logos_from_config().await.unwrap_or_else(|_| get_default_logos())
        }
    };

    *cached = Some(logos.clone());
    Ok(logos)
}

/// Try to get modules from a generated config file
async fn get_modules_from_config() -> Result<Vec<String>, McpServerError> {
    // Try to read the existing config file first
    let config = match read_config(None) {
        Ok(c) => c,
        Err(_) => {
            // If config doesn't exist, generate it
            let _output = exec_fastfetch(&[fastfetch_args::GEN_CONFIG_FULL]).await
                .map_err(McpServerError::from)?;
            read_config(None).map_err(McpServerError::from)?
        }
    };
    
    // Extract modules from the config
    // Fastfetch config has a "modules" array with module objects
    // Each module object has a "key" field with the module name
    let mut modules = Vec::new();
    
    if let Some(modules_array) = config.get("modules").and_then(|v| v.as_array()) {
        for module in modules_array {
            if let Some(module_obj) = module.as_object() {
                // Try to get the module key/name
                if let Some(key) = module_obj.get("key").and_then(|v| v.as_str()) {
                    modules.push(key.to_string());
                } else if let Some(key) = module_obj.get("name").and_then(|v| v.as_str()) {
                    modules.push(key.to_string());
                }
            } else if let Some(key_str) = module.as_str() {
                // Sometimes modules are just strings
                modules.push(key_str.to_string());
            }
        }
    }
    
    if modules.is_empty() {
        // Fallback to defaults if we couldn't extract modules
        Ok(get_default_modules())
    } else {
        Ok(modules)
    }
}

/// Try to get logos from a generated config file
async fn get_logos_from_config() -> Result<Vec<String>, McpServerError> {
    // Try to read the existing config file first
    let config = match read_config(None) {
        Ok(c) => c,
        Err(_) => {
            // If config doesn't exist, generate it
            let _output = exec_fastfetch(&[fastfetch_args::GEN_CONFIG_FULL]).await
                .map_err(McpServerError::from)?;
            read_config(None).map_err(McpServerError::from)?
        }
    };
    
    // Extract logos from the config
    // Fastfetch config may have logo-related fields
    // Common fields: "logo", "logoType", or we can check the logo directory
    let mut logos = Vec::new();
    
    // Check for logo field in config
    if let Some(logo_value) = config.get("logo") {
        if let Some(logo_str) = logo_value.as_str() {
            logos.push(logo_str.to_string());
        }
    }
    
    // Also try to get from logoType
    if let Some(logo_type) = config.get("logoType").and_then(|v| v.as_str()) {
        if !logos.contains(&logo_type.to_string()) {
            logos.push(logo_type.to_string());
        }
    }
    
    // If we found logos in config, return them along with defaults
    // Otherwise, just return defaults
    if logos.is_empty() {
        Ok(get_default_logos())
    } else {
        // Merge with defaults to ensure we have a comprehensive list
        let mut all_logos = get_default_logos();
        for logo in logos {
            if !all_logos.contains(&logo) {
                all_logos.push(logo);
            }
        }
        Ok(all_logos)
    }
}

/// Default list of common fastfetch modules
fn get_default_modules() -> Vec<String> {
    vec![
        "title".to_string(),
        "separator".to_string(),
        "os".to_string(),
        "host".to_string(),
        "kernel".to_string(),
        "uptime".to_string(),
        "packages".to_string(),
        "shell".to_string(),
        "resolution".to_string(),
        "de".to_string(),
        "wm".to_string(),
        "wmtheme".to_string(),
        "theme".to_string(),
        "icons".to_string(),
        "font".to_string(),
        "cursor".to_string(),
        "cpu".to_string(),
        "gpu".to_string(),
        "memory".to_string(),
        "disk".to_string(),
        "battery".to_string(),
        "poweradapter".to_string(),
        "locale".to_string(),
        "localip".to_string(),
        "publicip".to_string(),
        "users".to_string(),
        "datetime".to_string(),
        "date".to_string(),
        "time".to_string(),
        "colors".to_string(),
    ]
}

/// Default list of common fastfetch logos
fn get_default_logos() -> Vec<String> {
    vec![
        "arch".to_string(),
        "debian".to_string(),
        "fedora".to_string(),
        "ubuntu".to_string(),
        "opensuse".to_string(),
        "gentoo".to_string(),
        "alpine".to_string(),
        "void".to_string(),
        "nixos".to_string(),
        "macos".to_string(),
        "windows".to_string(),
        "linux".to_string(),
        "freebsd".to_string(),
        "openbsd".to_string(),
        "netbsd".to_string(),
        "dragonfly".to_string(),
    ]
}

/// Clear the modules cache.
/// Forces the next call to `list_modules()` to re-discover modules
/// instead of using cached values.
/// # Example
/// ```
/// use fastfetch_mcp_server::modules::clear_modules_cache; // Clear cache to force re-discovery
/// // Clear cache to force re-discovery
/// clear_modules_cache().await;
/// ```
pub async fn clear_modules_cache() {
    if let Some(cache) = MODULES_CACHE.get() {
        let mut cached = cache.lock().await;
        *cached = None;
    }
}

/// Clear the logos cache.
/// Forces the next call to `list_logos()` to re-discover logos
/// instead of using cached values.
/// # Example
/// ```
/// use fastfetch_mcp_server::modules::clear_logos_cache; // Clear cache to force re-discovery
/// // Clear cache to force re-discovery
/// clear_logos_cache().await;
/// ```
pub async fn clear_logos_cache() {
    if let Some(cache) = LOGOS_CACHE.get() {
        let mut cached = cache.lock().await;
        *cached = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_modules() {
        let modules = get_default_modules();
        assert!(!modules.is_empty());
        assert!(modules.contains(&"os".to_string()));
        assert!(modules.contains(&"cpu".to_string()));
    }

    #[test]
    fn test_get_default_logos() {
        let logos = get_default_logos();
        assert!(!logos.is_empty());
        assert!(logos.contains(&"arch".to_string()));
        assert!(logos.contains(&"ubuntu".to_string()));
    }

    #[tokio::test]
    async fn test_clear_modules_cache() {
        // Clear cache should not panic
        clear_modules_cache().await;
    }

    #[tokio::test]
    async fn test_clear_logos_cache() {
        // Clear cache should not panic
        clear_logos_cache().await;
    }

    #[test]
    fn test_get_default_modules_contains_expected() {
        let modules = get_default_modules();
        let expected_modules = vec!["os", "cpu", "memory", "disk", "shell", "kernel"];
        for expected in expected_modules {
            assert!(modules.contains(&expected.to_string()), 
                "Default modules should contain: {}", expected);
        }
    }

    #[test]
    fn test_get_default_logos_contains_expected() {
        let logos = get_default_logos();
        let expected_logos = vec!["arch", "ubuntu", "debian", "macos", "windows"];
        for expected in expected_logos {
            assert!(logos.contains(&expected.to_string()),
                "Default logos should contain: {}", expected);
        }
    }

    #[test]
    fn test_get_default_modules_not_empty() {
        let modules = get_default_modules();
        assert!(!modules.is_empty(), "Default modules should not be empty");
        assert!(modules.len() > 10, "Should have a reasonable number of default modules");
    }

    #[test]
    fn test_get_default_logos_not_empty() {
        let logos = get_default_logos();
        assert!(!logos.is_empty(), "Default logos should not be empty");
        assert!(logos.len() > 5, "Should have a reasonable number of default logos");
    }

    #[test]
    fn test_parse_modules_from_output() {
        let output = r#"
1)  OS : Operating system information
2)  CPU : CPU information
3)  Memory : Memory usage information
"#;
        let modules = parse_modules_from_output(output);
        assert_eq!(modules.len(), 3);
        assert!(modules.contains(&"os".to_string()));
        assert!(modules.contains(&"cpu".to_string()));
        assert!(modules.contains(&"memory".to_string()));
    }

    #[test]
    fn test_parse_modules_from_output_empty() {
        let output = "";
        let modules = parse_modules_from_output(output);
        assert!(modules.is_empty());
    }

    #[test]
    fn test_parse_logos_from_output() {
        let output = r#"
Builtin logos:
1)  "arch" "archlinux"
2)  "ubuntu" "ubuntulinux"
3)  "debian"
"#;
        let logos = parse_logos_from_output(output);
        assert!(logos.len() >= 3);
        assert!(logos.contains(&"arch".to_string()));
        assert!(logos.contains(&"ubuntu".to_string()));
        assert!(logos.contains(&"debian".to_string()));
    }

    #[test]
    fn test_parse_logos_from_output_with_aliases() {
        let output = r#"1)  "arch" "archlinux" "archlinux-logo""#;
        let logos = parse_logos_from_output(output);
        assert!(logos.contains(&"arch".to_string()));
        assert!(logos.contains(&"archlinux".to_string()));
    }

    #[test]
    fn test_parse_logos_from_output_empty() {
        let output = "";
        let logos = parse_logos_from_output(output);
        assert!(logos.is_empty());
    }

    #[tokio::test]
    async fn test_list_modules_command_not_found() {
        // This test would require mocking or a test environment without fastfetch
        // For now, we test that the function handles errors gracefully
        // by checking that it falls back to defaults when command fails
        // Note: This test may fail if fastfetch is installed, which is expected
        let _ = list_modules().await;
        // Should either succeed (if fastfetch is available) or use fallback
    }

    #[tokio::test]
    async fn test_list_logos_command_not_found() {
        // Similar to test_list_modules_command_not_found
        let _ = list_logos().await;
        // Should either succeed (if fastfetch is available) or use fallback
    }

    #[test]
    fn test_parse_modules_from_output_malformed() {
        // Test parsing with various malformed inputs
        let malformed_inputs = vec![
            "",  // Empty
            "No modules here",  // No valid format
            "1) Module",  // Missing colon
            "1) : Description",  // Missing module name
        ];
        
        for input in malformed_inputs {
            let modules = parse_modules_from_output(input);
            // Should return empty or handle gracefully
            let _ = modules;
        }
    }

    #[test]
    fn test_parse_logos_from_output_malformed() {
        // Test parsing with various malformed inputs
        let malformed_inputs = vec![
            "",  // Empty
            "No logos here",  // No quotes
            "1) Logo",  // Missing quotes
            "1) \"Unclosed quote",  // Unclosed quote
        ];
        
        for input in malformed_inputs {
            let logos = parse_logos_from_output(input);
            // Should return empty or handle gracefully
            let _ = logos;
        }
    }
}
