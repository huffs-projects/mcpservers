use crate::models::WaybarScript;
use crate::utils::{DocMapper, WaybarParser};
use anyhow::Result;
use std::collections::HashMap;

pub fn query_scripts(config_path: Option<&str>, filter_name: Option<String>) -> Result<Vec<WaybarScript>> {
    let mut scripts = Vec::new();

    if let Some(path) = config_path {
        // Expand and validate path
        let expanded_path = crate::utils::FileOps::validate_file_path(path)?;
        let path_str = expanded_path.to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path encoding"))?;
        
        match WaybarParser::parse_json(path_str) {
            Ok(config) => {
                let custom_scripts = WaybarParser::extract_custom_scripts(&config);
                
                for (name, command) in custom_scripts {
                    if let Some(ref filter) = filter_name {
                        if !name.contains(filter) {
                            continue;
                        }
                    }

                    let script = WaybarScript::new(
                        name.clone(),
                        command.clone(),
                        DocMapper::get_script_doc_url(),
                    );

                    scripts.push(script);
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to parse config: {}", e));
            }
        }
    }

    Ok(scripts)
}

pub fn get_script_template() -> HashMap<String, String> {
    let mut templates = HashMap::new();
    
    templates.insert(
        "custom-repeating".to_string(),
        r#"{
  "custom/example": {
    "exec": "echo 'Hello World'",
    "interval": 5,
    "format": "{}",
    "return-type": "json"
  }
}"#.to_string(),
    );

    templates.insert(
        "exec-once".to_string(),
        r#"{
  "exec/example": {
    "exec": "echo 'One-time execution'",
    "format": "{}"
  }
}"#.to_string(),
    );

    templates
}

