use crate::models::ValidationResult;
use crate::utils::{WaybarParser, WaybarSchema};
use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;

// Compile dangerous pattern regexes once at startup
static DANGEROUS_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"rm\s+-rf").unwrap(),
        Regex::new(r"mkfs").unwrap(),
        Regex::new(r"dd\s+if=").unwrap(),
        Regex::new(r">\s+/dev/").unwrap(),
    ]
});

pub fn validate_config(config_path: &str, css_path: Option<&str>) -> Result<ValidationResult> {
    let mut result = ValidationResult::success();

    // Expand and validate config path
    let expanded_config = crate::utils::FileOps::validate_file_path(config_path)?;
    let config_path_str = expanded_config.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid path encoding"))?;

    // Validate JSON
    match WaybarParser::parse_json(config_path_str) {
        Ok(config) => {
            result.add_log(format!("Successfully parsed JSON: {}", config_path_str));
            validate_json_structure(&config, &mut result);
            validate_modules(&config, &mut result);
            validate_scripts(&config, &mut result);
        }
        Err(e) => {
            result.add_error(format!("Failed to parse JSON: {}", e));
            return Ok(result);
        }
    }

    // Validate CSS if provided
    if let Some(css) = css_path {
        let expanded_css = crate::utils::FileOps::validate_file_path(css)?;
        let css_path_str = expanded_css.to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path encoding"))?;
        
        match WaybarParser::parse_css(css_path_str) {
            Ok(css_content) => {
                result.add_log(format!("Successfully parsed CSS: {}", css_path_str));
                validate_css_syntax(&css_content, &mut result);
            }
            Err(e) => {
                result.add_error(format!("Failed to parse CSS: {}", e));
            }
        }
    }

    Ok(result)
}

fn validate_json_structure(config: &Value, result: &mut ValidationResult) {
    // Check for required top-level keys
    use crate::utils::MODULE_ARRAY_KEYS;
    let mut found_keys = HashSet::new();

    if let Some(obj) = config.as_object() {
        for key in obj.keys() {
            if MODULE_ARRAY_KEYS.contains(&key.as_str()) {
                found_keys.insert(key.as_str());
            }
        }
    }

    if found_keys.is_empty() {
        result.add_warning(format!(
            "No module arrays found. Expected at least one of: {}. Add modules to display in the bar.",
            MODULE_ARRAY_KEYS.join(", ")
        ));
    }
}

fn validate_modules(config: &Value, result: &mut ValidationResult) {
    let all_modules = WaybarSchema::get_all_modules();
    let modules = WaybarParser::extract_modules(config);

    for module in modules {
        if let Some(module_def) = config.get(&module) {
            if let Some(module_options) = all_modules.get(&module) {
                // Check required options
                for option in module_options {
                    if option.required {
                        if !module_def.get(&option.option_name).is_some() {
                            result.add_error(format!(
                                "Module '{}' is missing required option '{}'. {}",
                                module, option.option_name, option.description
                            ));
                            result.missing_required_keys.push(format!(
                                "{}.{}",
                                module, option.option_name
                            ));
                        }
                    }
                    
                    // Validate option types if present
                    if let Some(value) = module_def.get(&option.option_name) {
                        validate_option_type(&module, &option.option_name, value, &option.option_type, result);
                        
                        // Validate specific option values
                        if option.option_name == "interval" {
                            if let Some(interval) = value.as_u64() {
                                if interval == 0 {
                                    result.add_error(format!(
                                        "Module '{}' has invalid interval value: {}. Interval must be greater than 0.",
                                        module, interval
                                    ));
                                } else if interval > 3600 {
                                    result.add_warning(format!(
                                        "Module '{}' has a very large interval: {} seconds. Consider using a smaller value for better responsiveness.",
                                        module, interval
                                    ));
                                }
                            }
                        }
                    }
                }
            } else if !module.starts_with("custom/") && !module.starts_with("exec/") {
                result.add_warning(format!(
                    "Unknown module: '{}'. This may be a custom module or a typo. Custom modules should be prefixed with 'custom/' or 'exec/'.",
                    module
                ));
            }
        } else {
            result.add_warning(format!(
                "Module '{}' is referenced in modules array but has no configuration block.",
                module
            ));
        }
    }
}

fn validate_option_type(
    module: &str,
    option_name: &str,
    value: &Value,
    expected_type: &str,
    result: &mut ValidationResult,
) {
    let type_matches = match expected_type {
        "string" => value.is_string(),
        "integer" => value.is_number() && value.as_u64().is_some(),
        "boolean" => value.is_boolean(),
        "array" => value.is_array(),
        "object" => value.is_object(),
        _ => true, // Unknown type, skip validation
    };
    
    if !type_matches {
        result.add_error(format!(
            "Module '{}' option '{}' has incorrect type. Expected '{}', but got '{}'.",
            module,
            option_name,
            expected_type,
            match value {
                Value::String(_) => "string",
                Value::Number(_) => "number",
                Value::Bool(_) => "boolean",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
                Value::Null => "null",
            }
        ));
    }
}

