use serde_json::Value;

use crate::parser::muttrc::MuttrcParser;
use crate::error::McpResult;
use crate::utils::extract_string_param;

pub struct ConfigValidateHandler {
    parser: MuttrcParser,
}

impl ConfigValidateHandler {
    pub fn new() -> Self {
        Self {
            parser: MuttrcParser::new(),
        }
    }

    pub fn validate_config(&self, args: Option<&Value>) -> McpResult<Value> {
        let config = extract_string_param(args, "config")?;

        match self.parser.parse(&config) {
            Ok(commands) => {
                let mut issues: Vec<String> = Vec::new();
                let warnings: Vec<String> = Vec::new();

                // Check for common issues
                for cmd in &commands {
                    if let Some(ref opt) = cmd.option {
                        if !MuttrcParser::validate_option_name(opt) {
                            issues.push(format!(
                                "Line {}: Invalid option name: {}",
                                cmd.line_number, opt
                            ));
                        }
                    }
                }

                Ok(serde_json::json!({
                    "valid": issues.is_empty(),
                    "commands_parsed": commands.len(),
                    "issues": issues,
                    "warnings": warnings,
                    "summary": if issues.is_empty() {
                        "Configuration is valid".to_string()
                    } else {
                        format!("Found {} issue(s)", issues.len())
                    }
                }))
            }
            Err(e) => Ok(serde_json::json!({
                "valid": false,
                "error": format!("Parse error at line {}: {}", e.line, e.message),
                "issues": vec![format!("Line {}: {}", e.line, e.message)]
            })),
        }
    }

    pub fn check_options(&self, args: Option<&Value>) -> McpResult<Value> {
        let config = extract_string_param(args, "config")?;

        let options = self.parser.extract_options(&config);
        let mut checked = Vec::new();
        let mut unknown = Vec::new();

        // Common known options
        let known_options = vec![
            "real_name", "from", "imap_user", "imap_pass", "imap_server",
            "smtp_url", "smtp_pass", "folder", "spoolfile", "record",
            "postponed", "trash", "ssl_force_tls", "ssl_starttls",
            "mailboxes", "index_format", "pager_format", "sort", "editor",
            "pager", "mbox_type", "crypt_use_gpgme", "crypt_autosign",
        ];

        for (name, value) in &options {
            if known_options.contains(&name.as_str()) {
                checked.push(serde_json::json!({
                    "option": name,
                    "value": value,
                    "status": "known"
                }));
            } else {
                unknown.push(serde_json::json!({
                    "option": name,
                    "value": value,
                    "status": "unknown",
                    "note": "Option name not recognized - verify spelling"
                }));
            }
        }

        Ok(serde_json::json!({
            "total_options": options.len(),
            "known_options": checked.len(),
            "unknown_options": unknown.len(),
            "checked": checked,
            "unknown": unknown
        }))
    }

    pub fn lint_config(&self, args: Option<&Value>) -> McpResult<Value> {
        let config = extract_string_param(args, "config")?;

        let mut suggestions = Vec::new();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check for plain text passwords
        if config.contains("imap_pass") || config.contains("smtp_pass") {
            let has_encrypted = config.contains("gpg") || config.contains("pass ") || config.contains("`");
            if !has_encrypted {
                suggestions.push(serde_json::json!({
                    "type": "security",
                    "message": "Plain text passwords detected. Consider using encrypted password storage",
                    "suggestion": "set imap_pass = \"`gpg --batch -q --decrypt ~/.neomutt/pass.gpg`\""
                }));
            }
        }

        if config.contains("imap") && !config.contains("ssl_force_tls") && !config.contains("ssl_starttls") {
            suggestions.push(serde_json::json!({
                "type": "security",
                "message": "IMAP connection should use SSL/TLS",
                "suggestion": "set ssl_force_tls = yes"
            }));
        }

        if config.contains("smtp") && !config.contains("ssl_force_tls") && !config.contains("ssl_starttls") {
            suggestions.push(serde_json::json!({
                "type": "security",
                "message": "SMTP connection should use SSL/TLS",
                "suggestion": "set ssl_starttls = yes"
            }));
        }

        // Check for deprecated or incorrect syntax
        let lines: Vec<&str> = config.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();
            
            // Check for common typos
            if line.contains("realname") && !line.contains("real_name") {
                suggestions.push(serde_json::json!({
                    "type": "typo",
                    "line": line_num + 1,
                    "message": "Use 'real_name' instead of 'realname'",
                    "suggestion": line.replace("realname", "real_name")
                }));
            }

            // Check for unquoted strings with spaces
            if line.starts_with("set ") && line.contains(" = ") {
                let parts: Vec<&str> = line.split(" = ").collect();
                if parts.len() == 2 {
                    let value = parts[1].trim();
                    if value.contains(' ') && !value.starts_with('"') && !value.starts_with('\'') {
                        suggestions.push(serde_json::json!({
                            "type": "syntax",
                            "line": line_num + 1,
                            "message": "String values with spaces should be quoted",
                            "suggestion": format!("{} = \"{}\"", parts[0], value)
                        }));
                    }
                }
            }
        }

