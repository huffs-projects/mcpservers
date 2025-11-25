use anyhow::Result;
use std::collections::HashMap;

pub type ConfigMap = HashMap<String, HashMap<String, String>>;

/// Parse Mako config file (INI-style) into structured Rust types
pub fn parse_config(content: &str) -> Result<ConfigMap> {
    let mut config = ConfigMap::new();
    let mut current_section = "default".to_string();

    for line in content.lines() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse section headers [section]
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_string();
            config.entry(current_section.clone()).or_insert_with(Default::default);
            continue;
        }

        // Parse key=value pairs
        if let Some(equal_pos) = line.find('=') {
            let key = line[..equal_pos].trim().to_string();
            let value = line[equal_pos + 1..].trim().to_string();
            
            config
                .entry(current_section.clone())
                .or_insert_with(Default::default)
                .insert(key, value);
        }
    }

    Ok(config)
}

/// Serialize ConfigMap back to INI format
pub fn serialize_config(config: &ConfigMap) -> String {
    let mut output = String::new();
    
    for (section, entries) in config {
        if section != "default" || !config.is_empty() {
            output.push_str(&format!("[{}]\n", section));
        }
        
        for (key, value) in entries {
            output.push_str(&format!("{}={}\n", key, value));
        }
        
        output.push('\n');
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let content = "[default]\nfont=monospace 10\nbackground-color=#285577\n";
        let config = parse_config(content).unwrap();
        assert_eq!(config.get("default").unwrap().get("font"), Some(&"monospace 10".to_string()));
        assert_eq!(config.get("default").unwrap().get("background-color"), Some(&"#285577".to_string()));
    }

    #[test]
    fn test_parse_multiple_sections() {
        let content = "[default]\nfont=monospace 10\n\n[urgency=low]\nbackground-color=#333333\n";
        let config = parse_config(content).unwrap();
        assert!(config.contains_key("default"));
        assert!(config.contains_key("urgency=low"));
        assert_eq!(config.get("default").unwrap().get("font"), Some(&"monospace 10".to_string()));
        assert_eq!(config.get("urgency=low").unwrap().get("background-color"), Some(&"#333333".to_string()));
    }

    #[test]
    fn test_parse_with_comments() {
        let content = "[default]\n# This is a comment\nfont=monospace 10\nbackground-color=#285577\n";
        let config = parse_config(content).unwrap();
        assert_eq!(config.get("default").unwrap().get("font"), Some(&"monospace 10".to_string()));
        assert!(!config.get("default").unwrap().contains_key("# This is a comment"));
    }

    #[test]
    fn test_parse_empty_lines() {
        let content = "[default]\n\nfont=monospace 10\n\nbackground-color=#285577\n";
        let config = parse_config(content).unwrap();
        assert_eq!(config.get("default").unwrap().len(), 2);
    }

    #[test]
    fn test_parse_no_section() {
        let content = "font=monospace 10\nbackground-color=#285577\n";
        let config = parse_config(content).unwrap();
        assert_eq!(config.get("default").unwrap().get("font"), Some(&"monospace 10".to_string()));
    }

    #[test]
    fn test_serialize_config() {
        let mut config = ConfigMap::new();
        let mut default_section = HashMap::new();
        default_section.insert("font".to_string(), "monospace 10".to_string());
        default_section.insert("background-color".to_string(), "#285577".to_string());
        config.insert("default".to_string(), default_section);

        let serialized = serialize_config(&config);
        assert!(serialized.contains("[default]"));
        assert!(serialized.contains("font=monospace 10"));
        assert!(serialized.contains("background-color=#285577"));
    }

    #[test]
    fn test_parse_and_serialize_roundtrip() {
        let content = "[default]\nfont=monospace 10\nbackground-color=#285577\n";
        let config = parse_config(content).unwrap();
        let serialized = serialize_config(&config);
        let reparsed = parse_config(&serialized).unwrap();

        assert_eq!(config.get("default").unwrap().get("font"), reparsed.get("default").unwrap().get("font"));
        assert_eq!(config.get("default").unwrap().get("background-color"), reparsed.get("default").unwrap().get("background-color"));
    }

    #[test]
    fn test_parse_with_spaces() {
        let content = "[default]\n  font  =  monospace 10  \nbackground-color=#285577\n";
        let config = parse_config(content).unwrap();
        assert_eq!(config.get("default").unwrap().get("font"), Some(&"monospace 10".to_string()));
    }
}

