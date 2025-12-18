use serde_json::Value;

use crate::handlers::config_gen::ConfigGenHandler;
use crate::handlers::config_validate::ConfigValidateHandler;
use crate::error::{McpError, McpResult};
use crate::utils::{extract_string_param, extract_optional_string_param};

pub struct InteractiveHandler {
    config_gen: ConfigGenHandler,
    config_validate: ConfigValidateHandler,
}

impl InteractiveHandler {
    pub fn new() -> Self {
        Self {
            config_gen: ConfigGenHandler::new(),
            config_validate: ConfigValidateHandler::new(),
        }
    }

    pub fn setup_wizard(&self, args: Option<&Value>) -> McpResult<Value> {
        let step = extract_optional_string_param(args, "step").unwrap_or_else(|| "start".to_string());

        match step.as_str() {
            "start" => Ok(serde_json::json!({
                "step": "start",
                "message": "Welcome to the NeoMutt configuration wizard",
                "next_steps": [
                    {
                        "step": "basic_info",
                        "description": "Enter your name and email address",
                        "fields": ["real_name", "email"]
                    },
                    {
                        "step": "imap",
                        "description": "Configure IMAP server settings",
                        "fields": ["imap_server", "imap_port", "imap_user"]
                    },
                    {
                        "step": "smtp",
                        "description": "Configure SMTP server settings",
                        "fields": ["smtp_server", "smtp_port", "smtp_user"]
                    },
                    {
                        "step": "security",
                        "description": "Configure security and encryption",
                        "fields": ["use_ssl", "use_starttls", "gpg_key"]
                    },
                    {
                        "step": "finish",
                        "description": "Review and generate configuration"
                    }
                ]
            })),
            "basic_info" => Ok(serde_json::json!({
                "step": "basic_info",
                "message": "Enter your basic information",
                "fields": {
                    "real_name": {
                        "type": "string",
                        "description": "Your full name",
                        "required": true,
                        "example": "John Doe"
                    },
                    "email": {
                        "type": "string",
                        "description": "Your email address",
                        "required": true,
                        "example": "john@example.com"
                    }
                },
                "next_step": "imap"
            })),
            "imap" => Ok(serde_json::json!({
                "step": "imap",
                "message": "Configure your IMAP server",
                "fields": {
                    "imap_server": {
                        "type": "string",
                        "description": "IMAP server hostname",
                        "required": true,
                        "example": "imap.example.com"
                    },
                    "imap_port": {
                        "type": "number",
                        "description": "IMAP port (usually 993 for SSL, 143 for STARTTLS)",
                        "required": false,
                        "default": 993
                    },
                    "imap_user": {
                        "type": "string",
                        "description": "IMAP username (usually your email)",
                        "required": false,
                        "example": "john@example.com"
                    }
                },
                "next_step": "smtp"
            })),
            "smtp" => Ok(serde_json::json!({
                "step": "smtp",
                "message": "Configure your SMTP server",
                "fields": {
                    "smtp_server": {
                        "type": "string",
                        "description": "SMTP server hostname",
                        "required": true,
                        "example": "smtp.example.com"
                    },
                    "smtp_port": {
                        "type": "number",
                        "description": "SMTP port (usually 587 for STARTTLS, 465 for SSL)",
                        "required": false,
                        "default": 587
                    },
                    "smtp_user": {
                        "type": "string",
                        "description": "SMTP username (usually your email)",
                        "required": false,
                        "example": "john@example.com"
                    }
                },
                "next_step": "security"
            })),
            "security" => Ok(serde_json::json!({
                "step": "security",
                "message": "Configure security settings",
                "fields": {
                    "use_ssl": {
                        "type": "boolean",
                        "description": "Use SSL/TLS for connections",
                        "required": false,
                        "default": true
                    },
                    "use_starttls": {
                        "type": "boolean",
                        "description": "Use STARTTLS (for SMTP)",
                        "required": false,
                        "default": true
                    },
                    "gpg_key": {
                        "type": "string",
                        "description": "GPG key ID for encryption (optional)",
                        "required": false
                    }
                },
                "next_step": "finish"
            })),
            "finish" => Ok(serde_json::json!({
                "step": "finish",
                "message": "Configuration wizard complete! Use the generate_config tool with your collected information.",
                "summary": "You can now generate your configuration file using the collected settings."
            })),
            _ => Err(McpError::ParameterError {
                message: format!("Unknown wizard step: {}", step),
                parameter: Some("step".to_string()),
            }),
        }
    }

