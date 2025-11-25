use crate::models::ValidationResult;
use crate::utils::logger::Logger;
use crate::utils::parser::StarshipConfig;
use crate::utils::security::PathValidator;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub config_path: String,
}

pub struct ValidateEndpoint;

impl ValidateEndpoint {
    pub async fn execute(params: ValidateRequest) -> Result<ValidationResult> {
        let logger = Logger::new("starship_validate");
        logger.info(format!("Validating config: {}", params.config_path));

        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut logs = String::new();

        // Validate path format
        PathValidator::validate_path_format(&params.config_path)
            .map_err(|e| {
                errors.push(format!("Invalid path format: {}", e));
                e
            })?;

        // Validate and sanitize path
        let path_validator = PathValidator::default();
        let safe_path = path_validator
            .validate_path(&params.config_path)
            .map_err(|e| {
                errors.push(format!("Path validation failed: {}", e));
                e
            })?;

        // Read and parse the config file
        let contents = tokio::fs::read_to_string(&safe_path)
            .await
            .with_context(|| format!("Failed to read config file: {}", safe_path.display()))?;

        logs.push_str(&format!("Read config file: {} bytes\n", contents.len()));

        // Parse TOML
        let config = match StarshipConfig::from_str(&contents) {
            Ok(cfg) => {
                logs.push_str("✓ TOML syntax is valid\n");
                cfg
            }
            Err(e) => {
                errors.push(format!("TOML parse error: {}", e));
                logs.push_str(&format!("✗ TOML parse failed: {}\n", e));
                return Ok(ValidationResult {
                    success: false,
                    errors,
                    warnings,
                    logs,
                });
            }
        };

        // Validate structure using enhanced parser
        let structure_errors = config.validate_structure();
        let structure_error_count = structure_errors.len();
        errors.extend(structure_errors);
        if structure_error_count == 0 {
            logs.push_str("✓ Structure validation passed\n");
        } else {
            logs.push_str(&format!("✗ Structure validation found {} errors\n", structure_error_count));
        }

        // Check for common issues
        if config.modules.is_empty() {
            warnings.push("Configuration appears to be empty".to_string());
            logs.push_str("⚠ Configuration is empty\n");
        } else {
            logs.push_str(&format!("✓ Found {} configuration entries\n", config.modules.len()));
        }

        // Validate format field
        if let Some(format_val) = config.modules.get("format") {
            if format_val.is_str() {
                let format_str = format_val.as_str().unwrap_or("");
                if format_str.is_empty() {
                    warnings.push("'format' field is empty".to_string());
                    logs.push_str("⚠ Format field is empty\n");
                } else {
                    logs.push_str("✓ Format field is valid\n");
                }
            } else {
                errors.push("'format' field must be a string".to_string());
                logs.push_str("✗ Format field must be a string\n");
            }
        } else {
            warnings.push("No 'format' field specified - using default".to_string());
            logs.push_str("⚠ No format specified (will use default)\n");
        }

        // Validate timeout values
        if let Some(scan_timeout) = config.modules.get("scan_timeout") {
            if let Some(timeout) = scan_timeout.as_integer() {
                if timeout < 0 {
                    errors.push("'scan_timeout' must be a positive integer".to_string());
                    logs.push_str("✗ scan_timeout must be positive\n");
                } else {
                    logs.push_str(&format!("✓ scan_timeout: {}ms\n", timeout));
                }
            } else {
                errors.push("'scan_timeout' must be an integer".to_string());
                logs.push_str("✗ scan_timeout must be an integer\n");
            }
        }

        if let Some(cmd_timeout) = config.modules.get("command_timeout") {
            if let Some(timeout) = cmd_timeout.as_integer() {
                if timeout < 0 {
                    errors.push("'command_timeout' must be a positive integer".to_string());
                    logs.push_str("✗ command_timeout must be positive\n");
                } else {
                    logs.push_str(&format!("✓ command_timeout: {}ms\n", timeout));
                }
            } else {
                errors.push("'command_timeout' must be an integer".to_string());
                logs.push_str("✗ command_timeout must be an integer\n");
            }
        }

        // Validate boolean fields
        if let Some(add_newline) = config.modules.get("add_newline") {
            if !add_newline.is_bool() {
                errors.push("'add_newline' must be a boolean".to_string());
                logs.push_str("✗ add_newline must be a boolean\n");
            } else {
                logs.push_str("✓ add_newline is valid\n");
            }
        }

        // Validate module configurations
        let module_names = config.get_all_module_names();
        if !module_names.is_empty() {
            logs.push_str(&format!("✓ Found {} module(s): {}\n", 
                module_names.len(), 
                module_names.join(", ")));
        }

        // Check for disabled modules
        let disabled_modules: Vec<String> = config.modules
            .keys()
            .filter_map(|key| {
                if key.ends_with(".disabled") {
                    key.strip_suffix(".disabled").map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect();
        
        if !disabled_modules.is_empty() {
            logs.push_str(&format!("ℹ Disabled modules: {}\n", disabled_modules.join(", ")));
        }

        // Validate nested module structures
        for module_name in &module_names {
            if let Some(module_val) = config.get_module(module_name) {
                if let Some(table) = module_val.as_table() {
                    // Check for common module fields
                    for (field, value) in table {
                        match field.as_str() {
                            "format" | "style" | "symbol" => {
                                if !value.is_str() {
                                    warnings.push(format!("{}.{} should be a string", module_name, field));
                                }
                            }
                            "disabled" => {
                                if value.as_bool().is_none() {
                                    errors.push(format!("{}.disabled must be a boolean", module_name));
                                }
                            }
                            "truncation_length" => {
                                if !value.is_integer() {
                                    warnings.push(format!("{}.truncation_length should be an integer", module_name));
                                }
                            }
                            _ => {
                                // Unknown field - might be valid, just note it
                            }
                        }
                    }
                }
            }
        }

        // Check for potential issues
        if config.modules.contains_key("format") {
            let format_str = config.modules.get("format")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            // Check for common format string issues
            if format_str.contains("$all") && config.modules.len() > 10 {
                warnings.push("Using $all with many module configurations may cause performance issues".to_string());
                logs.push_str("⚠ Using $all with many modules\n");
            }
        }

        let success = errors.is_empty();
        
        if success {
            logs.push_str("✓ Validation passed\n");
        } else {
            logs.push_str(&format!("✗ Validation failed with {} error(s)\n", errors.len()));
        }

        logger.info(format!(
            "Validation complete: {} errors, {} warnings",
            errors.len(),
            warnings.len()
        ));

        Ok(ValidationResult {
            success,
            errors,
            warnings,
            logs,
        })
    }
}
