use std::collections::HashMap;

/// Map items to sr.ht/man/cloudninja anchors
pub fn get_doc_link(keyword: &str) -> Option<String> {
    let docs = get_doc_map();
    docs.get(keyword).cloned()
}

fn get_doc_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    // Common Wofi options
    map.insert("width".to_string(), "https://sr.ht/~scoopta/wofi/#width".to_string());
    map.insert("height".to_string(), "https://sr.ht/~scoopta/wofi/#height".to_string());
    map.insert("location".to_string(), "https://sr.ht/~scoopta/wofi/#location".to_string());
    map.insert("fuzzy".to_string(), "https://cloudninja.pw/docs/wofi.html#fuzzy-matching".to_string());
    map.insert("drun".to_string(), "https://cloudninja.pw/docs/wofi.html#drun-mode".to_string());
    map.insert("run".to_string(), "https://cloudninja.pw/docs/wofi.html#run-mode".to_string());
    map.insert("ssh".to_string(), "https://cloudninja.pw/docs/wofi.html#ssh-mode".to_string());
    map.insert("dmenu".to_string(), "https://cloudninja.pw/docs/wofi.html#dmenu-mode".to_string());
    
    map
}

/// Get comprehensive documentation string for a keyword
pub fn get_docs(keyword: &str) -> String {
    let srht_link = format!("https://sr.ht/~scoopta/wofi/#{}", keyword);
    let man_link = format!("https://github.com/SimplyCEO/wofi/man#{}", keyword);
    let cloudninja_link = format!("https://cloudninja.pw/docs/wofi.html#{}", keyword);

    format!(
        "Documentation for '{}':\n\n\
        • Official Wofi (sr.ht): {}\n\
        • Man page: {}\n\
        • CloudNinja Deep Docs: {}",
        keyword, srht_link, man_link, cloudninja_link
    )
}

