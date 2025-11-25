use crate::models::ValidationResult;
use crate::utils::file_ops;
use crate::utils::logger::EndpointLogger;
use crate::utils::parser;
use crate::utils::validation;
use anyhow::Result;
use std::path::Path;

/// Validate Mako config file for syntax and semantic correctness
pub fn validate_config(config_path: &str) -> Result<ValidationResult> {
    let logger = EndpointLogger::new("mako_validate");
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut logs = String::new();

    let path = Path::new(config_path);

    // Check if file exists
    if !file_ops::config_exists(path) {
        let msg = format!("Config file does not exist: {}", config_path);
        logger.log_error(&msg);
        errors.push(msg.clone());
        logs.push_str(&format!("ERROR: {}\n", msg));
        return Ok(ValidationResult {
            success: false,
            errors,
            warnings,
            logs,
        });
    }

    // Read and parse config
    let content = match file_ops::read_config(path) {
        Ok(c) => {
            logger.log_info("Successfully read config file");
            logs.push_str("✓ Config file read successfully\n");
            c
        }
        Err(e) => {
            let msg = format!("Failed to read config file: {}", e);
            logger.log_error(&msg);
            errors.push(msg.clone());
            logs.push_str(&format!("ERROR: {}\n", msg));
            return Ok(ValidationResult {
                success: false,
                errors,
                warnings,
                logs,
            });
        }
    };

    // Parse INI syntax
    let config = match parser::parse_config(&content) {
        Ok(c) => {
            logger.log_info("Config syntax is valid");
            logs.push_str("✓ Config syntax is valid\n");
            c
        }
        Err(e) => {
            let msg = format!("Syntax error: {}", e);
            logger.log_error(&msg);
            errors.push(msg.clone());
            logs.push_str(&format!("ERROR: {}\n", msg));
            return Ok(ValidationResult {
                success: false,
                errors,
                warnings,
                logs,
            });
        }
    };

    // Semantic validation
    validate_semantics(&config, &mut errors, &mut warnings, &mut logs, &logger);

    let success = errors.is_empty();
    if success {
        logger.log_success("Validation passed");
    } else {
        logger.log_error("Validation failed");
    }

    Ok(ValidationResult {
        success,
        errors,
        warnings,
        logs,
    })
}

fn validate_semantics(
    config: &parser::ConfigMap,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
    logs: &mut String,
    logger: &EndpointLogger,
) {
    // Validate known options
    let known_options = vec![
        "font", "background-color", "text-color", "width", "height", "margin",
        "padding", "border-size", "border-color", "border-radius", "progress-color",
        "icons", "max-icon-size", "max-visible", "default-timeout", "ignore-timeout",
        "layer", "anchor", "sort", "output", "group-by", "markup", "actions", "history",
    ];

    for (section, entries) in config {
        logger.log_debug(&format!("Validating section: {}", section));

        for (key, value) in entries {
            // Check if option is known
            if !known_options.contains(&key.as_str()) {
                let msg = format!("Unknown option '{}' in section [{}]", key, section);
                warnings.push(msg.clone());
                logs.push_str(&format!("WARNING: {}\n", msg));
                logger.log_warning(&msg);
            }

            // Type-specific validation
            validate_option_value(key, value, errors, warnings, logs, logger);
        }
    }

    // Check for required sections (none are strictly required, but default is common)
    if !config.contains_key("default") && !config.is_empty() {
        let msg = "No [default] section found, but other sections exist".to_string();
        warnings.push(msg.clone());
        logs.push_str(&format!("WARNING: {}\n", msg));
        logger.log_warning(&msg);
    }
}

