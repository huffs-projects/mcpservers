use crate::core::model::ApplyResult;
use crate::core::patch::LuaPatch;
use crate::utils::diff::DiffGenerator;
use crate::utils::fs::AtomicFileOps;
use serde::Deserialize;
use std::path::Path;

/// Query parameters for nvim_apply endpoint
#[derive(Debug, Deserialize)]
pub struct ApplyQuery {
    pub file_path: String,
    pub patch: String,
    #[serde(default = "default_dry_run")]
    pub dry_run: bool,
}

fn default_dry_run() -> bool {
    true
}

/// Apply endpoint handler
pub struct ApplyEndpoint {
    patch_engine: LuaPatch,
}

impl ApplyEndpoint {
    pub fn new() -> Self {
        Self {
            patch_engine: LuaPatch::new(),
        }
    }

    /// Handle apply query
    pub async fn handle_query(&mut self, query: ApplyQuery) -> Result<ApplyResult, String> {
        let path = Path::new(&query.file_path);
        
        if !path.exists() {
            return Err(format!(
                "File does not exist: {} (absolute path: {})",
                query.file_path,
                path.canonicalize().unwrap_or_else(|_| path.to_path_buf()).display()
            ));
        }

        // Read original content
        let original = std::fs::read_to_string(path)
            .map_err(|e| format!(
                "Failed to read file {}: {} (error kind: {:?})",
                query.file_path,
                e,
                e.kind()
            ))?;

        // Validate patch
        let validation_result = self.patch_engine.validate_patch(&original, &query.patch);
        if let Err(diags) = validation_result {
            let error_count = diags.iter().filter(|d| matches!(d.severity, crate::core::diagnostics::DiagnosticSeverity::Error)).count();
            let warning_count = diags.len() - error_count;
            let warnings: Vec<String> = diags.iter().map(|d| {
                format!(
                    "[{}] {} (range: {:?})",
                    match d.severity {
                        crate::core::diagnostics::DiagnosticSeverity::Error => "ERROR",
                        crate::core::diagnostics::DiagnosticSeverity::Warning => "WARN",
                        _ => "INFO",
                    },
                    d.message,
                    d.range
                )
            }).collect();
            
            return Ok(ApplyResult {
                success: false,
                diff_applied: String::new(),
                backup_path: None,
                warnings: vec![format!(
                    "Patch validation failed: {} errors, {} warnings. File: {}, patch size: {} bytes",
                    error_count,
                    warning_count,
                    query.file_path,
                    query.patch.len()
                )].into_iter().chain(warnings).collect(),
            });
        }

        // Apply patch (simplified - would parse unified diff properly)
        let modified = DiffGenerator::apply_diff(&original, &query.patch)
            .map_err(|e| format!(
                "Failed to apply diff to {}: {}. Original file size: {} bytes, patch size: {} bytes",
                query.file_path,
                e,
                original.len(),
                query.patch.len()
            ))?;

        // Generate diff
        let diff = DiffGenerator::unified_diff(&original, &modified, &query.file_path, &query.file_path);

        if query.dry_run {
            return Ok(ApplyResult {
                success: true,
                diff_applied: diff,
                backup_path: None,
                warnings: vec!["Dry run - no changes applied".to_string()],
            });
        }

        // Apply changes atomically
        let backup_path = AtomicFileOps::write_with_backup(path, &modified)
            .map_err(|e| format!(
                "Failed to write file {}: {}. Backup creation attempted but failed",
                query.file_path,
                e
            ))?;

        Ok(ApplyResult {
            success: true,
            diff_applied: diff,
            backup_path: Some(backup_path.to_string_lossy().to_string()),
            warnings: Vec::new(),
        })
    }
}

impl Default for ApplyEndpoint {
    fn default() -> Self {
        Self::new()
    }
}

