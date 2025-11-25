use crate::models::PatchResult;
use crate::utils::{file, security, validation};
use anyhow::{Context, Result};
use std::path::Path;
use tracing::{debug, info};

pub async fn apply_patch(
    file_path: &Path,
    patch: &str,
    dry_run: bool,
    backup_path: Option<&Path>,
) -> Result<PatchResult> {
    debug!(
        "Applying patch: file={}, dry_run={}, backup_path={:?}",
        file_path.display(),
        dry_run,
        backup_path
    );

    security::validate_path(file_path)
        .context("Invalid file path")?;
    
    validation::validate_patch_content(patch)
        .context("Invalid patch content")?;

    if let Some(backup) = backup_path {
        security::validate_path(backup)
            .context("Invalid backup path")?;
    }

    if !file_path.exists() {
        anyhow::bail!("File does not exist: {}", file_path.display());
    }

    let original_content = file::read_file(file_path)
        .context("Failed to read file to patch")?;

    let patched_content = apply_patch_to_content(&original_content, patch)?;

    if dry_run {
        let diff = generate_diff(&original_content, &patched_content);
        return Ok(PatchResult {
            success: true,
            diff_applied: Some(diff),
            backup_created: false,
            error: None,
        });
    }

    let _backup = file::backup_file(file_path, backup_path)
        .context("Failed to create backup")?;

    file::write_file(file_path, &patched_content)
        .context("Failed to write patched content")?;

    let diff = generate_diff(&original_content, &patched_content);

    // Audit logging for patch operations
    info!(
        "Patch applied: file={}, dry_run={}, backup_created=true",
        file_path.display(),
        dry_run
    );

    Ok(PatchResult {
        success: true,
        diff_applied: Some(diff),
        backup_created: true,
        error: None,
    })
}

fn apply_patch_to_content(original: &str, patch: &str) -> Result<String> {
    let lines: Vec<&str> = original.lines().collect();
    let patch_lines: Vec<&str> = patch.lines().collect();

    let mut result = Vec::new();
    let mut i = 0;
    let mut patch_idx = 0;
    let mut in_hunk = false;

    while patch_idx < patch_lines.len() {
        let patch_line = patch_lines[patch_idx];
        let trimmed = patch_line.trim();

        if trimmed.starts_with("@@") {
            in_hunk = true;
            patch_idx += 1;
            continue;
        }

        if !in_hunk {
            patch_idx += 1;
            continue;
        }

        if trimmed.starts_with("---") || trimmed.starts_with("+++") {
            patch_idx += 1;
            continue;
        }

        if trimmed.starts_with('+') && !trimmed.starts_with("+++") {
            result.push(trimmed[1..].trim_start().to_string());
            patch_idx += 1;
        } else if trimmed.starts_with('-') && !trimmed.starts_with("---") {
            if i < lines.len() {
                i += 1;
            }
            patch_idx += 1;
        } else if patch_line.starts_with(' ') {
            let context_line = patch_line[1..].trim_start();
            if i < lines.len() && lines[i] == context_line {
                result.push(lines[i].to_string());
                i += 1;
            } else if i < lines.len() {
                result.push(lines[i].to_string());
                i += 1;
            }
            patch_idx += 1;
        } else if trimmed.is_empty() {
            patch_idx += 1;
        } else {
            patch_idx += 1;
        }
    }

    while i < lines.len() {
        result.push(lines[i].to_string());
        i += 1;
    }

    Ok(result.join("\n"))
}

fn generate_diff(original: &str, modified: &str) -> String {
    use std::io::Write;
    let mut diff = Vec::new();
    
    writeln!(diff, "--- original")
        .expect("Writing to Vec should never fail");
    writeln!(diff, "+++ modified")
        .expect("Writing to Vec should never fail");
    
    let original_lines: Vec<&str> = original.lines().collect();
    let modified_lines: Vec<&str> = modified.lines().collect();
    
    let max_len = original_lines.len().max(modified_lines.len());
    
    for i in 0..max_len {
        let orig = original_lines.get(i).copied().unwrap_or("");
        let modif = modified_lines.get(i).copied().unwrap_or("");
        
        if orig != modif {
            if !orig.is_empty() {
                writeln!(diff, "-{}", orig)
                    .expect("Writing to Vec should never fail");
            }
            if !modif.is_empty() {
                writeln!(diff, "+{}", modif)
                    .expect("Writing to Vec should never fail");
            }
        } else {
            writeln!(diff, " {}", orig)
                .expect("Writing to Vec should never fail");
        }
    }
    
    String::from_utf8_lossy(&diff).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_apply_patch_dry_run() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        
        crate::utils::file::write_file(path, "line1\nline2\nline3").unwrap();

        let patch = "--- a/file\n+++ b/file\n@@ -1,3 +1,3 @@\n line1\n-line2\n+line2modified\n line3";
        
        let result = apply_patch(path, patch, true, None).await.unwrap();
        
        assert!(result.success);
        assert!(!result.backup_created);
        assert!(result.diff_applied.is_some());
    }

    #[tokio::test]
    async fn test_apply_patch_actual() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        
        crate::utils::file::write_file(path, "line1\nline2\nline3").unwrap();

        let patch = "--- a/file\n+++ b/file\n@@ -1,3 +1,3 @@\n line1\n-line2\n+line2modified\n line3";
        
        let result = apply_patch(path, patch, false, None).await.unwrap();
        
        assert!(result.success);
        assert!(result.backup_created);
        
        let content = crate::utils::file::read_file(path).unwrap();
        assert!(content.contains("line2modified"));
    }

    #[tokio::test]
    async fn test_apply_patch_file_not_exists() {
        let path = Path::new("/nonexistent/file");
        
        let result = apply_patch(path, "patch", true, None).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_patch_to_content() {
        let original = "line1\nline2\nline3";
        let patch = "--- a/file\n+++ b/file\n@@ -1,3 +1,3 @@\n line1\n-line2\n+line2modified\n line3";
        
        let result = apply_patch_to_content(original, patch).unwrap();
        assert!(result.contains("line2modified"));
        let lines: Vec<&str> = result.lines().collect();
        assert!(!lines.contains(&"line2"));
        assert!(lines.contains(&"line2modified"));
    }

    #[test]
    fn test_generate_diff() {
        let original = "line1\nline2\nline3";
        let modified = "line1\nline2modified\nline3";
        
        let diff = generate_diff(original, modified);
        
        assert!(diff.contains("--- original"));
        assert!(diff.contains("+++ modified"));
        assert!(diff.contains("-line2"));
        assert!(diff.contains("+line2modified"));
    }

    #[test]
    fn test_generate_diff_no_changes() {
        let content = "line1\nline2\nline3";
        let diff = generate_diff(content, content);
        
        assert!(diff.contains("--- original"));
        assert!(diff.contains("+++ modified"));
        assert!(!diff.contains("\n-line"));
        assert!(!diff.contains("\n+line"));
    }
}
