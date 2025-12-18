use serde_json::Value;

use crate::models::config::EmailAccount;
use crate::error::{McpError, McpResult};
use crate::utils::{
    extract_string_param, extract_optional_string_param, extract_optional_number_param,
    extract_optional_bool_param, validate_email, validate_hostname, validate_port,
};

pub struct ConfigGenHandler;

impl ConfigGenHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_config(&self, args: Option<&Value>) -> McpResult<Value> {
        let requirements = extract_string_param(args, "requirements")?;

        // Generate a basic configuration based on requirements
        // Pre-allocate capacity for better performance
        let mut config = String::with_capacity(1024);
        
        config.push_str("# NeoMutt Configuration\n");
        config.push_str("# Generated based on requirements\n\n");

        // Add basic settings
        config.push_str("# Basic settings\n");
        config.push_str("set real_name = \"Your Name\"\n");
        config.push_str("set from = \"your.email@example.com\"\n");
        config.push_str("set editor = \"vim\"\n");
        config.push_str("set pager = \"less\"\n\n");

        // Check requirements for specific features
        let req_lower = requirements.to_lowercase();
        
        if req_lower.contains("imap") {
            config.push_str("# IMAP settings\n");
            config.push_str("set folder = \"imap://imap.example.com:993\"\n");
            config.push_str("set imap_user = \"your.email@example.com\"\n");
            config.push_str("set ssl_force_tls = yes\n");
            config.push_str("set spoolfile = \"+INBOX\"\n");
            config.push_str("set mailboxes = \"+INBOX\"\n\n");
        }

        if req_lower.contains("smtp") || req_lower.contains("send") {
            config.push_str("# SMTP settings\n");
            config.push_str("set smtp_url = \"smtp://your.email@example.com@smtp.example.com:587/\"\n");
            config.push_str("set ssl_starttls = yes\n\n");
        }

        if req_lower.contains("maildir") {
            config.push_str("# Maildir format\n");
            config.push_str("set mbox_type = Maildir\n");
            config.push_str("set folder = \"~/Mail\"\n");
            config.push_str("set spoolfile = \"+INBOX\"\n");
            config.push_str("set record = \"+Sent\"\n");
            config.push_str("set postponed = \"+Drafts\"\n");
            config.push_str("set trash = \"+Trash\"\n\n");
        }

        if req_lower.contains("multiple") || req_lower.contains("account") {
            config.push_str("# Multiple accounts example\n");
            config.push_str("# Use account-hook to configure different accounts\n");
            config.push_str("# account-hook imap://host1/ 'set imap_user=user1 imap_pass=pass1'\n");
            config.push_str("# account-hook imap://host2/ 'set imap_user=user2 imap_pass=pass2'\n\n");
        }

        if req_lower.contains("encrypt") || req_lower.contains("gpg") || req_lower.contains("pgp") {
            config.push_str("# Encryption settings\n");
            config.push_str("set crypt_use_gpgme = yes\n");
            config.push_str("set crypt_autosign = yes\n");
            config.push_str("set crypt_autoencrypt = yes\n\n");
        }

        config.push_str("# Additional customization\n");
        config.push_str("# set index_format = \"...\"\n");
        config.push_str("# set sort = date\n");

