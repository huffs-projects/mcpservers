use crate::models::WofiStyleRule;
use std::collections::HashMap;

/// Get Wofi style rules, optionally filtered by selector
pub fn get_styles(selector: Option<&str>) -> Vec<WofiStyleRule> {
    let all_styles = get_all_styles();
    
    if let Some(sel) = selector {
        all_styles
            .into_iter()
            .filter(|s| s.selector.contains(sel))
            .collect()
    } else {
        all_styles
    }
}

fn get_all_styles() -> Vec<WofiStyleRule> {
    vec![
        WofiStyleRule {
            selector: "window".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("margin".to_string(), "5px".to_string());
                m.insert("border-radius".to_string(), "10px".to_string());
                m
            },
            description: "Main window container".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "box".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("padding".to_string(), "5px".to_string());
                m
            },
            description: "Box container element".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "input".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("margin".to_string(), "5px".to_string());
                m.insert("border-radius".to_string(), "5px".to_string());
                m.insert("padding".to_string(), "5px".to_string());
                m
            },
            description: "Input field for search text".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "list".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("margin".to_string(), "5px".to_string());
                m
            },
            description: "List container for results".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "listview".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("padding".to_string(), "5px".to_string());
                m
            },
            description: "List view container".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "entry".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("padding".to_string(), "5px".to_string());
                m
            },
            description: "Individual list entry".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "category".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("padding".to_string(), "5px".to_string());
                m
            },
            description: "Category header".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "entry:selected".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("background-color".to_string(), "#4a9eff".to_string());
                m
            },
            description: "Selected entry styling".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "entry:active".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("background-color".to_string(), "#3a8eef".to_string());
                m
            },
            description: "Active entry styling".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "entry:expanded".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("background-color".to_string(), "#5aaeff".to_string());
                m
            },
            description: "Expanded entry styling".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "category-button".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("padding".to_string(), "5px".to_string());
                m
            },
            description: "Category button styling".to_string(),
            source: "sr.ht".to_string(),
        },
        WofiStyleRule {
            selector: "placeholder".to_string(),
            properties: {
                let mut m = HashMap::new();
                m.insert("color".to_string(), "#888888".to_string());
                m
            },
            description: "Placeholder text styling".to_string(),
            source: "cloudninja".to_string(),
        },
    ]
}

