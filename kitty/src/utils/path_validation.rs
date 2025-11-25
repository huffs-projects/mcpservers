use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// Validate that a file path is safe to access
/// 
/// Prevents directory traversal attacks by ensuring the path is within
/// allowed directories (typically the user's config directory).
/// 
/// # Arguments
/// * `path` - The path to validate
/// * `allowed_base` - The base directory that paths must be within
/// 
/// # Returns
/// * `Ok(PathBuf)` - The canonicalized, validated path
/// * `Err` - If the path is invalid or outside allowed directory
/// 
/// # Example
/// ```
/// use kitty_mcp_server::utils::path_validation::validate_path;
/// 
/// let config_dir = std::env::var("HOME").unwrap() + "/.config/kitty";
/// let result = validate_path("kitty.conf", &config_dir);
/// ```
pub fn validate_path(path: &str, allowed_base: &str) -> Result<PathBuf> {
    let base_path = Path::new(allowed_base)
        .canonicalize()
        .context("Failed to canonicalize base path")?;
    
    let file_path = Path::new(path);
    
    // If path is relative, resolve it relative to base
    let resolved_path = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        base_path.join(file_path)
    };
    
    let canonical_path = resolved_path
        .canonicalize()
        .context("Failed to canonicalize file path")?;
    
    // Ensure the canonical path is within the base directory
    if !canonical_path.starts_with(&base_path) {
        anyhow::bail!("Path is outside allowed directory: {}", path);
    }
    
    Ok(canonical_path)
}

/// Get the default Kitty config directory
/// 
/// Returns the standard location for kitty.conf based on the platform.
pub fn default_kitty_config_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config").join("kitty")
    } else if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config).join("kitty")
    } else {
        // Fallback
        PathBuf::from("~/.config/kitty")
    }
}

/// Validate a config file path with default restrictions
/// 
/// Validates that the path is within the user's config directory.
pub fn validate_config_path(path: &str) -> Result<PathBuf> {
    let config_dir = default_kitty_config_dir();
    let config_dir_str = config_dir.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid config directory path"))?;
    
    validate_path(path, config_dir_str)
}

