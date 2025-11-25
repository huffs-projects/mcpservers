use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Validates and sanitizes file paths to prevent directory traversal attacks
pub struct PathValidator {
    allowed_directories: Vec<PathBuf>,
}

impl PathValidator {
    pub fn new(allowed_directories: Vec<PathBuf>) -> Self {
        Self {
            allowed_directories,
        }
    }

    /// Validates a path and returns a canonicalized, safe path
    pub fn validate_path(&self, path: &str) -> Result<PathBuf> {
        let input_path = Path::new(path);
        
        // Check for path traversal attempts
        if path.contains("..") {
            return Err(anyhow::anyhow!("Path traversal detected: '..' not allowed"));
        }

        // Check for absolute paths that might escape
        if input_path.is_absolute() {
            // Only allow if it's within an allowed directory
            let canonical = input_path
                .canonicalize()
                .context("Failed to canonicalize path")?;
            
            return self.check_allowed(&canonical);
        }

        // For relative paths, canonicalize and check
        let canonical = std::env::current_dir()
            .context("Failed to get current directory")?
            .join(input_path)
            .canonicalize()
            .context("Failed to canonicalize path")?;

        self.check_allowed(&canonical)
    }

    fn check_allowed(&self, path: &Path) -> Result<PathBuf> {
        if self.allowed_directories.is_empty() {
            // If no restrictions, allow all
            return Ok(path.to_path_buf());
        }

        for allowed_dir in &self.allowed_directories {
            if path.starts_with(allowed_dir) {
                return Ok(path.to_path_buf());
            }
        }

        Err(anyhow::anyhow!(
            "Path '{}' is not within allowed directories",
            path.display()
        ))
    }

    /// Validates path format and length
    pub fn validate_path_format(path: &str) -> Result<()> {
        // Maximum path length (reasonable limit)
        const MAX_PATH_LENGTH: usize = 4096;

        if path.is_empty() {
            return Err(anyhow::anyhow!("Path cannot be empty"));
        }

        if path.len() > MAX_PATH_LENGTH {
            return Err(anyhow::anyhow!(
                "Path exceeds maximum length of {} characters",
                MAX_PATH_LENGTH
            ));
        }

        // Check for null bytes
        if path.contains('\0') {
            return Err(anyhow::anyhow!("Path cannot contain null bytes"));
        }

        Ok(())
    }
}

impl Default for PathValidator {
    fn default() -> Self {
        // Default: allow current directory and home directory
        let mut allowed = vec![
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        ];
        
        if let Ok(home) = std::env::var("HOME") {
            allowed.push(PathBuf::from(home));
        }

        Self::new(allowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path_format_empty() {
        assert!(PathValidator::validate_path_format("").is_err());
    }

    #[test]
    fn test_validate_path_format_null_byte() {
        assert!(PathValidator::validate_path_format("path\0with\0null").is_err());
    }

    #[test]
    fn test_validate_path_format_too_long() {
        let long_path = "a".repeat(5000);
        assert!(PathValidator::validate_path_format(&long_path).is_err());
    }

    #[test]
    fn test_validate_path_format_valid() {
        assert!(PathValidator::validate_path_format("/valid/path").is_ok());
    }

    #[test]
    fn test_validate_path_traversal() {
        let validator = PathValidator::default();
        assert!(validator.validate_path("../etc/passwd").is_err());
        assert!(validator.validate_path("../../etc/passwd").is_err());
    }
}
