use crate::models::MakoOption;
use crate::utils::logger::EndpointLogger;

/// Get all Mako configuration options with their types, defaults, and valid values
/// Based on Mako's actual configuration options from the source repository
pub fn get_mako_options(search_term: Option<&str>) -> Vec<MakoOption> {
    let _logger = EndpointLogger::new("mako_options");
    
    let all_options = vec![
        MakoOption {
            name: "font".to_string(),
            option_type: "string".to_string(),
            default: Some("monospace 10".to_string()),
            description: "Font family and size for notifications".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#font".to_string(),
        },
        MakoOption {
            name: "background-color".to_string(),
            option_type: "color".to_string(),
            default: Some("#285577".to_string()),
            description: "Background color for notifications".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#background-color".to_string(),
        },
        MakoOption {
            name: "text-color".to_string(),
            option_type: "color".to_string(),
            default: Some("#ffffff".to_string()),
            description: "Text color for notifications".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#text-color".to_string(),
        },
        MakoOption {
            name: "width".to_string(),
            option_type: "integer".to_string(),
            default: Some("300".to_string()),
            description: "Width of notification window in pixels".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#width".to_string(),
        },
        MakoOption {
            name: "height".to_string(),
            option_type: "integer".to_string(),
            default: Some("100".to_string()),
            description: "Height of notification window in pixels".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#height".to_string(),
        },
        MakoOption {
            name: "margin".to_string(),
            option_type: "string".to_string(),
            default: Some("10".to_string()),
            description: "Margin around notification window (top,right,bottom,left or single value)".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#margin".to_string(),
        },
        MakoOption {
            name: "padding".to_string(),
            option_type: "string".to_string(),
            default: Some("5".to_string()),
            description: "Padding inside notification window".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#padding".to_string(),
        },
        MakoOption {
            name: "border-size".to_string(),
            option_type: "integer".to_string(),
            default: Some("1".to_string()),
            description: "Border size in pixels".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#border-size".to_string(),
        },
        MakoOption {
            name: "border-color".to_string(),
            option_type: "color".to_string(),
            default: Some("#33ffffff".to_string()),
            description: "Border color for notifications".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#border-color".to_string(),
        },
        MakoOption {
            name: "border-radius".to_string(),
            option_type: "integer".to_string(),
            default: Some("0".to_string()),
            description: "Border radius in pixels".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#border-radius".to_string(),
        },
        MakoOption {
            name: "progress-color".to_string(),
            option_type: "color".to_string(),
            default: Some("over #00ff00 #00ff00".to_string()),
            description: "Color for progress indicators".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#progress-color".to_string(),
        },
        MakoOption {
            name: "icons".to_string(),
            option_type: "boolean".to_string(),
            default: Some("1".to_string()),
            description: "Show icons in notifications".to_string(),
            valid_values: Some(vec!["0".to_string(), "1".to_string()]),
            documentation_url: "https://github.com/emersion/mako#icons".to_string(),
        },
        MakoOption {
            name: "max-icon-size".to_string(),
            option_type: "integer".to_string(),
            default: Some("64".to_string()),
            description: "Maximum icon size in pixels".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#max-icon-size".to_string(),
        },
        MakoOption {
            name: "max-visible".to_string(),
            option_type: "integer".to_string(),
            default: Some("5".to_string()),
            description: "Maximum number of visible notifications".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#max-visible".to_string(),
        },
        MakoOption {
            name: "default-timeout".to_string(),
            option_type: "integer".to_string(),
            default: Some("0".to_string()),
            description: "Default timeout in milliseconds (0 = never timeout)".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#default-timeout".to_string(),
        },
        MakoOption {
            name: "ignore-timeout".to_string(),
            option_type: "boolean".to_string(),
            default: Some("0".to_string()),
            description: "Ignore timeout from notification server".to_string(),
            valid_values: Some(vec!["0".to_string(), "1".to_string()]),
            documentation_url: "https://github.com/emersion/mako#ignore-timeout".to_string(),
        },
        MakoOption {
            name: "layer".to_string(),
            option_type: "string".to_string(),
            default: Some("overlay".to_string()),
            description: "Layer for notification window".to_string(),
            valid_values: Some(vec!["background".to_string(), "bottom".to_string(), "top".to_string(), "overlay".to_string()]),
            documentation_url: "https://github.com/emersion/mako#layer".to_string(),
        },
        MakoOption {
            name: "anchor".to_string(),
            option_type: "string".to_string(),
            default: Some("top-right".to_string()),
            description: "Anchor position for notifications".to_string(),
            valid_values: Some(vec!["top-left".to_string(), "top-right".to_string(), "bottom-left".to_string(), "bottom-right".to_string(), "top-center".to_string(), "bottom-center".to_string()]),
            documentation_url: "https://github.com/emersion/mako#anchor".to_string(),
        },
        MakoOption {
            name: "sort".to_string(),
            option_type: "string".to_string(),
            default: Some("+priority".to_string()),
            description: "Sort order for notifications".to_string(),
            valid_values: Some(vec!["+priority".to_string(), "-priority".to_string(), "+time".to_string(), "-time".to_string()]),
            documentation_url: "https://github.com/emersion/mako#sort".to_string(),
        },
        MakoOption {
            name: "output".to_string(),
            option_type: "string".to_string(),
            default: None,
            description: "Output name to display notifications on".to_string(),
            valid_values: None,
            documentation_url: "https://github.com/emersion/mako#output".to_string(),
        },
        MakoOption {
            name: "group-by".to_string(),
            option_type: "string".to_string(),
            default: None,
            description: "Group notifications by app-name or app-icon".to_string(),
            valid_values: Some(vec!["app-name".to_string(), "app-icon".to_string()]),
            documentation_url: "https://github.com/emersion/mako#group-by".to_string(),
        },
        MakoOption {
            name: "markup".to_string(),
            option_type: "integer".to_string(),
            default: Some("1".to_string()),
            description: "Enable markup parsing (0=disabled, 1=full, 2=strip)".to_string(),
            valid_values: Some(vec!["0".to_string(), "1".to_string(), "2".to_string()]),
            documentation_url: "https://github.com/emersion/mako#markup".to_string(),
        },
        MakoOption {
            name: "actions".to_string(),
            option_type: "boolean".to_string(),
            default: Some("1".to_string()),
            description: "Enable action buttons in notifications".to_string(),
            valid_values: Some(vec!["0".to_string(), "1".to_string()]),
            documentation_url: "https://github.com/emersion/mako#markup".to_string(),
        },
        MakoOption {
            name: "history".to_string(),
            option_type: "boolean".to_string(),
            default: Some("0".to_string()),
            description: "Enable notification history".to_string(),
            valid_values: Some(vec!["0".to_string(), "1".to_string()]),
            documentation_url: "https://github.com/emersion/mako#history".to_string(),
        },
    ];

    if let Some(term) = search_term {
        let term_lower = term.to_lowercase();
        all_options
            .into_iter()
            .filter(|opt| {
                opt.name.to_lowercase().contains(&term_lower)
                    || opt.description.to_lowercase().contains(&term_lower)
            })
            .collect()
    } else {
        all_options
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mako_options_no_filter() {
        let options = get_mako_options(None);
        assert!(!options.is_empty());
        assert!(options.len() >= 20); // Should have many options
    }

    #[test]
    fn test_get_mako_options_with_search() {
        let options = get_mako_options(Some("color"));
        assert!(!options.is_empty());
        assert!(options.iter().any(|opt| opt.name.contains("color")));
    }

    #[test]
    fn test_get_mako_options_search_case_insensitive() {
        let options_lower = get_mako_options(Some("color"));
        let options_upper = get_mako_options(Some("COLOR"));
        assert_eq!(options_lower.len(), options_upper.len());
    }

    #[test]
    fn test_get_mako_options_search_by_description() {
        let options = get_mako_options(Some("notification"));
        assert!(!options.is_empty());
    }

    #[test]
    fn test_get_mako_options_no_results() {
        let options = get_mako_options(Some("nonexistent_option_xyz"));
        assert!(options.is_empty());
    }

    #[test]
    fn test_mako_option_structure() {
        let options = get_mako_options(None);
        let font_option = options.iter().find(|opt| opt.name == "font").unwrap();

        assert_eq!(font_option.name, "font");
        assert_eq!(font_option.option_type, "string");
        assert!(font_option.default.is_some());
        assert!(!font_option.description.is_empty());
        assert!(!font_option.documentation_url.is_empty());
    }
}
