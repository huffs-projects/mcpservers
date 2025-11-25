use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use tempfile::NamedTempFile;

/// Atomic write with rollback capability
pub struct AtomicWrite {
    temp_file: NamedTempFile,
    target_path: PathBuf,
    backup_path: Option<PathBuf>,
}

impl AtomicWrite {
    pub fn new(target_path: &Path) -> Result<Self> {
        let temp_file = NamedTempFile::new_in(
            target_path.parent().unwrap_or_else(|| Path::new("."))
        ).context("Failed to create temp file")?;

        Ok(Self {
            temp_file,
            target_path: target_path.to_path_buf(),
            backup_path: None,
        })
    }

    pub fn write(&mut self, content: &str) -> Result<()> {
        fs::write(&self.temp_file, content)
            .context("Failed to write to temp file")?;
        Ok(())
    }

    pub fn create_backup(&mut self) -> Result<()> {
        if self.target_path.exists() {
            let state_home = std::env::var("XDG_STATE_HOME")
                .unwrap_or_else(|_| {
                    std::env::var("HOME")
                        .map(|h| format!("{}/.local/state", h))
                        .unwrap_or_else(|_| "/tmp".to_string())
                });
            
            let backup_dir = PathBuf::from(state_home).join("wofi-mcp-backups");
            fs::create_dir_all(&backup_dir)
                .context("Failed to create backup directory")?;
            
            let backup_name = format!(
                "{}.{}",
                self.target_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("config"),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            
            self.backup_path = Some(backup_dir.join(backup_name));
            fs::copy(&self.target_path, self.backup_path.as_ref().unwrap())
                .context("Failed to create backup")?;
        }
        Ok(())
    }

    pub fn commit(self) -> Result<PathBuf> {
        self.temp_file
            .persist(&self.target_path)
            .context("Failed to persist temp file")?;
        
        Ok(self.backup_path.unwrap_or_else(|| self.target_path.clone()))
    }

    pub fn rollback(self) -> Result<()> {
        // Temp file is automatically deleted on drop
        if let Some(backup) = &self.backup_path {
            if backup.exists() {
                fs::copy(backup, &self.target_path)
                    .context("Failed to restore from backup")?;
            }
        }
        Ok(())
    }
}

