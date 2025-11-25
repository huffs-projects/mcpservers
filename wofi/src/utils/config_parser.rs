use std::collections::HashMap;
use anyhow::Result;

/// Parse Wofi config into structured key-value pairs
pub fn parse_config(content: &str) -> Result<HashMap<String, String>> {
    let mut config = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse key=value format
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();
            config.insert(key, value);
        }
    }

    Ok(config)
}

/// Serialize config map back to config file format
pub fn serialize_config(config: &HashMap<String, String>) -> String {
    let mut lines = Vec::new();
    
    for (key, value) in config.iter() {
        lines.push(format!("{}={}", key, value));
    }

    lines.join("\n")
}

