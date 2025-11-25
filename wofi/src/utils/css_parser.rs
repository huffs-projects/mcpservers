use anyhow::Result;
use std::collections::HashMap;

/// Parse CSS content, preserving comments
pub fn parse_css(content: &str) -> Result<HashMap<String, HashMap<String, String>>> {
    let mut rules = HashMap::new();
    let mut current_selector = None;
    let mut current_properties = HashMap::new();
    let mut in_rule = false;

    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments
        if line.starts_with("/*") || line.starts_with("//") {
            continue;
        }

        // Selector line (ends with {)
        if line.ends_with('{') {
            if let Some(prev_selector) = current_selector.take() {
                if !current_properties.is_empty() {
                    rules.insert(prev_selector, current_properties.clone());
                }
            }
            let selector = line.trim_end_matches('{').trim().to_string();
            current_selector = Some(selector);
            current_properties.clear();
            in_rule = true;
        }
        // Property line (key: value;)
        else if in_rule && line.contains(':') {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_string();
                let value = value.trim().trim_end_matches(';').trim().to_string();
                current_properties.insert(key, value);
            }
        }
        // Closing brace
        else if line == "}" {
            if let Some(selector) = current_selector.take() {
                if !current_properties.is_empty() {
                    rules.insert(selector, current_properties.clone());
                }
            }
            current_properties.clear();
            in_rule = false;
        }
    }

    // Handle last rule if file doesn't end with }
    if let Some(selector) = current_selector {
        if !current_properties.is_empty() {
            rules.insert(selector, current_properties);
        }
    }

    Ok(rules)
}

/// Serialize CSS rules back to CSS format
pub fn serialize_css(rules: &HashMap<String, HashMap<String, String>>) -> String {
    let mut lines = Vec::new();

    for (selector, properties) in rules.iter() {
        lines.push(format!("{} {{", selector));
        for (key, value) in properties.iter() {
            lines.push(format!("  {}: {};", key, value));
        }
        lines.push("}".to_string());
        lines.push("".to_string());
    }

    lines.join("\n")
}

