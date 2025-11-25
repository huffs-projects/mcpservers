use crate::models::WaybarStyleSnippet;
use crate::utils::DocMapper;
use std::collections::HashMap;

pub fn query_styles(selector: Option<String>) -> Vec<WaybarStyleSnippet> {
    let mut snippets = Vec::new();
    let all_selectors = DocMapper::get_css_selectors();
    let doc_url = DocMapper::get_style_doc_url();

    for (sel, description) in all_selectors {
        if let Some(ref filter) = selector {
            if !sel.contains(filter) {
                continue;
            }
        }

        let mut properties = HashMap::new();
        
        match sel.as_str() {
            "window" | "window#waybar" => {
                properties.insert("background-color".to_string(), "#1e1e2e".to_string());
                properties.insert("color".to_string(), "#cdd6f4".to_string());
                properties.insert("border".to_string(), "none".to_string());
            }
            "#battery" | "#cpu" | "#memory" | "#network" | "#clock" => {
                properties.insert("padding".to_string(), "0 10px".to_string());
                properties.insert("margin".to_string(), "0 4px".to_string());
            }
            "#tray" => {
                properties.insert("padding".to_string(), "0 10px".to_string());
            }
            "tooltip" => {
                properties.insert("background-color".to_string(), "#1e1e2e".to_string());
                properties.insert("color".to_string(), "#cdd6f4".to_string());
                properties.insert("border-radius".to_string(), "5px".to_string());
            }
            _ => {
                properties.insert("padding".to_string(), "0 5px".to_string());
            }
        }

        snippets.push(
            WaybarStyleSnippet::new(sel.clone(), properties, doc_url.clone())
                .with_notes(description),
        );
    }

    snippets
}

pub fn get_common_style_templates() -> HashMap<String, String> {
    let mut templates = HashMap::new();

    templates.insert(
        "catppuccin-mocha".to_string(),
        r#"* {
  border: none;
  border-radius: 0;
  font-family: "JetBrainsMono Nerd Font";
  font-size: 13px;
  min-height: 0;
}

window#waybar {
  background-color: #1e1e2e;
  color: #cdd6f4;
}

#workspaces button {
  padding: 0 5px;
  background-color: transparent;
  color: #cdd6f4;
}

#workspaces button.focused {
  background-color: #89b4fa;
  color: #1e1e2e;
}

#battery, #cpu, #memory, #network, #clock {
  padding: 0 10px;
  margin: 0 4px;
}

tooltip {
  background-color: #1e1e2e;
  color: #cdd6f4;
  border-radius: 5px;
}"#.to_string(),
    );

    templates.insert(
        "minimal".to_string(),
        r#"* {
  border: none;
  font-family: "Fira Code";
  font-size: 12px;
}

window#waybar {
  background-color: #2b2b2b;
  color: #ffffff;
}

#workspaces button {
  padding: 0 8px;
}

#workspaces button.focused {
  background-color: #4a9eff;
}

#battery, #cpu, #memory, #network, #clock {
  padding: 0 8px;
  margin: 0 2px;
}"#.to_string(),
    );

    templates
}