        // Validate syntax and check for conflicts
        let mut option_values: std::collections::HashMap<String, (String, usize)> = std::collections::HashMap::new();
        
        match self.parser.parse(&config) {
            Ok(commands) => {
                // Check for conflicting options and validate values
                for cmd in &commands {
                    if let (Some(ref opt), Some(ref val)) = (&cmd.option, &cmd.value) {
                        // Check for duplicate options
                        if let Some((existing_val, existing_line)) = option_values.get(opt) {
                            if existing_val != val {
                                warnings.push(serde_json::json!({
                                    "type": "conflict",
                                    "line": cmd.line_number,
                                    "message": format!("Option '{}' is set multiple times with different values", opt),
                                    "previous": format!("Line {}: {}", existing_line, existing_val),
                                    "current": format!("Line {}: {}", cmd.line_number, val)
                                }));
                            }
                        } else {
                            option_values.insert(opt.clone(), (val.clone(), cmd.line_number));
                        }
                        
                        // Validate option values
                        self.validate_option_value(opt, val, cmd.line_number, &mut errors, &mut suggestions);
                    }
                }
            }
            Err(e) => {
                errors.push(serde_json::json!({
                    "type": "syntax_error",
                    "line": e.line,
                    "message": e.message
                }));
            }
        }

        Ok(serde_json::json!({
            "errors": errors,
            "suggestions": suggestions,
            "warnings": warnings,
            "summary": format!("Found {} error(s), {} warning(s), and {} suggestion(s)", errors.len(), warnings.len(), suggestions.len())
        }))
    }
    
    fn validate_option_value(
        &self,
        option: &str,
        value: &str,
        line: usize,
        errors: &mut Vec<Value>,
        suggestions: &mut Vec<Value>,
    ) {
        // Validate URL formats for IMAP/SMTP
        if option == "folder" || option == "smtp_url" {
            if !value.starts_with("imap://") && !value.starts_with("smtp://") && !value.starts_with("maildir://") && !value.starts_with("+") && !value.starts_with("~") {
                errors.push(serde_json::json!({
                    "type": "invalid_value",
                    "line": line,
                    "option": option,
                    "message": format!("Invalid URL format for {}: {}", option, value),
                    "suggestion": if option == "folder" {
                        "Use format: imap://host:port or maildir://path"
                    } else {
                        "Use format: smtp://user@host:port/"
                    }
                }));
            }
        }
        
        // Validate boolean/quad options
        if option.contains("ssl_") || option == "crypt_autosign" || option == "crypt_autoencrypt" {
            let valid_values = ["yes", "no", "ask-yes", "ask-no"];
            if !valid_values.contains(&value.to_lowercase().as_str()) {
                errors.push(serde_json::json!({
                    "type": "invalid_value",
                    "line": line,
                    "option": option,
                    "message": format!("Invalid value for boolean option '{}': {}. Expected: yes, no, ask-yes, or ask-no", option, value),
                    "suggestion": format!("set {} = yes", option)
                }));
            }
        }
        
        // Validate port numbers
        if option.contains("_port") {
            if let Ok(port) = value.parse::<u16>() {
                if port == 0 || port > 65535 {
                    errors.push(serde_json::json!({
                        "type": "invalid_value",
                        "line": line,
                        "option": option,
                        "message": format!("Invalid port number: {}", port),
                        "suggestion": "Port must be between 1 and 65535"
                    }));
                }
            } else {
                errors.push(serde_json::json!({
                    "type": "invalid_value",
                    "line": line,
                    "option": option,
                    "message": format!("Port must be a number, got: {}", value),
                    "suggestion": "Use a valid port number (e.g., 993, 587)"
                }));
            }
        }
        
        // Check for deprecated options
        let deprecated = vec![
            ("realname", "real_name"),
            ("my_hdr", "my_hdr"),
        ];
        
        for (deprecated_name, replacement) in deprecated {
            if option == deprecated_name {
                suggestions.push(serde_json::json!({
                    "type": "deprecated",
                    "line": line,
                    "option": option,
                    "message": format!("Option '{}' is deprecated", deprecated_name),
                    "suggestion": format!("Use '{}' instead", replacement)
                }));
            }
        }
    }
}

