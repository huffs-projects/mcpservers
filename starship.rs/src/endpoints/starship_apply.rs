use crate::models::ApplyResult;
use crate::utils::file::FileManager;
use crate::utils::logger::Logger;
use crate::utils::parser::StarshipConfig;
use crate::utils::security::PathValidator;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApplyRequest {
    pub config_path: String,
    pub patch: String,
    #[serde(default = "default_dry_run")]
    pub dry_run: bool,
    pub backup_path: Option<String>,
}

fn default_dry_run() -> bool {
    true
}

pub struct ApplyEndpoint;

impl ApplyEndpoint {
    pub async fn execute(params: ApplyRequest) -> Result<ApplyResult> {
        let logger = Logger::new("starship_apply");
        logger.info(format!("Applying config changes to: {}", params.config_path));

        // Validate patch content
        crate::utils::validation::InputValidator::validate_patch(&params.patch)
            .context("Invalid patch content")?;

        // Validate path format
        PathValidator::validate_path_format(&params.config_path)
            .context("Invalid config path format")?;

        // Validate and sanitize config path
        let path_validator = PathValidator::default();
        let safe_config_path = path_validator
            .validate_path(&params.config_path)
            .context("Config path validation failed")?;

        // Validate backup path if provided
        let safe_backup_path = if let Some(ref backup_path) = params.backup_path {
            PathValidator::validate_path_format(backup_path)
                .context("Invalid backup path format")?;
            Some(
                path_validator
                    .validate_path(backup_path)
                    .context("Backup path validation failed")?,
            )
        } else {
            None
        };

        let file_manager = FileManager::new();

        // Read current config
        let current_contents = file_manager
            .read_config(&safe_config_path)
            .await
            .with_context(|| format!("Failed to read config: {}", safe_config_path.display()))?;

        // Parse current config
        let mut config = StarshipConfig::from_str(&current_contents)
            .context("Failed to parse current config")?;

        // Apply patch
        config
            .merge_patch(&params.patch)
            .context("Failed to merge patch")?;

        // Generate new config
        let new_contents = config.to_string().context("Failed to serialize new config")?;

        // Compute diff
        let diff = FileManager::compute_diff(&current_contents, &new_contents);

        if params.dry_run {
            logger.info("Dry-run mode: changes not applied");
            return Ok(ApplyResult {
                success: true,
                diff_applied: diff,
                backup_created: false,
            });
        }

        // Create backup
        let backup_path = file_manager
            .create_backup(&safe_config_path, safe_backup_path.as_deref())
            .await
            .context("Failed to create backup")?;

        logger.info(format!("Backup created: {}", backup_path.display()));

        // Write new config
        file_manager
            .write_config(&safe_config_path, &new_contents)
            .await
            .with_context(|| format!("Failed to write config: {}", safe_config_path.display()))?;

        logger.info("Configuration applied successfully");

        Ok(ApplyResult {
            success: true,
            diff_applied: diff,
            backup_created: true,
        })
    }
}