        Ok(serde_json::json!({
            "config": config,
            "requirements": requirements,
            "note": "Please review and customize the generated configuration"
        }))
    }

    pub fn add_account(&self, args: Option<&Value>) -> McpResult<Value> {
        let email = extract_string_param(args, "email")?;
        
        // Validate email format
        if !validate_email(&email) {
            return Err(McpError::ValidationError {
                message: format!("Invalid email address format: {}", email),
                field: Some("email".to_string()),
            });
        }

        let imap_server = extract_string_param(args, "imap_server")?;
        
        // Validate hostname
        if !validate_hostname(&imap_server) {
            return Err(McpError::ValidationError {
                message: format!("Invalid IMAP server hostname: {}", imap_server),
                field: Some("imap_server".to_string()),
            });
        }

        let smtp_server = extract_string_param(args, "smtp_server")?;
        
        // Validate hostname
        if !validate_hostname(&smtp_server) {
            return Err(McpError::ValidationError {
                message: format!("Invalid SMTP server hostname: {}", smtp_server),
                field: Some("smtp_server".to_string()),
            });
        }

        let imap_port = extract_optional_number_param::<u16>(args, "imap_port")
            .unwrap_or(993);
        
        if !validate_port(imap_port) {
            return Err(McpError::ValidationError {
                message: format!("Invalid IMAP port: {}", imap_port),
                field: Some("imap_port".to_string()),
            });
        }

        let smtp_port = extract_optional_number_param::<u16>(args, "smtp_port")
            .unwrap_or(587);
        
        if !validate_port(smtp_port) {
            return Err(McpError::ValidationError {
                message: format!("Invalid SMTP port: {}", smtp_port),
                field: Some("smtp_port".to_string()),
            });
        }

        let use_ssl = extract_optional_bool_param(args, "use_ssl").unwrap_or(true);

        let account = EmailAccount {
            email: email.clone(),
            real_name: None,
            imap_server: imap_server.clone(),
            imap_port,
            imap_user: None,
            imap_pass: None,
            smtp_server: smtp_server.clone(),
            smtp_port,
            smtp_user: None,
            smtp_pass: None,
            use_ssl,
            use_starttls: !use_ssl && smtp_port == 587,
        };

        let config = account.to_muttrc();

        Ok(serde_json::json!({
            "account": {
                "email": email,
                "imap_server": imap_server,
                "smtp_server": smtp_server
            },
            "config": config,
            "note": "Add this configuration to your muttrc file. Consider using account-hook for multiple accounts.",
            "security_warning": "Never store passwords in plain text. Consider using GPG-encrypted passwords or external password managers."
        }))
    }

    pub fn add_feature(&self, args: Option<&Value>) -> McpResult<Value> {
        let feature = extract_string_param(args, "feature")?;

        let feature_lower = feature.to_lowercase();
        // Pre-allocate capacity for config string
        let mut config = String::with_capacity(256);
        let description: String;

        match feature_lower.as_str() {
            "encryption" | "gpg" | "pgp" | "crypto" => {
                description = "GPG/PGP encryption and signing".to_string();
                config.push_str("# GPG/PGP Encryption\n");
                config.push_str("set crypt_use_gpgme = yes\n");
                config.push_str("set crypt_autosign = yes\n");
                config.push_str("set crypt_autoencrypt = yes\n");
                config.push_str("set crypt_replyencrypt = yes\n");
                config.push_str("set crypt_replysign = yes\n");
                // Optional: if gpg_key provided
                if let Some(gpg_key) = extract_optional_string_param(args, "gpg_key") {
                    config.push_str(&format!("set pgp_default_key = \"{}\"\n", gpg_key));
                }
            }
            "sidebar" | "mailbox_list" => {
                description = "Sidebar with mailbox list".to_string();
                config.push_str("# Sidebar\n");
                config.push_str("set sidebar_visible = yes\n");
                config.push_str("set sidebar_width = 30\n");
                config.push_str("set sidebar_format = \"%B%?F? [%F]?%* %?N?%N/?%S\"\n");
            }
            "notmuch" | "search" => {
                description = "Notmuch integration for search".to_string();
                config.push_str("# Notmuch Integration\n");
                config.push_str("set virtual_spoolfile = yes\n");
                config.push_str("# Requires notmuch to be installed\n");
                config.push_str("# Use 'notmuch' command to search\n");
            }
            "threading" | "threads" => {
                description = "Email threading support".to_string();
                config.push_str("# Threading\n");
                config.push_str("set sort = threads\n");
                config.push_str("set sort_aux = date\n");
                config.push_str("set strict_threads = yes\n");
            }
            "colors" | "color" => {
                description = "Color configuration".to_string();
                config.push_str("# Colors\n");
                config.push_str("color status green default\n");
                config.push_str("color tree red default\n");
                config.push_str("color hdrdefault cyan default\n");
                config.push_str("color quoted green default\n");
                config.push_str("color signature green default\n");
            }
            "index_format" | "custom_index" => {
                description = "Custom index format".to_string();
                let format = extract_optional_string_param(args, "format")
                    .unwrap_or_else(|| "%4C %Z %<[y?%<[m?%<[d?%[%H:%M   ]&%[%a %d  ]>&%[%d %b  ]>&%[%d/%m/%y]> %-15.15L (%?l?%4l&%4c?) %s".to_string());
                config.push_str("# Custom Index Format\n");
                config.push_str(&format!("set index_format = \"{}\"\n", format));
            }
            "key_bindings" | "bindings" => {
                description = "Custom key bindings".to_string();
                config.push_str("# Key Bindings\n");
                config.push_str("# bind index <delete> delete-message\n");
                config.push_str("# bind index <tab> next-unread\n");
                config.push_str("# bind pager <tab> next-unread\n");
            }
            "hooks" | "account_hook" => {
                description = "Account hooks for multiple accounts".to_string();
                config.push_str("# Account Hooks\n");
                config.push_str("# account-hook imap://host1/ 'set imap_user=user1 imap_pass=pass1'\n");
                config.push_str("# account-hook imap://host2/ 'set imap_user=user2 imap_pass=pass2'\n");
            }
            "maildir" => {
                description = "Maildir format support".to_string();
                config.push_str("# Maildir Format\n");
                config.push_str("set mbox_type = Maildir\n");
                config.push_str("set folder = \"~/Mail\"\n");
            }
            "mh" | "mh_format" => {
                description = "MH format support".to_string();
                config.push_str("# MH Format\n");
                config.push_str("set mbox_type = MH\n");
                config.push_str("set folder = \"~/Mail\"\n");
            }
            "mbox" | "mbox_format" => {
                description = "Mbox format support".to_string();
                config.push_str("# Mbox Format\n");
                config.push_str("set mbox_type = Mbox\n");
                config.push_str("set folder = \"~/Mail\"\n");
            }
            "imap_idle" | "idle" => {
                description = "IMAP IDLE support for real-time updates".to_string();
                config.push_str("# IMAP IDLE\n");
                config.push_str("set imap_idle = yes\n");
                config.push_str("set imap_check_subscribed = yes\n");
            }
            "header_cache" | "cache" => {
                description = "Header cache for faster loading".to_string();
                config.push_str("# Header Cache\n");
                config.push_str("set header_cache = \"~/.cache/neomutt\"\n");
                config.push_str("set header_cache_compress = yes\n");
            }
            "attachments" | "mime" => {
                description = "MIME and attachment handling".to_string();
                config.push_str("# MIME/Attachments\n");
                config.push_str("set mime_forward_decode = yes\n");
                config.push_str("set mime_forward_rest = yes\n");
                config.push_str("set attach_format = \"%u%D%I %t%4n %T%-40,40d %[%r%]\"\n");
            }
            "compose" | "editor" => {
                description = "Compose and editor settings".to_string();
                config.push_str("# Compose/Editor\n");
                config.push_str("set editor = \"vim\"\n");
                config.push_str("set edit_headers = yes\n");
                config.push_str("set compose_format = \"To: %f%t\\nCc: %c%t\\nBcc: %b%t\\nSubject: %s%t\\n--Attach: %a%t\\n--\"\n");
            }
            _ => {
                return Err(McpError::ParameterError {
                    message: format!(
                        "Unknown feature: {}. Supported features: encryption, sidebar, notmuch, threading, colors, index_format, key_bindings, hooks, maildir, mh, mbox, imap_idle, header_cache, attachments, compose",
                        feature
                    ),
                    parameter: Some("feature".to_string()),
                });
            }
        }

        // Add any additional options if provided
        if let Some(options) = args.and_then(|a| a.get("options")) {
            if let Some(opts_obj) = options.as_object() {
                config.push_str("\n# Additional options\n");
                for (key, value) in opts_obj {
                    if let Some(val_str) = value.as_str() {
                        config.push_str(&format!("set {} = \"{}\"\n", key, val_str));
                    } else if let Some(val_bool) = value.as_bool() {
                        config.push_str(&format!("set {} = {}\n", key, if val_bool { "yes" } else { "no" }));
                    } else if let Some(val_num) = value.as_u64() {
                        config.push_str(&format!("set {} = {}\n", key, val_num));
                    }
                }
            }
        }

        Ok(serde_json::json!({
            "feature": feature,
            "description": description,
            "config": config,
            "note": "Add this configuration to your muttrc file. Some features may require additional setup or dependencies."
        }))
    }
}

