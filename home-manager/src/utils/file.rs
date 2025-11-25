use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path.display()))
}

pub fn backup_file(path: &Path, backup_path: Option<&Path>) -> Result<PathBuf> {
    let backup = if let Some(custom_path) = backup_path {
        custom_path.to_path_buf()
    } else {
        let mut backup_path = path.to_path_buf();
        let extension = backup_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bak");
        backup_path.set_extension(format!("{}.backup", extension));
        backup_path
    };

    if path.exists() {
        fs::copy(path, &backup)
            .with_context(|| format!("Failed to create backup: {}", backup.display()))?;
    }

    Ok(backup)
}

pub fn create_temp_dir() -> Result<TempDir> {
    tempfile::tempdir().context("Failed to create temporary directory")
}

pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_write_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let content = "test content\nline 2";
        write_file(path, content).unwrap();

        let read_content = read_file(path).unwrap();
        assert_eq!(content, read_content);
    }

    #[test]
    fn test_file_exists() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        assert!(file_exists(path));
        
        let non_existent = path.join("nonexistent");
        assert!(!file_exists(&non_existent));
    }

    #[test]
    fn test_backup_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        
        write_file(path, "original content").unwrap();

        let backup = backup_file(path, None).unwrap();
        assert!(backup.exists());
        
        let backup_content = read_file(&backup).unwrap();
        assert_eq!("original content", backup_content);
    }

    #[test]
    fn test_backup_file_custom_path() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        let custom_backup = temp_file.path().parent().unwrap().join("custom.backup");
        
        write_file(path, "original content").unwrap();

        let backup = backup_file(path, Some(&custom_backup)).unwrap();
        assert_eq!(backup, custom_backup);
        assert!(backup.exists());
    }

    #[test]
    fn test_create_temp_dir() {
        let temp_dir = create_temp_dir().unwrap();
        assert!(temp_dir.path().exists());
        assert!(temp_dir.path().is_dir());
    }

    #[test]
    fn test_write_file_creates_directory() {
        let temp_dir = create_temp_dir().unwrap();
        let nested_path = temp_dir.path().join("nested").join("file.txt");
        
        write_file(&nested_path, "content").unwrap();
        assert!(nested_path.exists());
    }
}