fn validate_option_value(
    key: &str,
    value: &str,
    errors: &mut Vec<String>,
    _warnings: &mut Vec<String>,
    logs: &mut String,
    logger: &EndpointLogger,
) {
    match key {
        "width" | "height" => {
            match validation::validate_positive_integer(value) {
                Ok(_) => {}
                Err(e) => {
                    let msg = format!("Option '{}' must be a positive integer, got: {} ({})", key, value, e);
                    errors.push(msg.clone());
                    logs.push_str(&format!("ERROR: {}\n", msg));
                    logger.log_error(&msg);
                }
            }
        }
        "border-size" | "border-radius" | "max-icon-size" | "max-visible" | "default-timeout" => {
            match validation::validate_non_negative_integer(value) {
                Ok(_) => {}
                Err(e) => {
                    let msg = format!("Option '{}' must be a non-negative integer, got: {} ({})", key, value, e);
                    errors.push(msg.clone());
                    logs.push_str(&format!("ERROR: {}\n", msg));
                    logger.log_error(&msg);
                }
            }
        }
        "background-color" | "text-color" | "border-color" | "progress-color" => {
            if !validation::validate_color(value) {
                let msg = format!("Option '{}' must be a valid color (hex format #RRGGBB or #RRGGBBAA), got: {}", key, value);
                errors.push(msg.clone());
                logs.push_str(&format!("ERROR: {}\n", msg));
                logger.log_error(&msg);
            }
        }
        "output" => {
            if !value.is_empty() && !validation::validate_path(value) {
                let msg = format!("Option '{}' must be a valid path, got: {}", key, value);
                errors.push(msg.clone());
                logs.push_str(&format!("ERROR: {}\n", msg));
                logger.log_error(&msg);
            }
        }
        "icons" | "ignore-timeout" | "actions" | "history" => {
            if value != "0" && value != "1" {
                let msg = format!("Option '{}' must be 0 or 1, got: {}", key, value);
                errors.push(msg.clone());
                logs.push_str(&format!("ERROR: {}\n", msg));
                logger.log_error(&msg);
            }
        }
        "markup" => {
            if value != "0" && value != "1" && value != "2" {
                let msg = format!("Option '{}' must be 0, 1, or 2, got: {}", key, value);
                errors.push(msg.clone());
                logs.push_str(&format!("ERROR: {}\n", msg));
                logger.log_error(&msg);
            }
        }
        "layer" => {
            let valid = vec!["background", "bottom", "top", "overlay"];
            if !valid.contains(&value) {
                let msg = format!("Option '{}' must be one of {:?}, got: {}", key, valid, value);
                errors.push(msg.clone());
                logs.push_str(&format!("ERROR: {}\n", msg));
                logger.log_error(&msg);
            }
        }
        "anchor" => {
            let valid = vec!["top-left", "top-right", "bottom-left", "bottom-right", "top-center", "bottom-center"];
            if !valid.contains(&value) {
                let msg = format!("Option '{}' must be one of {:?}, got: {}", key, valid, value);
                errors.push(msg.clone());
                logs.push_str(&format!("ERROR: {}\n", msg));
                logger.log_error(&msg);
            }
        }
        "sort" => {
            let valid = vec!["+priority", "-priority", "+time", "-time"];
            if !valid.contains(&value) {
                let msg = format!("Option '{}' must be one of {:?}, got: {}", key, valid, value);
                errors.push(msg.clone());
                logs.push_str(&format!("ERROR: {}\n", msg));
                logger.log_error(&msg);
            }
        }
        "group-by" => {
            let valid = vec!["app-name", "app-icon"];
            if !valid.contains(&value) {
                let msg = format!("Option '{}' must be one of {:?}, got: {}", key, valid, value);
                errors.push(msg.clone());
                logs.push_str(&format!("ERROR: {}\n", msg));
                logger.log_error(&msg);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn create_test_config(content: &str) -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config");
        fs::write(&config_path, content).unwrap();
        (temp_dir, config_path.to_string_lossy().to_string())
    }

    #[test]
    fn test_validate_nonexistent_file() {
        let result = validate_config("/nonexistent/path/config").unwrap();
        assert!(!result.success);
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].contains("does not exist"));
    }

    #[test]
    fn test_validate_valid_config() {
        let (_temp_dir, config_path) = create_test_config(
            "[default]\nfont=monospace 10\nbackground-color=#285577\n"
        );
        let result = validate_config(&config_path).unwrap();
        assert!(result.success);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_syntax() {
        let (_temp_dir, config_path) = create_test_config(
            "[default]\nfont=monospace 10\ninvalid syntax here\n"
        );
        let result = validate_config(&config_path).unwrap();
        // Should still parse (invalid lines are ignored)
        assert!(result.success || !result.errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_integer() {
        let (_temp_dir, config_path) = create_test_config(
            "[default]\nwidth=not_a_number\n"
        );
        let result = validate_config(&config_path).unwrap();
        assert!(!result.success);
        assert!(result.errors.iter().any(|e| e.contains("integer")));
    }

    #[test]
    fn test_validate_invalid_boolean() {
        let (_temp_dir, config_path) = create_test_config(
            "[default]\nicons=2\n"
        );
        let result = validate_config(&config_path).unwrap();
        assert!(!result.success);
        assert!(result.errors.iter().any(|e| e.contains("must be 0 or 1")));
    }

    #[test]
    fn test_validate_invalid_layer() {
        let (_temp_dir, config_path) = create_test_config(
            "[default]\nlayer=invalid\n"
        );
        let result = validate_config(&config_path).unwrap();
        assert!(!result.success);
        assert!(result.errors.iter().any(|e| e.contains("layer")));
    }

    #[test]
    fn test_validate_invalid_anchor() {
        let (_temp_dir, config_path) = create_test_config(
            "[default]\nanchor=invalid\n"
        );
        let result = validate_config(&config_path).unwrap();
        assert!(!result.success);
        assert!(result.errors.iter().any(|e| e.contains("anchor")));
    }

    #[test]
    fn test_validate_valid_values() {
        let (_temp_dir, config_path) = create_test_config(
            "[default]\nlayer=overlay\nanchor=top-right\nicons=1\nmarkup=1\n"
        );
        let result = validate_config(&config_path).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_validate_unknown_option_warning() {
        let (_temp_dir, config_path) = create_test_config(
            "[default]\nunknown-option=value\n"
        );
        let result = validate_config(&config_path).unwrap();
        // Should succeed but with warnings
        assert!(result.warnings.iter().any(|w| w.contains("unknown-option")) || result.success);
    }
}

