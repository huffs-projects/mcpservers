use crate::models::ApplyResult;
use crate::utils::diff;
use crate::utils::file_ops;
use anyhow::{Context, Result};

pub fn apply_patch(
    config_path: &str,
    patch: &str,
    dry_run: bool,
    backup_path: Option<&str>,
) -> Result<ApplyResult> {
    let expanded_path = file_ops::expand_path(config_path)?;
    let path = expanded_path.as_path();
    
    let expanded_backup_path = backup_path.map(|p| file_ops::expand_path(p)).transpose()?;
    let backup_path_ref = expanded_backup_path.as_deref();
    
    if !file_ops::file_exists(path) {
        return Err(anyhow::anyhow!("Config file does not exist: {}", config_path));
    }
    
    diff::validate_patch(patch)
        .with_context(|| "Invalid patch format")?;
    
    let original_content = file_ops::read_config_file(path)?;
    let new_content = diff::apply_patch(&original_content, patch)?;
    
    let diff_applied = diff::compute_unified_diff(&original_content, &new_content);
    
    if dry_run {
        tracing::info!("Dry run - patch would be applied to {}", config_path);
        return Ok(ApplyResult {
            success: true,
            diff_applied,
            backup_created: false,
        });
    }
    
    let backup_created = if let Some(backup_dir) = backup_path_ref {
        let backup = file_ops::create_backup(path, Some(backup_dir))?;
        tracing::info!("Backup created at: {}", backup.display());
        true
    } else {
        let backup = file_ops::create_backup(path, None)?;
        tracing::info!("Backup created at: {}", backup.display());
        true
    };
    
    file_ops::atomic_write(path, &new_content)
        .with_context(|| format!("Failed to apply patch to {}", config_path))?;
    
    tracing::info!("Patch successfully applied to {}", config_path);
    
    Ok(ApplyResult {
        success: true,
        diff_applied,
        backup_created,
    })
}

