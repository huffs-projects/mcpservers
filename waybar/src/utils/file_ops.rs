use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FileOps;

impl FileOps {
    pub fn atomic_write(path: &str, content: &str) -> Result<()> {
        let temp_path = format!("{}.tmp", path);
        fs::write(&temp_path, content)
            .with_context(|| format!("Failed to write temporary file: {}", temp_path))?;
        fs::rename(&temp_path, path)
            .with_context(|| format!("Failed to rename temporary file to: {}", path))?;
        Ok(())
    }

    pub fn create_backup(path: &str, backup_dir: Option<&str>) -> Result<String> {
        let backup_path = if let Some(dir) = backup_dir {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let filename = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("backup");
            format!("{}/{}.{}", dir, filename, timestamp)
        } else {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            format!("{}.backup.{}", path, timestamp)
        };

        fs::copy(path, &backup_path)
            .with_context(|| format!("Failed to create backup: {}", backup_path))?;

        Ok(backup_path)
    }

    pub fn ensure_directory(path: &str) -> Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }
        Ok(())
    }

    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn read_file(path: &str) -> Result<String> {
        let expanded = Self::expand_path(path)?;
        fs::read_to_string(&expanded)
            .with_context(|| format!("Failed to read file: {}", expanded.display()))
    }

    /// Expand tilde (~) in path to user's home directory
    /// Also expands $HOME and $XDG_CONFIG_HOME environment variables
    pub fn expand_path(path: &str) -> Result<PathBuf> {
        let path = path.trim();
        
        // Handle tilde expansion
        if path.starts_with("~/") {
            let home = env::var("HOME")
                .context("HOME environment variable not set")?;
            return Ok(PathBuf::from(home).join(&path[2..]));
        }
        
        // Handle $HOME expansion
        if path.starts_with("$HOME/") {
            let home = env::var("HOME")
                .context("HOME environment variable not set")?;
            return Ok(PathBuf::from(home).join(&path[6..]));
        }
        
        // Handle $XDG_CONFIG_HOME expansion
        if path.starts_with("$XDG_CONFIG_HOME/") {
            let xdg_config = env::var("XDG_CONFIG_HOME")
                .or_else(|_| {
                    env::var("HOME").map(|home| format!("{}/.config", home))
                })
                .context("Could not determine XDG_CONFIG_HOME")?;
            return Ok(PathBuf::from(xdg_config).join(&path[18..]));
        }
        
        // Expand other environment variables
        let expanded = if path.contains('$') {
            let mut result = path.to_string();
            for (key, value) in env::vars() {
                let var = format!("${}", key);
                if result.contains(&var) {
                    result = result.replace(&var, &value);
                }
            }
            PathBuf::from(result)
        } else {
            PathBuf::from(path)
        };
        
        Ok(expanded)
    }

    /// Validate that a path exists and is a file
    pub fn validate_file_path(path: &str) -> Result<PathBuf> {
        let expanded = Self::expand_path(path)?;
        if !expanded.exists() {
            return Err(anyhow::anyhow!(
                "File does not exist: {} (expanded from: {})",
                expanded.display(),
                path
            ));
        }
        if !expanded.is_file() {
            return Err(anyhow::anyhow!(
                "Path is not a file: {} (expanded from: {})",
                expanded.display(),
                path
            ));
        }
        Ok(expanded)
    }

    /// Validate that a path exists and is a directory
    pub fn validate_dir_path(path: &str) -> Result<PathBuf> {
        let expanded = Self::expand_path(path)?;
        if !expanded.exists() {
            return Err(anyhow::anyhow!(
                "Directory does not exist: {} (expanded from: {})",
                expanded.display(),
                path
            ));
        }
        if !expanded.is_dir() {
            return Err(anyhow::anyhow!(
                "Path is not a directory: {} (expanded from: {})",
                expanded.display(),
                path
            ));
        }
        Ok(expanded)
    }
}

