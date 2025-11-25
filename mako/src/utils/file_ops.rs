use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Read config file content
pub fn read_config(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))
}

/// Create a backup of the config file with timestamp
pub fn create_backup(path: &Path, backup_path: Option<&Path>) -> Result<PathBuf> {
    let backup = match backup_path {
        Some(p) => p.to_path_buf(),
        None => {
            let timestamp = SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .with_context(|| "System time is before UNIX epoch")?
                .as_secs();
            let file_name = path
                .file_name()
                .with_context(|| format!("Path has no file name: {}", path.display()))?
                .to_string_lossy();
            let mut backup = path.to_path_buf();
            backup.set_file_name(format!("{}.backup.{}", file_name, timestamp));
            backup
        }
    };

    fs::copy(path, &backup)
        .with_context(|| format!("Failed to create backup: {}", backup.display()))?;

    Ok(backup)
}

/// Write config file atomically using a temporary file
pub fn write_config_atomic(path: &Path, content: &str) -> Result<()> {
    let _parent = path.parent().context("Config path has no parent directory")?;
    
    // Create temp file in same directory
    let file_name = path
        .file_name()
        .with_context(|| format!("Path has no file name: {}", path.display()))?
        .to_string_lossy();
    let mut temp_path = path.to_path_buf();
    temp_path.set_file_name(format!(".{}~", file_name));

    // Write to temp file
    fs::write(&temp_path, content)
        .with_context(|| format!("Failed to write temp file: {}", temp_path.display()))?;

    // Atomic rename
    fs::rename(&temp_path, path)
        .with_context(|| format!("Failed to rename temp file to config: {}", path.display()))?;

    Ok(())
}

/// Check if config file exists
pub fn config_exists(path: &Path) -> bool {
    path.exists()
}

/// Ensure parent directory exists
pub fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
    }
    Ok(())
}

