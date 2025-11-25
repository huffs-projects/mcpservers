use std::collections::HashMap;

pub struct DocMapper;

impl DocMapper {
    pub fn get_module_doc_url(module: &str) -> String {
        let base_url = "https://waybar.org/what-modules-come-built-in-with-waybar/";
        format!("{}#{}", base_url, module)
    }

    pub fn get_script_doc_url() -> String {
        "https://waybar.org/can-i-add-custom-scripts-to-waybar/".to_string()
    }

    pub fn get_style_doc_url() -> String {
        "https://waybar.org/how-can-i-style-waybar-with-css/".to_string()
    }

    pub fn get_config_location_doc_url() -> String {
        "https://waybar.org/where-is-waybars-configuration-file-located/".to_string()
    }

    pub fn get_appearance_doc_url() -> String {
        "https://waybar.org/can-i-customize-waybars-look/".to_string()
    }

    pub fn get_examples_url() -> String {
        "https://github.com/Alexays/Waybar/wiki/Examples".to_string()
    }

    pub fn get_hyprland_url() -> String {
        "https://wiki.hypr.land/Useful-Utilities/Status-Bars/".to_string()
    }

    pub fn get_css_selectors() -> HashMap<String, String> {
        let mut selectors = HashMap::new();
        selectors.insert("window".to_string(), "Main window container".to_string());
        selectors.insert("window#waybar".to_string(), "Waybar window".to_string());
        selectors.insert("tooltip".to_string(), "Tooltip styling".to_string());
        selectors.insert("label".to_string(), "Text labels".to_string());
        selectors.insert("button".to_string(), "Clickable buttons".to_string());
        selectors.insert("#battery".to_string(), "Battery module".to_string());
        selectors.insert("#cpu".to_string(), "CPU module".to_string());
        selectors.insert("#memory".to_string(), "Memory module".to_string());
        selectors.insert("#network".to_string(), "Network module".to_string());
        selectors.insert("#clock".to_string(), "Clock module".to_string());
        selectors.insert("#tray".to_string(), "Tray module".to_string());
        selectors.insert("#custom-*".to_string(), "Custom modules".to_string());
        selectors
    }
}

