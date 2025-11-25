use crate::models::ValidationResult;
use crate::utils::{config_parser, css_parser};
use std::fs;
use std::path::Path;

/// Validate Wofi config and CSS files
pub fn validate(config_path: &Path, css_path: Option<&Path>) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut invalid_css = Vec::new();
    let mut invalid_options = Vec::new();
    let mut invalid_modes = Vec::new();

    // Validate config file
    if let Ok(content) = fs::read_to_string(config_path) {
        if let Ok(config) = config_parser::parse_config(&content) {
            // Check for invalid options
            let valid_options = vec![
                "width", "height", "location", "layer", "anchor",
                "insensitive", "fuzzy", "levenshtein", "prefix",
                "parse_action", "cache_file", "mode", "term",
            ];
            
            for key in config.keys() {
                if !valid_options.contains(&key.as_str()) {
                    invalid_options.push(key.clone());
                    warnings.push(format!("Unknown option: {}", key));
                }
            }

            // Validate mode
            if let Some(mode) = config.get("mode") {
                let valid_modes = vec!["drun", "run", "ssh", "dmenu", "custom"];
                if !valid_modes.contains(&mode.as_str()) {
                    invalid_modes.push(mode.clone());
                    errors.push(format!("Invalid mode: {}", mode));
                }
            }
        } else {
            errors.push("Failed to parse config file".to_string());
        }
    } else {
        errors.push(format!("Failed to read config file: {}", config_path.display()));
    }

    // Validate CSS file if provided
    if let Some(css_path) = css_path {
        if let Ok(content) = fs::read_to_string(css_path) {
            if css_parser::parse_css(&content).is_err() {
                invalid_css.push("CSS parsing failed".to_string());
                errors.push("Invalid CSS syntax".to_string());
            }
        } else {
            warnings.push(format!("CSS file not found: {}", css_path.display()));
        }
    }

    ValidationResult {
        success: errors.is_empty(),
        errors,
        warnings,
        invalid_css,
        invalid_options,
        invalid_modes,
    }
}