    pub fn suggest_config(&self, args: Option<&Value>) -> McpResult<Value> {
        let use_case = extract_string_param(args, "use_case")?;

        let use_case_lower = use_case.to_lowercase();
        let mut suggestions = Vec::new();
        // Pre-allocate capacity for config snippet
        let mut config_snippet = String::with_capacity(512);

        // Analyze use case and provide suggestions
        if use_case_lower.contains("gmail") || use_case_lower.contains("google") {
            suggestions.push("Gmail requires app-specific passwords");
            suggestions.push("Use IMAP server: imap.gmail.com:993");
            suggestions.push("Use SMTP server: smtp.gmail.com:587");
            config_snippet.push_str("# Gmail configuration\n");
            config_snippet.push_str("set folder = \"imap://imap.gmail.com:993\"\n");
            config_snippet.push_str("set imap_user = \"your.email@gmail.com\"\n");
            config_snippet.push_str("set smtp_url = \"smtp://your.email@gmail.com@smtp.gmail.com:587/\"\n");
            config_snippet.push_str("set ssl_force_tls = yes\n");
            config_snippet.push_str("set ssl_starttls = yes\n");
        }

        if use_case_lower.contains("multiple") || use_case_lower.contains("account") {
            suggestions.push("Use account-hook for multiple accounts");
            suggestions.push("Each account needs its own hook");
            config_snippet.push_str("# Multiple accounts\n");
            config_snippet.push_str("account-hook imap://host1/ 'set imap_user=user1 imap_pass=pass1'\n");
            config_snippet.push_str("account-hook imap://host2/ 'set imap_user=user2 imap_pass=pass2'\n");
        }

        if use_case_lower.contains("encrypt") || use_case_lower.contains("pgp") || use_case_lower.contains("gpg") {
            suggestions.push("Enable GPG encryption");
            suggestions.push("Configure crypt_autosign and crypt_autoencrypt");
            config_snippet.push_str("# Encryption\n");
            config_snippet.push_str("set crypt_use_gpgme = yes\n");
            config_snippet.push_str("set crypt_autosign = yes\n");
            config_snippet.push_str("set crypt_autoencrypt = yes\n");
        }

        if use_case_lower.contains("offline") || use_case_lower.contains("local") {
            suggestions.push("Use Maildir format for local storage");
            suggestions.push("Configure mbsync or isync for synchronization");
            config_snippet.push_str("# Local mail storage\n");
            config_snippet.push_str("set mbox_type = Maildir\n");
            config_snippet.push_str("set folder = \"~/Mail\"\n");
            config_snippet.push_str("set spoolfile = \"+INBOX\"\n");
        }

        if use_case_lower.contains("work") || use_case_lower.contains("enterprise") {
            suggestions.push("May require corporate certificate configuration");
            suggestions.push("Check with IT for server settings");
            suggestions.push("May need to disable certificate verification (not recommended)");
        }

        // Default suggestions if no specific use case matched
        if suggestions.is_empty() {
            suggestions.push("Start with basic IMAP/SMTP configuration");
            suggestions.push("Enable SSL/TLS for security");
            suggestions.push("Configure mailboxes and folders");
            config_snippet.push_str("# Basic configuration\n");
            config_snippet.push_str("set real_name = \"Your Name\"\n");
            config_snippet.push_str("set from = \"your.email@example.com\"\n");
            config_snippet.push_str("set folder = \"imap://imap.example.com:993\"\n");
            config_snippet.push_str("set ssl_force_tls = yes\n");
        }

        Ok(serde_json::json!({
            "use_case": use_case,
            "suggestions": suggestions,
            "config_snippet": config_snippet,
            "next_steps": [
                "Review the suggestions",
                "Customize the configuration snippet",
                "Use validate_config to check for errors",
                "Test the configuration with NeoMutt"
            ]
        }))
    }

    pub fn troubleshoot(&self, args: Option<&Value>) -> McpResult<Value> {
        let error = extract_string_param(args, "error")?;
        let config = extract_optional_string_param(args, "config");

        let error_lower = error.to_lowercase();
        let mut solutions = Vec::new();
        let mut checks = Vec::new();

        // Common error patterns
        if error_lower.contains("connection") || error_lower.contains("connect") {
            solutions.push("Check your IMAP/SMTP server addresses and ports");
            solutions.push("Verify SSL/TLS settings (ssl_force_tls or ssl_starttls)");
            solutions.push("Check firewall and network connectivity");
            checks.push("Verify server hostnames are correct");
            checks.push("Check if ports are blocked by firewall");
        }

        if error_lower.contains("authentication") || error_lower.contains("login") || error_lower.contains("password") {
            solutions.push("Verify username and password are correct");
            solutions.push("For Gmail, use app-specific passwords");
            solutions.push("Check if account requires OAuth2");
            checks.push("Verify imap_user and smtp_user settings");
            checks.push("Check password storage method");
        }

        if error_lower.contains("ssl") || error_lower.contains("tls") || error_lower.contains("certificate") {
            solutions.push("Verify SSL/TLS settings match your server");
            solutions.push("Check certificate validity");
            solutions.push("May need to set ssl_verify_host = no (not recommended)");
            checks.push("Verify ssl_force_tls or ssl_starttls is set correctly");
            checks.push("Check server certificate");
        }

        if error_lower.contains("parse") || error_lower.contains("syntax") {
            solutions.push("Check configuration file syntax");
            solutions.push("Verify all strings are properly quoted");
            solutions.push("Check for typos in option names");
            if let Some(ref cfg) = config {
                // Validate the config
                if let Ok(validation) = self.config_validate.validate_config(Some(&serde_json::json!({
                    "config": cfg
                }))) {
                    return Ok(serde_json::json!({
                        "error": error,
                        "solutions": solutions,
                        "validation": validation,
                        "checks": checks
                    }));
                }
            }
        }

        if error_lower.contains("folder") || error_lower.contains("mailbox") {
            solutions.push("Verify folder paths are correct");
            solutions.push("Check mailbox names (case-sensitive)");
            solutions.push("Ensure mailboxes exist on the server");
            checks.push("Verify 'folder' setting");
            checks.push("Check 'mailboxes' list");
        }

        // Generic troubleshooting
        if solutions.is_empty() {
            solutions.push("Check NeoMutt documentation for the specific error");
            solutions.push("Verify all required settings are configured");
            solutions.push("Test with a minimal configuration first");
        }

        let mut result = serde_json::json!({
            "error": error,
            "solutions": solutions,
            "checks": checks
        });

        // If config provided, validate it
        if let Some(ref cfg) = config {
            if let Ok(validation) = self.config_validate.validate_config(Some(&serde_json::json!({
                "config": cfg
            }))) {
                result["validation"] = validation;
            }
        }

        Ok(result)
    }
}

