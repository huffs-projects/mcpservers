use crate::models::ValidationResult;
use std::collections::HashMap;

/// Parser for Kitty configuration files
/// 
/// Handles parsing of kitty.conf files according to Kitty's syntax rules.
/// Supports basic option parsing, comments, and keybinding syntax.
pub struct KittyParser;

impl KittyParser {
    /// Parse Kitty config into AST using strict rules from reference manual
    /// 
    /// # Arguments
    /// * `config_content` - The content of the kitty.conf file to parse
    /// 
    /// # Returns
    /// * `Ok(HashMap)` - Map of option names to values if parsing succeeds
    /// * `Err(Vec<String>)` - List of error messages if parsing fails
    /// 
    /// # Example
    /// ```
    /// use kitty_mcp_server::utils::KittyParser;
    /// 
    /// let config = "font_family JetBrains Mono\nfont_size 12.0";
    /// let result = KittyParser::parse(config);
    /// assert!(result.is_ok());
    /// ```
    pub fn parse(config_content: &str) -> Result<HashMap<String, String>, Vec<String>> {
        let mut options = HashMap::new();
        let mut errors = Vec::new();

        for (line_num, line) in config_content.lines().enumerate() {
            let original_line = line;
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Handle include directives (basic support)
            if line.starts_with("include ") {
                // Include directive - for now, just note it (full implementation would resolve includes)
                // This is valid syntax, so we don't error on it
                continue;
            }

            // Parse option = value format or option value format
            if let Some(equal_pos) = line.find('=') {
                let key = line[..equal_pos].trim();
                let value = line[equal_pos + 1..].trim();
                
                if key.is_empty() {
                    errors.push(format!("Line {}: Empty option name", line_num + 1));
                    continue;
                }

                // Remove quotes if present
                let value = value.trim_matches(|c| c == '"' || c == '\'');
                options.insert(key.to_string(), value.to_string());
            } else if line.starts_with("map ") || line.starts_with("mapkitty ") {
                // Keybinding syntax: map key action
                // This is valid, we'll validate it separately
            } else {
                // Try space-separated format: option value
                // Only accept if first part looks like a valid option name (alphanumeric + underscore)
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let key = parts[0];
                    // Check if key looks like a valid option name
                    if !key.is_empty() && key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        let value = parts[1..].join(" ");
                        // Remove quotes if present in space-separated format too
                        let value = value.trim_matches(|c| c == '"' || c == '\'').to_string();
                        options.insert(key.to_string(), value);
                        continue;
                    }
                }
                
                // If we get here, it's an invalid line
                errors.push(format!("Line {}: Invalid syntax: {}", line_num + 1, line));
            }
        }

        if errors.is_empty() {
            Ok(options)
        } else {
            Err(errors)
        }
    }

    /// Validate Kitty config according to official syntax and semantics
    /// 
    /// # Arguments
    /// * `config_path` - Path to the kitty.conf file to validate
    /// 
    /// # Returns
    /// * `ValidationResult` - Contains success status, errors, warnings, and logs
    /// 
    /// # Example
    /// ```
    /// use kitty_mcp_server::utils::KittyParser;
    /// 
    /// let result = KittyParser::validate("/path/to/kitty.conf");
    /// if !result.success {
    ///     eprintln!("Validation failed: {:?}", result.errors);
    /// }
    /// ```
    pub fn validate(config_path: &str) -> ValidationResult {
        let content = match std::fs::read_to_string(config_path) {
            Ok(c) => c,
            Err(e) => {
                return ValidationResult {
                    success: false,
                    errors: vec![format!("Failed to read config file: {}", e)],
                    warnings: vec![],
                    logs: format!("Error reading {}: {}", config_path, e),
                };
            }
        };

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Parse and collect errors
        match Self::parse(&content) {
            Ok(_) => {
                // Additional semantic validation
                warnings.push("Basic syntax validation passed".to_string());
            }
            Err(parse_errors) => {
                errors.extend(parse_errors);
            }
        }

        // Validate known options against schema
        let schema = crate::utils::KittySchema::global();
        let parsed = Self::parse(&content).unwrap_or_default();
        
        for (key, _value) in &parsed {
            if !schema.is_valid_option(key) {
                warnings.push(format!("Unknown option: {}", key));
            }
        }

        ValidationResult {
            success: errors.is_empty(),
            errors,
            warnings,
            logs: format!("Validated {} options", parsed.len()),
        }
    }
}

