use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Atomic file operations with backup support
pub struct AtomicFileOps;

impl AtomicFileOps {
    /// Write content to a file atomically with backup
    pub fn write_with_backup<P: AsRef<Path>>(
        path: P,
        content: &str,
    ) -> Result<PathBuf, String> {
        let path = path.as_ref();
        let backup_path = Self::create_backup_path(path)?;

        // Create backup if file exists
        if path.exists() {
            fs::copy(path, &backup_path)
                .map_err(|e| format!("Failed to create backup: {}", e))?;
        }

        // Write to temporary file first
        let temp_path = path.with_extension("tmp");
        let mut file = fs::File::create(&temp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write content: {}", e))?;
        
        file.sync_all()
            .map_err(|e| format!("Failed to sync file: {}", e))?;

        // Atomic rename
        fs::rename(&temp_path, path)
            .map_err(|e| format!("Failed to rename temp file: {}", e))?;

        Ok(backup_path)
    }

    /// Create a backup path with timestamp
    fn create_backup_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, String> {
        let path = path.as_ref();
        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Failed to get timestamp: {}", e))?
            .as_secs();

        let backup_name = format!(
            "{}.backup.{}",
            path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| "Invalid file name".to_string())?,
            timestamp
        );

        Ok(path.parent()
            .ok_or_else(|| "No parent directory".to_string())?
            .join(backup_name))
    }

    /// Restore from backup
    pub fn restore_from_backup<P: AsRef<Path>>(
        path: P,
        backup_path: P,
    ) -> Result<(), String> {
        let path = path.as_ref();
        let backup_path = backup_path.as_ref();

        if !backup_path.exists() {
            return Err("Backup file does not exist".to_string());
        }

        fs::copy(backup_path, path)
            .map_err(|e| format!("Failed to restore from backup: {}", e))?;

        Ok(())
    }

    /// Cross-platform path handling
    pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
        let path = path.as_ref();
        path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
    }

    /// Ensure directory exists
    pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let path = path.as_ref();
        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
        Ok(())
    }
}

