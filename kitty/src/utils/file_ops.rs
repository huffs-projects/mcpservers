use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Create a backup of a file with timestamp (async)
pub async fn backup_file(file_path: &str) -> Result<PathBuf> {
    let path = Path::new(file_path);
    let parent = path.parent().context("Invalid file path")?;
    let file_name = path.file_name().context("Invalid file name")?.to_str().unwrap();
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let backup_name = format!("{}.backup.{}", file_name, timestamp);
    let backup_path = parent.join(backup_name);
    
    fs::copy(path, &backup_path)
        .await
        .context("Failed to create backup")?;
    
    Ok(backup_path)
}

/// Atomic write: write to temp file, then rename (async)
pub async fn atomic_write(file_path: &str, content: &str) -> Result<()> {
    let path = Path::new(file_path);
    let parent = path.parent().context("Invalid file path")?;
    let file_name = path.file_name().context("Invalid file name")?.to_str().unwrap();
    
    let temp_path = parent.join(format!("{}.tmp", file_name));
    
    // Write to temp file
    fs::write(&temp_path, content.as_bytes())
        .await
        .context("Failed to write temp file")?;
    
    // Atomic rename
    fs::rename(&temp_path, path)
        .await
        .context("Failed to rename temp file")?;
    
    Ok(())
}


