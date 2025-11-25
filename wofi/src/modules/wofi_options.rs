use crate::models::WofiOption;

/// Get all Wofi options, optionally filtered
pub fn get_options(filter: Option<&str>) -> Vec<WofiOption> {
    let all_options = get_all_options();
    
    if let Some(filter_str) = filter {
        all_options
            .into_iter()
            .filter(|opt| {
                opt.name.contains(filter_str)
                    || opt.description.contains(filter_str)
                    || opt.option_type.contains(filter_str)
            })
            .collect()
    } else {
        all_options
    }
}

fn get_all_options() -> Vec<WofiOption> {
    vec![
        WofiOption {
            name: "width".to_string(),
            option_type: "integer".to_string(),
            default: Some("600".to_string()),
            description: "Window width in pixels".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "width".to_string(),
            cloudninja_topic: "window-dimensions".to_string(),
        },
        WofiOption {
            name: "height".to_string(),
            option_type: "integer".to_string(),
            default: Some("400".to_string()),
            description: "Window height in pixels".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "height".to_string(),
            cloudninja_topic: "window-dimensions".to_string(),
        },
        WofiOption {
            name: "location".to_string(),
            option_type: "string".to_string(),
            default: Some("center".to_string()),
            description: "Window location: center, top, bottom, left, right, or center-left, etc.".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "location".to_string(),
            cloudninja_topic: "window-positioning".to_string(),
        },
        WofiOption {
            name: "layer".to_string(),
            option_type: "string".to_string(),
            default: Some("overlay".to_string()),
            description: "Wayland layer: overlay, top, bottom, background".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "layer".to_string(),
            cloudninja_topic: "wlr-layer-shell".to_string(),
        },
        WofiOption {
            name: "anchor".to_string(),
            option_type: "string".to_string(),
            default: None,
            description: "Window anchor for wlr-layer-shell: top, bottom, left, right, or combinations".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "anchor".to_string(),
            cloudninja_topic: "wlr-layer-shell".to_string(),
        },
        WofiOption {
            name: "insensitive".to_string(),
            option_type: "boolean".to_string(),
            default: Some("false".to_string()),
            description: "Case-insensitive matching".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "insensitive".to_string(),
            cloudninja_topic: "matching-options".to_string(),
        },
        WofiOption {
            name: "fuzzy".to_string(),
            option_type: "boolean".to_string(),
            default: Some("true".to_string()),
            description: "Enable fuzzy matching".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "fuzzy".to_string(),
            cloudninja_topic: "fuzzy-matching".to_string(),
        },
        WofiOption {
            name: "levenshtein".to_string(),
            option_type: "boolean".to_string(),
            default: Some("false".to_string()),
            description: "Use Levenshtein distance for matching".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "levenshtein".to_string(),
            cloudninja_topic: "matching-options".to_string(),
        },
        WofiOption {
            name: "prefix".to_string(),
            option_type: "boolean".to_string(),
            default: Some("false".to_string()),
            description: "Prefix matching mode".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "prefix".to_string(),
            cloudninja_topic: "matching-options".to_string(),
        },
        WofiOption {
            name: "parse_action".to_string(),
            option_type: "boolean".to_string(),
            default: Some("false".to_string()),
            description: "Parse action field from desktop entries".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "parse_action".to_string(),
            cloudninja_topic: "drun-mode".to_string(),
        },
        WofiOption {
            name: "cache_file".to_string(),
            option_type: "string".to_string(),
            default: None,
            description: "Path to cache file for faster startup".to_string(),
            source: "cloudninja".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "cache_file".to_string(),
            cloudninja_topic: "performance".to_string(),
        },
        WofiOption {
            name: "mode".to_string(),
            option_type: "string".to_string(),
            default: Some("drun".to_string()),
            description: "Display mode: drun, run, ssh, dmenu, or custom".to_string(),
            source: "combined".to_string(),
            manpage_section: "MODES".to_string(),
            srht_anchor: "mode".to_string(),
            cloudninja_topic: "modes".to_string(),
        },
        WofiOption {
            name: "term".to_string(),
            option_type: "string".to_string(),
            default: None,
            description: "Terminal command to use for executing commands".to_string(),
            source: "sr.ht".to_string(),
            manpage_section: "OPTIONS".to_string(),
            srht_anchor: "term".to_string(),
            cloudninja_topic: "terminal-execution".to_string(),
        },
    ]
}

