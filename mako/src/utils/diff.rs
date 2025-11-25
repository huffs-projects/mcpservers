use crate::utils::parser::ConfigMap;
use std::collections::HashSet;

/// Generate unified diff between two configs
pub fn generate_diff(old_config: &ConfigMap, new_config: &ConfigMap) -> String {
    let mut diff_lines = Vec::new();
    
    // Collect all sections
    let mut all_sections: HashSet<&String> = old_config.keys().collect();
    all_sections.extend(new_config.keys());
    
    let mut sorted_sections: Vec<&String> = all_sections.iter().cloned().collect();
    sorted_sections.sort();
    
    for section in sorted_sections {
        let old_section = old_config.get(section);
        let new_section = new_config.get(section);
        
        if old_section.is_none() && new_section.is_some() {
            // New section added
            diff_lines.push(format!("+[{}]", section));
            if let Some(entries) = new_section {
                for (key, value) in entries {
                    diff_lines.push(format!("+{}={}", key, value));
                }
            }
            continue;
        }
        
        if old_section.is_some() && new_section.is_none() {
            // Section removed
            diff_lines.push(format!("-[{}]", section));
            if let Some(entries) = old_section {
                for (key, value) in entries {
                    diff_lines.push(format!("-{}={}", key, value));
                }
            }
            continue;
        }
        
        if let (Some(old_entries), Some(new_entries)) = (old_section, new_section) {
            // Compare entries in section
            let mut all_keys: HashSet<&String> = old_entries.keys().collect();
            all_keys.extend(new_entries.keys());
            
            let mut sorted_keys: Vec<&String> = all_keys.iter().cloned().collect();
            sorted_keys.sort();
            
            for key in sorted_keys {
                let old_value = old_entries.get(key);
                let new_value = new_entries.get(key);
                
                match (old_value, new_value) {
                    (None, Some(v)) => {
                        diff_lines.push(format!("+{}={}", key, v));
                    }
                    (Some(v), None) => {
                        diff_lines.push(format!("-{}={}", key, v));
                    }
                    (Some(old_v), Some(new_v)) if old_v != new_v => {
                        diff_lines.push(format!("-{}={}", key, old_v));
                        diff_lines.push(format!("+{}={}", key, new_v));
                    }
                    _ => {}
                }
            }
        }
    }
    
    if diff_lines.is_empty() {
        "(no changes)".to_string()
    } else {
        diff_lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parser::ConfigMap;
    use std::collections::HashMap;

    fn create_config() -> ConfigMap {
        let mut config = ConfigMap::new();
        let mut default = HashMap::new();
        default.insert("font".to_string(), "monospace 10".to_string());
        default.insert("background-color".to_string(), "#285577".to_string());
        config.insert("default".to_string(), default);
        config
    }

    #[test]
    fn test_diff_no_changes() {
        let config = create_config();
        let diff = generate_diff(&config, &config);
        assert_eq!(diff, "(no changes)");
    }

    #[test]
    fn test_diff_added_key() {
        let old_config = create_config();
        let mut new_config = old_config.clone();
        new_config.get_mut("default").unwrap().insert("text-color".to_string(), "#ffffff".to_string());

        let diff = generate_diff(&old_config, &new_config);
        assert!(diff.contains("+text-color=#ffffff"));
    }

    #[test]
    fn test_diff_removed_key() {
        let old_config = create_config();
        let mut new_config = old_config.clone();
        new_config.get_mut("default").unwrap().remove("font");

        let diff = generate_diff(&old_config, &new_config);
        assert!(diff.contains("-font=monospace 10"));
    }

    #[test]
    fn test_diff_changed_value() {
        let old_config = create_config();
        let mut new_config = old_config.clone();
        new_config.get_mut("default").unwrap().insert("font".to_string(), "monospace 12".to_string());

        let diff = generate_diff(&old_config, &new_config);
        assert!(diff.contains("-font=monospace 10"));
        assert!(diff.contains("+font=monospace 12"));
    }

    #[test]
    fn test_diff_added_section() {
        let old_config = create_config();
        let mut new_config = old_config.clone();
        let mut new_section = HashMap::new();
        new_section.insert("background-color".to_string(), "#333333".to_string());
        new_config.insert("urgency=low".to_string(), new_section);

        let diff = generate_diff(&old_config, &new_config);
        assert!(diff.contains("+[urgency=low]"));
    }

    #[test]
    fn test_diff_removed_section() {
        let mut old_config = create_config();
        let mut section = HashMap::new();
        section.insert("background-color".to_string(), "#333333".to_string());
        old_config.insert("urgency=low".to_string(), section);
        let new_config = create_config();

        let diff = generate_diff(&old_config, &new_config);
        assert!(diff.contains("-[urgency=low]"));
    }

    #[test]
    fn test_diff_multiple_changes() {
        let old_config = create_config();
        let mut new_config = old_config.clone();
        new_config.get_mut("default").unwrap().insert("font".to_string(), "monospace 12".to_string());
        new_config.get_mut("default").unwrap().insert("text-color".to_string(), "#ffffff".to_string());
        new_config.get_mut("default").unwrap().remove("background-color");

        let diff = generate_diff(&old_config, &new_config);
        assert!(diff.contains("-font=monospace 10"));
        assert!(diff.contains("+font=monospace 12"));
        assert!(diff.contains("+text-color=#ffffff"));
        assert!(diff.contains("-background-color=#285577"));
    }
}

