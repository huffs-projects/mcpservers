use crate::models::ApplyResult;
use crate::utils::{backup_file, atomic_write, generate_unified_diff, path_validation};
use serde::Deserialize;
use tokio::fs;

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

pub async fn handle_kitty_apply(req: ApplyRequest) -> ApplyResult {
    // Validate path for security
    let validated_path = match path_validation::validate_config_path(&req.config_path) {
        Ok(p) => p,
        Err(e) => {
            return ApplyResult {
                success: false,
                diff_applied: format!("Invalid config path: {}", e),
                backup_created: false,
            };
        }
    };
    
    let config_path_str = validated_path.to_str()
        .unwrap_or(&req.config_path);
    
    // Read current config
    let current_content = match fs::read_to_string(config_path_str).await {
        Ok(content) => content,
        Err(e) => {
            return ApplyResult {
                success: false,
                diff_applied: format!("Failed to read config: {}", e),
                backup_created: false,
            };
        }
    };

    // Generate new content (simple patch application - in production, use proper diff/patch)
    let new_content = apply_patch(&current_content, &req.patch);
    
    // Generate diff
    let diff = generate_unified_diff(
        &current_content,
        &new_content,
        config_path_str,
        config_path_str,
    );

    if req.dry_run {
        return ApplyResult {
            success: true,
            diff_applied: diff,
            backup_created: false,
        };
    }

    // Create backup
    let backup_created = match backup_file(config_path_str).await {
        Ok(_) => true,
        Err(e) => {
            return ApplyResult {
                success: false,
                diff_applied: format!("Failed to create backup: {}", e),
                backup_created: false,
            };
        }
    };

    // Apply changes
    match atomic_write(config_path_str, &new_content).await {
        Ok(_) => ApplyResult {
            success: true,
            diff_applied: diff,
            backup_created,
        },
        Err(e) => ApplyResult {
            success: false,
            diff_applied: format!("Failed to write config: {}", e),
            backup_created,
        },
    }
}

/// Apply a patch to the current config content
/// 
/// This is a simplified implementation. For production use, consider using
/// a proper diff/patch library like `similar` or `diffy` for better handling
/// of unified diff formats and conflict resolution.
fn apply_patch(current: &str, patch: &str) -> String {
    // Simple patch application - in production, use proper diff/patch library
    // For now, append the patch to the current content
    format!("{}\n\n# Applied patch:\n{}", current, patch)
}

