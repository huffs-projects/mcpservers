use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

const MAX_PATH_LENGTH: usize = 4096;
const ALLOWED_SCHEMES: &[&str] = &["file", "http", "https"];

pub fn validate_path(path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy();
    
    if path_str.len() > MAX_PATH_LENGTH {
        anyhow::bail!("Path exceeds maximum length of {} characters", MAX_PATH_LENGTH);
    }

    if path_str.contains("..") {
        anyhow::bail!("Path contains '..' which is not allowed");
    }

    if path_str.contains('\0') {
        anyhow::bail!("Path contains null byte which is not allowed");
    }

    Ok(())
}

pub fn sanitize_path(path: &str) -> Result<PathBuf> {
    let sanitized = path
        .replace("..", "")
        .replace('\0', "")
        .trim()
        .to_string();

    if sanitized.is_empty() {
        anyhow::bail!("Path is empty after sanitization");
    }

    let path = PathBuf::from(sanitized);
    validate_path(&path)?;
    
    Ok(path)
}

pub fn ensure_path_within_base(path: &Path, base: &Path) -> Result<PathBuf> {
    validate_path(path)?;
    
    let canonical_base = base.canonicalize()
        .context("Failed to canonicalize base path")?;
    
    let canonical_path = path.canonicalize()
        .context("Failed to canonicalize path")?;

    if !canonical_path.starts_with(&canonical_base) {
        anyhow::bail!(
            "Path {} is not within base directory {}",
            canonical_path.display(),
            canonical_base.display()
        );
    }

    Ok(canonical_path)
}

pub fn validate_file_extension(path: &Path, allowed_extensions: &[&str]) -> Result<()> {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        if allowed_extensions.iter().any(|&allowed| allowed == ext_str) {
            return Ok(());
        }
    }
    
    anyhow::bail!(
        "File extension not allowed. Allowed extensions: {:?}",
        allowed_extensions
    )
}

pub fn validate_search_term(term: &str) -> Result<()> {
    if term.len() > 1000 {
        anyhow::bail!("Search term exceeds maximum length of 1000 characters");
    }

    if term.contains('\0') {
        anyhow::bail!("Search term contains null byte");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_path_normal() {
        let path = Path::new("/home/user/.config");
        assert!(validate_path(path).is_ok());
    }

    #[test]
    fn test_validate_path_traversal() {
        let path = Path::new("/home/user/../../etc/passwd");
        assert!(validate_path(path).is_err());
    }

    #[test]
    fn test_sanitize_path() {
        let sanitized = sanitize_path("/home/user/../../etc/passwd").unwrap();
        assert!(!sanitized.to_string_lossy().contains(".."));
    }

    #[test]
    fn test_ensure_path_within_base() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();
        let file_path = base.join("subdir").join("file.txt");
        std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        std::fs::write(&file_path, "test").unwrap();

        assert!(ensure_path_within_base(&file_path, base).is_ok());
    }

    #[test]
    fn test_ensure_path_outside_base() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();
        let outside_path = Path::new("/etc/passwd");

        assert!(ensure_path_within_base(outside_path, base).is_err());
    }

    #[test]
    fn test_validate_file_extension() {
        let path = Path::new("config.nix");
        assert!(validate_file_extension(path, &["nix"]).is_ok());
        assert!(validate_file_extension(path, &["txt"]).is_err());
    }

    #[test]
    fn test_validate_search_term() {
        assert!(validate_search_term("git").is_ok());
        assert!(validate_search_term(&"a".repeat(1001)).is_err());
    }
}

