use crate::models::WofiMode;

/// Get Wofi modes, optionally filtered
pub fn get_modes(filter: Option<&str>) -> Vec<WofiMode> {
    let all_modes = get_all_modes();
    
    if let Some(filter_str) = filter {
        all_modes
            .into_iter()
            .filter(|m| {
                m.name.contains(filter_str)
                    || m.mode_type.contains(filter_str)
            })
            .collect()
    } else {
        all_modes
    }
}

fn get_all_modes() -> Vec<WofiMode> {
    vec![
        WofiMode {
            name: "drun".to_string(),
            mode_type: "builtin".to_string(),
            exec: None,
            stdin_format: None,
            stdout_format: Some("name|description|exec|icon".to_string()),
            cloudninja_section: "drun-mode".to_string(),
            manpage_section: "MODES".to_string(),
        },
        WofiMode {
            name: "run".to_string(),
            mode_type: "builtin".to_string(),
            exec: None,
            stdin_format: None,
            stdout_format: Some("executable_path".to_string()),
            cloudninja_section: "run-mode".to_string(),
            manpage_section: "MODES".to_string(),
        },
        WofiMode {
            name: "ssh".to_string(),
            mode_type: "builtin".to_string(),
            exec: None,
            stdin_format: None,
            stdout_format: Some("host|user@host".to_string()),
            cloudninja_section: "ssh-mode".to_string(),
            manpage_section: "MODES".to_string(),
        },
        WofiMode {
            name: "dmenu".to_string(),
            mode_type: "builtin".to_string(),
            exec: None,
            stdin_format: Some("line_per_item".to_string()),
            stdout_format: Some("selected_line".to_string()),
            cloudninja_section: "dmenu-mode".to_string(),
            manpage_section: "MODES".to_string(),
        },
        WofiMode {
            name: "custom".to_string(),
            mode_type: "custom".to_string(),
            exec: Some("/path/to/script.sh".to_string()),
            stdin_format: Some("line_per_item".to_string()),
            stdout_format: Some("selected_line".to_string()),
            cloudninja_section: "custom-mode".to_string(),
            manpage_section: "MODES".to_string(),
        },
    ]
}