fn validate_scripts(config: &Value, result: &mut ValidationResult) {
    let scripts = WaybarParser::extract_custom_scripts(config);

    for (name, command) in scripts {
        // Basic command validation
        if command.trim().is_empty() {
            result.add_error(format!("Empty command in module: {}", name));
            result.invalid_script_commands.push(name.clone());
            continue;
        }

        // Check for potentially dangerous commands using pre-compiled regexes
        for re in DANGEROUS_PATTERNS.iter() {
            if re.is_match(&command) {
                result.add_warning(format!(
                    "Potentially dangerous command in module '{}': {}",
                    name, command
                ));
            }
        }

        // Validate interval if present
        if let Some(module_def) = config.get(&name) {
            if let Some(interval) = module_def.get("interval") {
                if let Some(interval_val) = interval.as_u64() {
                    if interval_val == 0 {
                        result.add_error(format!(
                            "Invalid interval (0) in module: {}",
                            name
                        ));
                    }
                }
            }
        }
    }
}

fn validate_css_syntax(css: &str, result: &mut ValidationResult) {
    // Enhanced CSS validation with line numbers
    let mut brace_count = 0;
    let mut in_comment = false;
    let mut last_char = '\0';
    let mut line_num = 1;
    let mut char_pos = 0;
    let mut open_brace_line = Vec::new();

    for ch in css.chars() {
        char_pos += 1;
        if ch == '\n' {
            line_num += 1;
            char_pos = 0;
        }

        if in_comment {
            if last_char == '*' && ch == '/' {
                in_comment = false;
            }
            last_char = ch;
            continue;
        }

        if last_char == '/' && ch == '*' {
            in_comment = true;
            last_char = '\0';
            continue;
        }

        match ch {
            '{' => {
                brace_count += 1;
                open_brace_line.push(line_num);
            }
            '}' => {
                brace_count -= 1;
                if brace_count < 0 {
                    result.add_error(format!(
                        "Unmatched closing brace '}}' at line {}. Check for extra closing braces.",
                        line_num
                    ));
                    return;
                }
                open_brace_line.pop();
            }
            _ => {}
        }

        last_char = ch;
    }

    if brace_count != 0 {
        if let Some(open_line) = open_brace_line.first() {
            result.add_error(format!(
                "Unmatched opening brace '{{' at line {}. Missing {} closing brace(s).",
                open_line, brace_count
            ));
        } else {
            result.add_error(format!(
                "Unmatched braces in CSS. Missing {} closing brace(s).",
                brace_count
            ));
        }
    }

    // Check for common Waybar-specific selectors
    let waybar_selectors = vec![
        "window#waybar", "#waybar", "#battery", "#cpu", "#memory",
        "#network", "#clock", "#tray", "#workspaces", "#custom-",
    ];
    
    let mut found_waybar_selectors = false;
    for selector in &waybar_selectors {
        if css.contains(selector) {
            found_waybar_selectors = true;
            break;
        }
    }
    
    if !found_waybar_selectors {
        result.add_warning(
            "No Waybar-specific selectors found. Consider using selectors like '#battery', '#cpu', etc. for module styling.".to_string()
        );
    }

    // Enhanced property validation with value checking
    let valid_properties: Vec<&str> = vec![
        "background-color", "color", "padding", "margin", "border",
        "border-radius", "font-family", "font-size", "min-height",
        "max-height", "height", "width", "spacing", "opacity",
    ];

    let mut line_num_prop = 1;
    for line in css.lines() {
        if line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let prop = parts[0].trim();
                let value = parts[1].split(';').next().unwrap_or("").trim();
                
                if !prop.is_empty() {
                    // Check for common typos
                    if prop == "background" && !value.starts_with("url(") {
                        result.add_warning(format!(
                            "Line {}: Consider using 'background-color' instead of 'background' for solid colors.",
                            line_num_prop
                        ));
                    }
                    
                    // Validate numeric values
                    if prop == "font-size" || prop == "padding" || prop == "margin" {
                        if value.parse::<f64>().is_err() && !value.ends_with("px") && !value.ends_with("em") && !value.ends_with("rem") && !value.ends_with("%") {
                            result.add_warning(format!(
                                "Line {}: Property '{}' has unusual value '{}'. Expected a number with unit (e.g., '12px', '1em').",
                                line_num_prop, prop, value
                            ));
                        }
                    }
                    
                    if !valid_properties.contains(&prop) && !prop.starts_with("#") && !prop.contains("button") {
                        // Not an error, but warn about potentially invalid properties
                        if !result.warnings.iter().any(|w| w.contains(prop)) {
                            result.add_warning(format!(
                                "Line {}: Unknown CSS property '{}' (may still be valid CSS). Common Waybar properties include: {}",
                                line_num_prop,
                                prop,
                                valid_properties.iter().take(5).map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(", ")
                            ));
                        }
                    }
                }
            }
        }
        line_num_prop += 1;
    }
    
    // Suppress unused variable warning - char_pos is kept for potential future use
    let _ = char_pos;
}

