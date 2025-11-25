use anyhow::{Context, Result};
use fs2::FileExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct FileManager {
    lock: Arc<Mutex<()>>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            lock: Arc::new(Mutex::new(())),
        }
    }

    pub async fn read_config(&self, path: impl AsRef<Path>) -> Result<String> {
        let path = path.as_ref().to_path_buf();
        let _guard = self.lock.lock().await;
        
        // Use tokio::fs for async file operations
        // File locking needs to be done on blocking thread
        let contents = tokio::task::spawn_blocking(move || {
            let file = std::fs::File::open(&path)
                .with_context(|| format!("Failed to open config file: {}", path.display()))?;
            
            file.lock_shared()
                .context("Failed to acquire shared lock on config file")?;
            
            let contents = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read config file: {}", path.display()))?;
            
            file.unlock()
                .context("Failed to release lock on config file")?;
            
            Ok::<String, anyhow::Error>(contents)
        })
        .await
        .context("Failed to join blocking task")??;
        
        Ok(contents)
    }

    pub async fn write_config(
        &self,
        path: impl AsRef<Path>,
        contents: &str,
    ) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        let contents = contents.to_string();
        let _guard = self.lock.lock().await;
        
        // Use tokio::fs for async file operations
        // File locking needs to be done on blocking thread
        tokio::task::spawn_blocking(move || {
            let file = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&path)
                .with_context(|| format!("Failed to open config file for writing: {}", path.display()))?;
            
            file.lock_exclusive()
                .context("Failed to acquire exclusive lock on config file")?;
            
            std::fs::write(&path, &contents)
                .with_context(|| format!("Failed to write config file: {}", path.display()))?;
            
            file.sync_all()
                .context("Failed to sync config file")?;
            
            file.unlock()
                .context("Failed to release lock on config file")?;
            
            Ok::<(), anyhow::Error>(())
        })
        .await
        .context("Failed to join blocking task")??;
        
        Ok(())
    }

    pub async fn create_backup(
        &self,
        config_path: impl AsRef<Path>,
        backup_path: Option<impl AsRef<Path>>,
    ) -> Result<PathBuf> {
        let config_path = config_path.as_ref();
        let backup_path = match backup_path {
            Some(p) => p.as_ref().to_path_buf(),
            None => {
                let mut backup = config_path.to_path_buf();
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                backup.set_file_name(format!(
                    "{}.backup.{}",
                    config_path.file_name().unwrap().to_string_lossy(),
                    timestamp
                ));
                backup
            }
        };

        let contents = self.read_config(config_path).await?;
        self.write_config(&backup_path, &contents).await?;
        
        Ok(backup_path)
    }

    pub fn compute_diff(old: &str, new: &str) -> String {
        let diff = diff::lines(old, new);
        let mut result = String::new();
        
        for change in diff {
            match change {
                diff::Result::Left(line) => {
                    result.push_str(&format!("-{}\n", line));
                }
                diff::Result::Both(line, _) => {
                    result.push_str(&format!(" {}\n", line));
                }
                diff::Result::Right(line) => {
                    result.push_str(&format!("+{}\n", line));
                }
            }
        }
        
        result
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_read_write_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.toml");
        let content = "[git_branch]\nformat = \"$branch\"";

        let manager = FileManager::new();
        manager.write_config(&config_path, content).await.unwrap();
        
        let read_content = manager.read_config(&config_path).await.unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_compute_diff() {
        let old = "line1\nline2\nline3";
        let new = "line1\nline2_modified\nline3\nline4";
        let diff = FileManager::compute_diff(old, new);
        assert!(diff.contains("-line2"));
        assert!(diff.contains("+line2_modified"));
        assert!(diff.contains("+line4"));
    }
}
