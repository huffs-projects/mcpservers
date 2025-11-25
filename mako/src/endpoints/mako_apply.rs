use crate::models::ApplyResult;
use crate::utils::diff;
use crate::utils::file_ops;
use crate::utils::logger::EndpointLogger;
use crate::utils::parser;
use crate::endpoints::mako_validate;
use anyhow::{Context, Result};
use std::path::Path;
use tempfile::NamedTempFile;

/// Apply patch to Mako configuration safely, with dry-run and backup
///
/// This function applies a configuration patch to a Mako config file with
/// the following safety features:
/// - Validates the new configuration before applying
/// - Creates automatic backups (unless dry-run)
/// - Atomic writes to prevent corruption
/// - Dry-run mode to preview changes
///
/// # Arguments
///
/// * `config_path` - Path to the Mako config file
/// * `patch` - INI-format patch to apply
/// * `dry_run` - If true, preview changes without applying
/// * `backup_path` - Optional custom backup path
///
/// # Returns
///
/// `ApplyResult` containing success status, diff, and backup information
///
/// # Errors
///
/// Returns an error if:
/// - Config file cannot be read or written
/// - Patch cannot be parsed
/// - New configuration fails validation
/// - File operations fail
pub fn apply_patch(
    config_path: &str,
    patch: &str,
    dry_run: bool,
    backup_path: Option<&str>,
) -> Result<ApplyResult> {
    let logger = EndpointLogger::new("mako_apply");
    let path = Path::new(config_path);

    // Ensure parent directory exists
    file_ops::ensure_parent_dir(path)?;

    // Read existing config or create empty
    let old_config = if file_ops::config_exists(path) {
        let content = file_ops::read_config(path)?;
        parser::parse_config(&content)
            .with_context(|| "Failed to parse existing config")?
    } else {
        logger.log_info("Config file does not exist, creating new one");
        parser::ConfigMap::new()
    };

    // Parse patch
    let patch_config = parser::parse_config(patch)
        .with_context(|| "Failed to parse patch")?;

    // Merge patch into existing config
    let mut new_config = old_config.clone();
    for (section, entries) in patch_config {
        let section_entries = new_config.entry(section).or_insert_with(Default::default);
        for (key, value) in entries {
            section_entries.insert(key, value);
        }
    }

    // Generate diff
    let diff_output = diff::generate_diff(&old_config, &new_config);
    logger.log_info(&format!("Generated diff:\n{}", diff_output));

    // Validate new config before applying
    let new_content = parser::serialize_config(&new_config);
    let temp_file = NamedTempFile::new()
        .with_context(|| "Failed to create temporary file for validation")?;
    std::fs::write(temp_file.path(), &new_content)
        .with_context(|| "Failed to write temporary config for validation")?;
    
    let temp_path_str = temp_file.path().to_string_lossy().to_string();
    match mako_validate::validate_config(&temp_path_str) {
        Ok(validation_result) => {
            if !validation_result.success {
                logger.log_error("New config validation failed");
                return Err(anyhow::anyhow!(
                    "Cannot apply patch: new configuration is invalid. Errors: {}\nLogs:\n{}",
                    validation_result.errors.join(", "),
                    validation_result.logs
                ));
            }
            logger.log_info("New config validation passed");
        }
        Err(e) => {
            logger.log_warning(&format!("Validation check failed: {}, proceeding anyway", e));
        }
    }

    if dry_run {
        logger.log_info("Dry-run mode: not applying changes");
        return Ok(ApplyResult {
            success: true,
            diff_applied: diff_output,
            backup_created: false,
        });
    }

    // Create backup
    let backup_created = if file_ops::config_exists(path) {
        let backup = backup_path.map(Path::new);
        match file_ops::create_backup(path, backup) {
            Ok(backup_path) => {
                logger.log_info(&format!("Created backup: {}", backup_path.display()));
                true
            }
            Err(e) => {
                logger.log_warning(&format!("Failed to create backup: {}", e));
                false
            }
        }
    } else {
        false
    };

    // Write new config
    file_ops::write_config_atomic(path, &new_content)
        .with_context(|| "Failed to write config file")?;

    logger.log_success("Patch applied successfully");

    Ok(ApplyResult {
        success: true,
        diff_applied: diff_output,
        backup_created,
    })
}

