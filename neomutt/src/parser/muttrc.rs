use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use crate::models::config::ConfigCommand;
use crate::error::McpError;

// Compile regexes once at startup for better performance
static SET_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*set\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.+)$"#).unwrap()
});

static UNSET_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*unset\s+([a-zA-Z_][a-zA-Z0-9_]*)"#).unwrap()
});

static SOURCE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*source\s+(.+)$"#).unwrap()
});

static ACCOUNT_HOOK_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*account-hook\s+(.+?)\s+(.+)$"#).unwrap()
});

static FOLDER_HOOK_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*folder-hook\s+(.+?)\s+(.+)$"#).unwrap()
});

static COMMENT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*#"#).unwrap()
});

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl From<ParseError> for McpError {
    fn from(err: ParseError) -> Self {
        McpError::ParseError {
            line: err.line,
            message: err.message,
            context: None,
        }
    }
}

pub struct MuttrcParser;

impl MuttrcParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, content: &str) -> Result<Vec<ConfigCommand>, ParseError> {
        let mut commands = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || COMMENT_RE.is_match(line) {
                continue;
            }

            // Handle semicolon-separated commands
            let parts: Vec<&str> = line.split(';').collect();
            for part in parts {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }

                if let Some(cmd) = self.parse_line(part, line_num + 1)? {
                    commands.push(cmd);
                }
            }
        }

        Ok(commands)
    }

    fn parse_line(&self, line: &str, line_num: usize) -> Result<Option<ConfigCommand>, ParseError> {
        if COMMENT_RE.is_match(line) {
            return Ok(None);
        }

        if let Some(caps) = SET_RE.captures(line) {
            let option = caps.get(1).map(|m| m.as_str().to_string());
            let value = caps.get(2).map(|m| Self::unquote(m.as_str()));
            return Ok(Some(ConfigCommand {
                command: "set".to_string(),
                option,
                value,
                line_number: line_num,
            }));
        }

        if let Some(caps) = UNSET_RE.captures(line) {
            let option = caps.get(1).map(|m| m.as_str().to_string());
            return Ok(Some(ConfigCommand {
                command: "unset".to_string(),
                option,
                value: None,
                line_number: line_num,
            }));
        }

        if SOURCE_RE.is_match(line) {
            return Ok(Some(ConfigCommand {
                command: "source".to_string(),
                option: None,
                value: None,
                line_number: line_num,
            }));
        }

        if ACCOUNT_HOOK_RE.is_match(line) {
            return Ok(Some(ConfigCommand {
                command: "account-hook".to_string(),
                option: None,
                value: None,
                line_number: line_num,
            }));
        }

        if FOLDER_HOOK_RE.is_match(line) {
            return Ok(Some(ConfigCommand {
                command: "folder-hook".to_string(),
                option: None,
                value: None,
                line_number: line_num,
            }));
        }

        // Unknown command - return error with more context
        Err(ParseError {
            line: line_num,
            message: format!("Unknown command: {}. Expected one of: set, unset, source, account-hook, folder-hook", line),
        })
    }

    fn unquote(s: &str) -> String {
        let s = s.trim();
        if s.starts_with('"') && s.ends_with('"') {
            s[1..s.len() - 1].to_string()
        } else if s.starts_with('\'') && s.ends_with('\'') {
            s[1..s.len() - 1].to_string()
        } else {
            s.to_string()
        }
    }

    pub fn validate_option_name(name: &str) -> bool {
        // Basic validation - option names should be alphanumeric with underscores
        name.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    pub fn extract_options(&self, content: &str) -> HashMap<String, String> {
        let mut options = HashMap::new();
        
        if let Ok(commands) = self.parse(content) {
            for cmd in commands {
                if cmd.command == "set" {
                    if let (Some(opt), Some(val)) = (cmd.option, cmd.value) {
                        options.insert(opt, val);
                    }
                }
            }
        }
        
        options
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        let parser = MuttrcParser::new();
        let config = "set real_name = \"John Doe\"";
        let commands = parser.parse(config).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "set");
        assert_eq!(commands[0].option, Some("real_name".to_string()));
    }

    #[test]
    fn test_parse_unset() {
        let parser = MuttrcParser::new();
        let config = "unset real_name";
        let commands = parser.parse(config).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "unset");
    }

    #[test]
    fn test_validate_option_name() {
        assert!(MuttrcParser::validate_option_name("real_name"));
        assert!(MuttrcParser::validate_option_name("imap_user"));
        assert!(!MuttrcParser::validate_option_name("real-name"));
        assert!(!MuttrcParser::validate_option_name("real name"));
    }
}

