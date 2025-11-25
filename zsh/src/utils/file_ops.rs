use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;

/// Expands `~` and `$HOME` in a path string to the user's home directory.
/// 
/// # Examples
/// 
/// ```
/// use zsh_mcp_server::utils::file_ops::expand_path;
/// 
/// let path = expand_path("~/.zshrc").unwrap();
/// assert!(path.to_string_lossy().contains(".zshrc"));
/// ```
pub fn expand_path(path_str: &str) -> Result<PathBuf> {
    let expanded = if path_str.starts_with("~/") {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        home.join(&path_str[2..])
    } else if path_str.starts_with('~') && path_str.len() == 1 {
        dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))
            .map_err(|e| anyhow::anyhow!("{}", e))?
    } else if path_str.contains("$HOME") {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .to_string_lossy()
            .to_string();
        PathBuf::from(path_str.replace("$HOME", &home))
    } else {
        PathBuf::from(path_str)
    };
    Ok(expanded)
}

pub fn read_config_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))
}

pub fn write_config_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)
        .with_context(|| format!("Failed to write config file: {}", path.display()))
}

pub fn create_backup(path: &Path, backup_dir: Option<&Path>) -> Result<PathBuf> {
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let base_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("zshrc");
    let backup_name = format!("{}.backup.{}", base_name, timestamp);
    
    let backup_path = if let Some(dir) = backup_dir {
        dir.join(&backup_name)
    } else {
        path.parent()
            .map(|p| p.join(&backup_name))
            .unwrap_or_else(|| PathBuf::from(backup_name))
    };

    if let Some(parent) = backup_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create backup directory: {}", parent.display()))?;
    }

    fs::copy(path, &backup_path)
        .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;

    Ok(backup_path)
}

pub fn atomic_write(path: &Path, content: &str) -> Result<()> {
    let temp_path = path.with_extension("tmp");
    
    write_config_file(&temp_path, content)
        .with_context(|| format!("Failed to write temporary file: {}", temp_path.display()))?;
    
    fs::rename(&temp_path, path)
        .with_context(|| format!("Failed to rename temporary file to: {}", path.display()))?;
    
    Ok(())
}

pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

/// Gets the default path to the user's `.zshrc` file.
/// 
/// Returns `~/.zshrc` if home directory can be determined, otherwise `.zshrc`.
/// 
/// # Returns
/// 
/// PathBuf pointing to the default zshrc location.
#[allow(dead_code)]
pub fn get_default_zshrc_path() -> PathBuf {
    dirs::home_dir()
        .map(|home| home.join(".zshrc"))
        .unwrap_or_else(|| PathBuf::from(".zshrc"))
}

