use crate::models::WofiTemplate;

/// Get Wofi templates, optionally filtered by use case
pub fn get_templates(use_case: Option<&str>) -> Vec<WofiTemplate> {
    let all_templates = get_all_templates();
    
    if let Some(use_case_str) = use_case {
        all_templates
            .into_iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&use_case_str.to_lowercase())
                    || t.description.to_lowercase().contains(&use_case_str.to_lowercase())
                    || t.modes_used.iter().any(|m| m.contains(use_case_str))
            })
            .collect()
    } else {
        all_templates
    }
}

fn get_all_templates() -> Vec<WofiTemplate> {
    vec![
        WofiTemplate {
            name: "Minimal Launcher".to_string(),
            description: "Canonical default minimal launcher from sr.ht".to_string(),
            config_snippet: "width=600\nheight=400\nlocation=center\nmode=drun".to_string(),
            css_snippet: Some("window {\n  margin: 5px;\n}\n\n#input {\n  margin: 5px;\n}\n\n#list {\n  margin: 5px;\n}".to_string()),
            modes_used: vec!["drun".to_string()],
            source_documents: vec!["sr.ht".to_string()],
        },
        WofiTemplate {
            name: "App Launcher (drun)".to_string(),
            description: "Desktop entry application launcher".to_string(),
            config_snippet: "width=600\nheight=400\nlocation=center\nmode=drun\nfuzzy=true\nparse_action=true".to_string(),
            css_snippet: Some("window {\n  margin: 5px;\n  border-radius: 10px;\n}\n\n#input {\n  margin: 5px;\n  border-radius: 5px;\n}\n\n#list {\n  margin: 5px;\n}\n\nentry:selected {\n  background-color: #4a9eff;\n}".to_string()),
            modes_used: vec!["drun".to_string()],
            source_documents: vec!["sr.ht".to_string(), "cloudninja".to_string()],
        },
        WofiTemplate {
            name: "Command Launcher (run)".to_string(),
            description: "PATH executable command launcher".to_string(),
            config_snippet: "width=600\nheight=400\nlocation=center\nmode=run\nfuzzy=true".to_string(),
            css_snippet: Some("window {\n  margin: 5px;\n}\n\n#input {\n  margin: 5px;\n}\n\n#list {\n  margin: 5px;\n}".to_string()),
            modes_used: vec!["run".to_string()],
            source_documents: vec!["sr.ht".to_string(), "cloudninja".to_string()],
        },
        WofiTemplate {
            name: "SSH Launcher".to_string(),
            description: "SSH host connection launcher".to_string(),
            config_snippet: "width=600\nheight=400\nlocation=center\nmode=ssh\nfuzzy=true".to_string(),
            css_snippet: Some("window {\n  margin: 5px;\n}\n\n#input {\n  margin: 5px;\n}\n\n#list {\n  margin: 5px;\n}\n\nentry {\n  padding: 5px;\n}".to_string()),
            modes_used: vec!["ssh".to_string()],
            source_documents: vec!["cloudninja".to_string()],
        },
        WofiTemplate {
            name: "Fuzzy Fullscreen Mode".to_string(),
            description: "Fullscreen fuzzy launcher".to_string(),
            config_snippet: "width=100%\nheight=100%\nlocation=center\nmode=drun\nfuzzy=true\ninsensitive=true".to_string(),
            css_snippet: Some("window {\n  margin: 0px;\n}\n\n#input {\n  margin: 20px;\n  font-size: 24px;\n}\n\n#list {\n  margin: 20px;\n}".to_string()),
            modes_used: vec!["drun".to_string()],
            source_documents: vec!["cloudninja".to_string()],
        },
        WofiTemplate {
            name: "Hyprland-Optimized Launcher".to_string(),
            description: "Optimized for Hyprland compositor with wlr-layer-shell".to_string(),
            config_snippet: "width=600\nheight=400\nlocation=center\nlayer=overlay\nanchor=top\nmode=drun\nfuzzy=true".to_string(),
            css_snippet: Some("window {\n  margin: 10px;\n  border-radius: 10px;\n  background-color: rgba(0, 0, 0, 0.9);\n}\n\n#input {\n  margin: 10px;\n  border-radius: 5px;\n  padding: 10px;\n}\n\n#list {\n  margin: 10px;\n}\n\nentry:selected {\n  background-color: #4a9eff;\n}".to_string()),
            modes_used: vec!["drun".to_string()],
            source_documents: vec!["cloudninja".to_string()],
        },
        WofiTemplate {
            name: "Dmenu Compatibility Mode".to_string(),
            description: "dmenu-protocol-compatible mode with filtering".to_string(),
            config_snippet: "width=600\nheight=400\nlocation=center\nmode=dmenu\nfuzzy=true".to_string(),
            css_snippet: Some("window {\n  margin: 5px;\n}\n\n#input {\n  margin: 5px;\n}\n\n#list {\n  margin: 5px;\n}".to_string()),
            modes_used: vec!["dmenu".to_string()],
            source_documents: vec!["cloudninja".to_string()],
        },
        WofiTemplate {
            name: "Custom Script Mode".to_string(),
            description: "User-supplied script generating menu choices (stdin list → stdout selection)".to_string(),
            config_snippet: "width=600\nheight=400\nlocation=center\nmode=custom\nexec=/path/to/script.sh".to_string(),
            css_snippet: Some("window {\n  margin: 5px;\n}\n\n#input {\n  margin: 5px;\n}\n\n#list {\n  margin: 5px;\n}".to_string()),
            modes_used: vec!["custom".to_string()],
            source_documents: vec!["cloudninja".to_string()],
        },
    ]
}

