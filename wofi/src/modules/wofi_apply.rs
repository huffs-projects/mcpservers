use crate::models::ApplyResult;
use crate::utils::{config_parser, css_parser, diff_utils, atomic_write, config_locator};
use std::fs;
use std::path::Path;
use anyhow::Result;

/// Apply patches to config and CSS files with atomic writes and backups
pub fn apply(
    config_path: &Path,
    css_path: Option<&Path>,
    patch_config: &str,
    patch_css: Option<&str>,
    dry_run: bool,
) -> Result<ApplyResult> {
    // Read existing files
    let old_config = fs::read_to_string(config_path)
        .unwrap_or_else(|_| String::new());
    
    let old_css = css_path
        .and_then(|p| fs::read_to_string(p).ok())
        .unwrap_or_else(|| String::new());

    // Generate diffs
    let diff_config = diff_utils::generate_diff(&old_config, patch_config, "config");
    let diff_css = patch_css.map(|new_css| {
        diff_utils::generate_diff(&old_css, new_css, "style.css")
    });

    if dry_run {
        return Ok(ApplyResult {
            success: true,
            diff_config,
            diff_css,
            backup_path: "dry-run".to_string(),
        });
    }

    // Create backup and apply changes
    let mut config_writer = atomic_write::AtomicWrite::new(config_path)?;
    config_writer.create_backup()?;
    config_writer.write(patch_config)?;
    let backup_path = config_writer.commit()?;

    // Apply CSS if provided
    if let (Some(css_path), Some(new_css)) = (css_path, patch_css) {
        let mut css_writer = atomic_write::AtomicWrite::new(css_path)?;
        css_writer.write(new_css)?;
        css_writer.commit()?;
    }

    Ok(ApplyResult {
        success: true,
        diff_config,
        diff_css,
        backup_path: backup_path.to_string_lossy().to_string(),
    })
}

