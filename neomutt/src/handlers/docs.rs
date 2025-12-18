use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};
use reqwest::blocking::Client;
use regex::Regex;

use crate::error::{McpError, McpResult};
use crate::utils::extract_string_param;

pub struct DocsHandler {
    cache_dir: PathBuf,
    known_options: HashMap<String, String>,
    http_client: Client,
    cache_ttl: Duration,
}

impl DocsHandler {
    pub fn new() -> Self {
        let cache_dir = PathBuf::from("data/docs");
        // Ensure cache directory exists
        if let Err(e) = fs::create_dir_all(&cache_dir) {
            eprintln!("Warning: Could not create cache directory: {}", e);
        }
        
        // Create HTTP client once and reuse it
        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| Client::new());
        
        let mut handler = Self {
            cache_dir,
            known_options: HashMap::new(),
            http_client,
            cache_ttl: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
        };
        handler.load_known_options();
        handler
    }

    fn load_known_options(&mut self) {
        // Comprehensive list of NeoMutt options
        let options = vec![
            // Basic settings
            ("real_name", "Your real name for email headers"),
            ("from", "Your email address"),
            ("editor", "External editor command"),
            ("pager", "External pager command"),
            ("askcc", "Ask for CC: field when composing"),
            ("askbcc", "Ask for Bcc: field when composing"),
            ("autoedit", "Automatically edit messages before sending"),
            ("edit_headers", "Edit headers when composing"),
            ("confirmappend", "Confirm before appending to mailbox"),
            ("confirmcreate", "Confirm before creating mailbox"),
            ("confirmdelete", "Confirm before deleting messages"),
            ("confirmquit", "Confirm before quitting"),
            // IMAP settings
            ("imap_user", "IMAP username"),
            ("imap_pass", "IMAP password"),
            ("imap_server", "IMAP server hostname"),
            ("imap_port", "IMAP server port"),
            ("imap_idle", "Enable IMAP IDLE for real-time updates"),
            ("imap_check_subscribed", "Check subscribed mailboxes"),
            ("imap_list_subscribed", "List only subscribed mailboxes"),
            ("imap_authenticators", "IMAP authentication methods"),
            ("imap_oauth2_cmd", "Command to get OAuth2 token"),
            ("imap_keepalive", "Keep IMAP connection alive"),
            // SMTP settings
            ("smtp_url", "SMTP server URL"),
            ("smtp_pass", "SMTP password"),
            ("smtp_authenticators", "SMTP authentication methods"),
            ("smtp_oauth2_cmd", "Command to get OAuth2 token"),
            // SSL/TLS
            ("ssl_force_tls", "Force TLS/SSL connection"),
            ("ssl_starttls", "Use STARTTLS"),
            ("ssl_verify_dates", "Verify SSL certificate dates"),
            ("ssl_verify_host", "Verify SSL certificate hostname"),
            ("ssl_ca_certificates_file", "CA certificates file"),
            ("ssl_client_cert", "Client certificate file"),
            ("ssl_client_key", "Client private key file"),
            // Mailboxes and folders
            ("folder", "Default mail folder"),
            ("spoolfile", "Incoming mail spool file"),
            ("record", "Where to save sent mail"),
            ("postponed", "Where to save postponed mail"),
            ("trash", "Where to move deleted mail"),
            ("mailboxes", "List of mailboxes to check for new mail"),
            ("mbox_type", "Mailbox format (Maildir, Mbox, MH)"),
            ("check_mbox_size", "Check mailbox size before opening"),
            ("check_new", "Check for new mail on startup"),
            ("timeout", "Timeout for operations"),
            // Display and formatting
            ("index_format", "Format string for the index display"),
            ("pager_format", "Format string for the pager display"),
            ("status_format", "Format string for the status bar"),
            ("attach_format", "Format string for attachments"),
            ("compose_format", "Format string for compose screen"),
            ("folder_format", "Format string for folder list"),
            ("sidebar_format", "Format string for sidebar"),
            ("sort", "Sort method for mail index"),
            ("sort_aux", "Auxiliary sort method"),
            ("sort_re", "Reverse sort order"),
            ("strict_threads", "Strict threading mode"),
            ("thread_received", "Thread by received date"),
            // Sidebar
            ("sidebar_visible", "Show sidebar"),
            ("sidebar_width", "Sidebar width in characters"),
            ("sidebar_divider_char", "Character to divide sidebar"),
            ("sidebar_folder_indent", "Indent folders in sidebar"),
            ("sidebar_short_path", "Show short paths in sidebar"),
            ("sidebar_sort_method", "How to sort sidebar"),
            // Colors
            ("color", "Set color for specific elements"),
            ("mono", "Use monochrome display"),
            // Cryptography
            ("crypt_use_gpgme", "Use GPGME for encryption"),
            ("crypt_autosign", "Automatically sign messages"),
            ("crypt_autoencrypt", "Automatically encrypt messages"),
            ("crypt_replyencrypt", "Encrypt replies to encrypted messages"),
            ("crypt_replysign", "Sign replies to signed messages"),
            ("crypt_replysignencrypted", "Sign encrypted replies"),
            ("pgp_default_key", "Default PGP key ID"),
            ("pgp_sign_as", "Sign messages with this key"),
            ("pgp_encrypt_self", "Encrypt messages to self"),
            ("pgp_self_encrypt", "Encrypt messages to self"),
            ("pgp_autoinline", "Automatically inline PGP signatures"),
            ("pgp_replyinline", "Reply inline to PGP messages"),
            ("pgp_timeout", "PGP timeout in seconds"),
            ("pgp_use_gpg_agent", "Use GPG agent"),
            // Headers
            ("ignore", "Ignore header fields"),
            ("unignore", "Don't ignore header fields"),
            ("hdr_format", "Format for header display"),
            ("weed", "Headers to weed out"),
            // Hooks
            ("account-hook", "Execute command for specific account"),
            ("folder-hook", "Execute command for specific folder"),
            ("message-hook", "Execute command for specific messages"),
            ("send-hook", "Execute command before sending"),
            ("send2-hook", "Execute command after sending"),
            ("reply-hook", "Execute command when replying"),
            ("save-hook", "Execute command when saving"),
            ("fcc-hook", "Execute command when saving copy"),
            ("fcc-save-hook", "Execute command when saving copy"),
            // Cache
            ("header_cache", "Header cache directory"),
            ("header_cache_compress", "Compress header cache"),
            ("message_cache_dir", "Message cache directory"),
            // MIME and attachments
            ("mime_forward_decode", "Decode MIME when forwarding"),
            ("mime_forward_rest", "Forward rest of message"),
            ("mime_forward_quote", "Quote forwarded message"),
            ("mime_forward", "Forward as attachment or inline"),
            ("mime_type_query_command", "Command to query MIME type"),
            ("mime_type_query_first", "Query MIME type first"),
            ("attach_sep", "Separator for attachments"),
            ("attach_split", "Split attachments"),
            ("forward_attachments", "Forward attachments"),
            ("forward_quote", "Quote forwarded message"),
            ("forward_format", "Format for forwarded message"),
            // Notmuch
            ("virtual_spoolfile", "Use virtual spoolfile"),
            // Other
            ("beep", "Beep on errors"),
            ("beep_new", "Beep when new mail arrives"),
            ("bounce", "Bounce messages"),
            ("bounce_delivered", "Bounce delivered messages"),
            ("copy", "Copy messages to another mailbox"),
            ("delete", "Delete messages"),
            ("delete_untag", "Untag when deleting"),
            ("duplicate_threads", "Show duplicate threads"),
            ("fast_reply", "Fast reply mode"),
            ("flag_safe", "Safe flagging"),
            ("followup_to", "Use Followup-To header"),
            ("force_name", "Force name in From field"),
            ("forward_references", "Forward references"),
            ("help", "Show help"),
            ("hidden_tags", "Hide tags"),
            ("hide_limited", "Hide limited messages"),
            ("hide_missing", "Hide missing messages"),
            ("hide_thread_subject", "Hide thread subject"),
            ("hide_top_limited", "Hide top limited messages"),
            ("hide_top_missing", "Hide top missing messages"),
            ("imap_condstore", "Use IMAP CONDSTORE extension"),
            ("imap_qresync", "Use IMAP QRESYNC extension"),
            ("imap_rfc5161", "Use IMAP RFC5161 extension"),
            ("imap_rfc5162", "Use IMAP RFC5162 extension"),
            ("imap_rfc6855", "Use IMAP RFC6855 extension"),
            ("imap_rfc7162", "Use IMAP RFC7162 extension"),
        ];

        for (name, desc) in options {
            self.known_options.insert(name.to_string(), desc.to_string());
        }

        // Try to load additional options from cached documentation
        self.load_options_from_cache();
    }

    fn load_options_from_cache(&mut self) {
        // Look for cached documentation files that might contain option lists
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    // Look for man page or option reference files
                    if file_name.contains("neomuttrc") || file_name.contains("options") {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            // Try to extract option names from documentation
                            // Look for patterns like "set option_name" or "option_name ="
                            let re = Regex::new(r"(?i)\bset\s+([a-z_][a-z0-9_]*)\s*=|([a-z_][a-z0-9_]*)\s*=").unwrap();
                            for cap in re.captures_iter(&content) {
                                if let Some(opt) = cap.get(1).or_else(|| cap.get(2)) {
                                    let opt_name = opt.as_str().to_string();
                                    if !self.known_options.contains_key(&opt_name) {
                                        // Extract description if possible (next line or nearby)
                                        let desc = format!("Configuration option: {}", opt_name);
                                        self.known_options.insert(opt_name, desc);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn search_docs(&self, args: Option<&Value>) -> McpResult<Value> {
        let query = extract_string_param(args, "query")?;

        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        // Search known options
        for (name, desc) in &self.known_options {
            if name.contains(&query_lower) || desc.to_lowercase().contains(&query_lower) {
                results.push(serde_json::json!({
                    "name": name,
                    "description": desc,
                    "type": "option"
                }));
            }
        }

        // Search cached documentation files
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".txt") || file_name.ends_with(".html") {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if content.to_lowercase().contains(&query_lower) {
                                // Extract a snippet
                                let snippet = self.extract_snippet(&content, &query_lower, 200);
                                results.push(serde_json::json!({
                                    "name": file_name.replace(".txt", "").replace(".html", ""),
                                    "description": snippet,
                                    "type": "cached_doc",
                                    "source": file_name
                                }));
                            }
                        }
                    }
                }
            }
        }

        // Also search common documentation topics
        let topics = vec![
            ("configuration", "Configuration guide"),
            ("getting started", "Getting started guide"),
            ("imap", "IMAP configuration"),
            ("smtp", "SMTP configuration"),
            ("encryption", "Encryption and security"),
            ("key bindings", "Key bindings"),
            ("hooks", "Hooks and automation"),
        ];

        for (topic, desc) in topics {
            if topic.contains(&query_lower) || desc.to_lowercase().contains(&query_lower) {
                results.push(serde_json::json!({
                    "name": topic,
                    "description": desc,
                    "type": "topic",
                    "url": format!("https://neomutt.org/guide/{}", topic.replace(" ", ""))
                }));
            }
        }

        Ok(serde_json::json!({
            "query": query,
            "results": results,
            "count": results.len()
        }))
    }

    fn extract_snippet(&self, content: &str, query: &str, max_len: usize) -> String {
        let content_lower = content.to_lowercase();
        if let Some(pos) = content_lower.find(query) {
            let start = pos.saturating_sub(50);
            let end = (pos + query.len() + 150).min(content.len());
            let snippet = &content[start..end];
                format!("...{}...", snippet.replace('\n', " ").trim())
        } else {
            content.chars().take(max_len).collect::<String>() + "..."
        }
    }

    pub fn get_config_option(&self, args: Option<&Value>) -> McpResult<Value> {
        let option = extract_string_param(args, "option")?;

        let description = self.known_options.get(&option).cloned()
            .unwrap_or_else(|| format!("Configuration option: {}", option));

        // Determine option type based on name patterns
        let option_type = if option.contains("_pass") || option.contains("password") {
            "string (password)"
        } else if option.contains("_port") || option.contains("timeout") {
            "number"
        } else if option.starts_with("ssl_") || option.contains("use_") || option.contains("enable_") {
            "boolean"
        } else if option.contains("_format") || option.contains("editor") || option.contains("pager") {
            "string"
        } else {
            "string"
        };

        let example = match option.as_str() {
            "real_name" => Some("\"John Doe\""),
            "from" => Some("\"user@example.com\""),
            "imap_user" => Some("\"user@example.com\""),
            "smtp_url" => Some("\"smtp://user@example.com@smtp.example.com:587/\""),
            "folder" => Some("\"imap://imap.example.com:993\""),
            "ssl_force_tls" => Some("yes"),
            _ => None,
        };

        Ok(serde_json::json!({
            "option": option,
            "description": description,
            "type": option_type,
            "example": example,
            "documentation_url": format!("https://neomutt.org/man/neomuttrc#{}", option)
        }))
    }

    pub fn get_guide_section(&self, args: Option<&Value>) -> McpResult<Value> {
        let section = extract_string_param(args, "section")?;

        // Map common section names to URLs
        let url = if section.starts_with("http") {
            section.to_string()
        } else {
            match section.to_lowercase().as_str() {
                "configuration" | "config" => "https://neomutt.org/guide/configuration.html".to_string(),
                "getting started" | "gettingstarted" => "https://neomutt.org/guide/gettingstarted.html".to_string(),
                "imap" => "https://neomutt.org/guide/imap.html".to_string(),
                "smtp" => "https://neomutt.org/guide/smtp.html".to_string(),
                "encryption" | "pgp" | "gpg" => "https://neomutt.org/guide/crypto.html".to_string(),
                "key bindings" | "keybindings" => "https://neomutt.org/guide/advancedusage.html#key-bindings".to_string(),
                "hooks" => "https://neomutt.org/guide/optionalfeatures.html#hooks".to_string(),
                _ => {
                    // Try to construct URL from section name
                    let normalized = section.to_lowercase().replace(" ", "");
                    format!("https://neomutt.org/guide/{}.html", normalized)
                }
            }
        };

        // Try to fetch and cache the content
        match self.fetch_and_cache(&url) {
            Ok(content) => {
                // Extract a summary from the HTML (basic text extraction)
                let text_content = self.extract_text_from_html(&content);
                let summary = if text_content.len() > 500 {
                    format!("{}...", &text_content[..500])
                } else {
                    text_content
                };

                Ok(serde_json::json!({
                    "section": section,
                    "url": url,
                    "content": content,
                    "summary": summary,
                    "cached": true
                }))
            }
            Err(e) => {
                // Fallback to URL only if fetch fails
                Ok(serde_json::json!({
                    "section": section,
                    "url": url,
                    "summary": format!("Documentation for: {}", section),
                    "error": format!("Could not fetch content: {}", e),
                    "note": "Visit the URL for full documentation."
                }))
            }
        }
    }

    fn extract_text_from_html(&self, html: &str) -> String {
        // Basic HTML text extraction - remove tags and decode entities
        let mut text = String::new();
        let mut in_tag = false;
        let mut in_script = false;
        let mut in_style = false;
        
        let chars: Vec<char> = html.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if i < chars.len() - 6 && &html[i..i.min(i+7)] == "<script" {
                in_script = true;
                i += 7;
                continue;
            }
            if i < chars.len() - 6 && &html[i..i.min(i+8)] == "</script" {
                in_script = false;
                i += 8;
                continue;
            }
            if i < chars.len() - 5 && &html[i..i.min(i+6)] == "<style" {
                in_style = true;
                i += 6;
                continue;
            }
            if i < chars.len() - 6 && &html[i..i.min(i+7)] == "</style" {
                in_style = false;
                i += 7;
                continue;
            }
            
            if chars[i] == '<' {
                in_tag = true;
            } else if chars[i] == '>' {
                in_tag = false;
            } else if !in_tag && !in_script && !in_style {
                if chars[i] == '\n' || chars[i] == '\r' {
                    if !text.ends_with(' ') {
                        text.push(' ');
                    }
                } else {
                    text.push(chars[i]);
                }
            }
            i += 1;
        }
        
        // Clean up multiple spaces
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn cache_path(&self, key: &str) -> PathBuf {
        // Create a safe filename from URL
        let safe_key = key
            .replace("https://", "")
            .replace("http://", "")
            .replace("/", "_")
            .replace(":", "_")
            .replace("?", "_")
            .replace("&", "_")
            .replace("=", "_");
        
        // Sanitize path to prevent directory traversal
        let path = self.cache_dir.join(format!("{}.html", safe_key));
        
        // Ensure the path stays within cache directory
        if path.starts_with(&self.cache_dir) {
            path
        } else {
            // Fallback to a safe filename if path traversal detected
            self.cache_dir.join("default.html")
        }
    }

    fn fetch_and_cache(&self, url: &str) -> McpResult<String> {
        let cache_file = self.cache_path(url);
        
        // Check if cached file exists and is recent
        if cache_file.exists() {
            if let Ok(metadata) = fs::metadata(&cache_file) {
                if let Ok(modified) = metadata.modified() {
                    // Use SystemTime comparison to check cache age
                    if let Ok(age) = SystemTime::now().duration_since(modified) {
                        // Use cache if less than TTL
                        if age < self.cache_ttl {
                            if let Ok(content) = fs::read_to_string(&cache_file) {
                                return Ok(content);
                            }
                        }
                    }
                }
            }
        }

        // Fetch from URL using the reused client
        let response = self.http_client
            .get(url)
            .header("User-Agent", "neomutt-mcp-server/0.1.0")
            .send()
            .map_err(|e| McpError::NetworkError {
                message: e.to_string(),
                url: Some(url.to_string()),
            })?;

        if !response.status().is_success() {
            return Err(McpError::NetworkError {
                message: format!("HTTP error: {}", response.status()),
                url: Some(url.to_string()),
            });
        }

        let content = response.text().map_err(|e| McpError::NetworkError {
            message: format!("Failed to read response body: {}", e),
            url: Some(url.to_string()),
        })?;

        // Cache the content
        if let Err(e) = fs::write(&cache_file, &content) {
            eprintln!("Warning: Could not cache documentation: {}", e);
        }

        Ok(content)
    }
}

