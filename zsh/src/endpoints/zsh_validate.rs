use crate::models::ValidationResult;
use crate::utils::parser;
use crate::utils::file_ops;
use anyhow::Result;

pub fn validate_config(config_path: &str) -> Result<ValidationResult> {
    let expanded_path = file_ops::expand_path(config_path)?;
    let path = expanded_path.as_path();
    
    if !file_ops::file_exists(path) {
        return Ok(ValidationResult {
            success: false,
            errors: vec![format!("Config file does not exist: {}", config_path)],
            warnings: vec![],
            logs: format!("Attempted to validate non-existent file: {}", config_path),
        });
    }
    
    let content = file_ops::read_config_file(path)?;
    
    let syntax_errors = parser::validate_syntax(&content)?;
    
    let mut warnings = Vec::new();
    
    if content.contains("$_") {
        warnings.push("Use of $_ variable detected - ensure it's intentional".to_string());
    }
    
    if content.contains("rm *") && !content.contains("RM_STAR_SILENT") {
        warnings.push("Consider setting RM_STAR_SILENT or RM_STAR_WAIT for safety".to_string());
    }
    
    let success = syntax_errors.is_empty();
    
    let logs = if success {
        format!("Validation successful for {}", config_path)
    } else {
        format!("Validation found {} error(s) in {}", syntax_errors.len(), config_path)
    };
    
    Ok(ValidationResult {
        success,
        errors: syntax_errors,
        warnings,
        logs,
    })
}

