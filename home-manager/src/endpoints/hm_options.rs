use crate::models::HMOption;
use crate::utils::{cache, nix, validation};
use crate::metrics;
use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;
use tracing::{debug, warn};

static OPTIONS_CACHE: OnceLock<cache::Cache<Vec<HMOption>>> = OnceLock::new();

fn get_options_cache() -> &'static cache::Cache<Vec<HMOption>> {
    OPTIONS_CACHE.get_or_init(|| cache::Cache::new(Duration::from_secs(3600)))
}

pub async fn query_options(
    search_term: Option<&str>,
    module_name: Option<&str>,
) -> Result<Vec<HMOption>> {
    debug!("Querying options: search_term={:?}, module_name={:?}", search_term, module_name);

    if let Some(term) = search_term {
        validation::validate_string_param(term, Some(1000))
            .context("Invalid search term")?;
    }

    if let Some(module) = module_name {
        validation::validate_string_param(module, Some(500))
            .context("Invalid module name")?;
    }

    let cache = get_options_cache();
    let cache_key = "all_options".to_string();
    
    let options = if let Some(cached) = cache.get(&cache_key) {
        debug!("Using cached options");
        metrics::get_global_metrics().record_cache_hit();
        cached
    } else {
        debug!("Loading options from docs");
        metrics::get_global_metrics().record_cache_miss();
        let loaded = load_options_from_docs()?;
        cache.set(cache_key, loaded.clone());
        loaded
    };

    let filtered: Vec<HMOption> = options
        .into_iter()
        .filter(|opt| {
            let matches_search = search_term
                .map(|term| {
                    opt.name.contains(term)
                        || opt.description.to_lowercase().contains(&term.to_lowercase())
                })
                .unwrap_or(true);

            let matches_module = module_name
                .map(|module| opt.module_source.contains(module))
                .unwrap_or(true);

            matches_search && matches_module
        })
        .collect();

    Ok(filtered)
}

fn load_options_from_docs() -> Result<Vec<HMOption>> {
    let home_manager_docs_path = find_home_manager_docs()?;
    
    if home_manager_docs_path.is_none() {
        warn!("Home-Manager documentation not found, returning empty options list");
        return Ok(vec![]);
    }

    let docs_path = home_manager_docs_path
        .ok_or_else(|| anyhow::anyhow!("Home-Manager documentation not found"))?;
    debug!("Loading options from: {}", docs_path.display());

    let options = parse_options_from_docs(&docs_path)?;
    Ok(options)
}

fn find_home_manager_docs() -> Result<Option<std::path::PathBuf>> {
    let possible_paths = vec![
        "/nix/var/nix/profiles/per-user/$USER/home-manager/share/doc/home-manager/options.html",
        "~/.nix-profile/share/doc/home-manager/options.html",
        "/etc/nixos/home-manager/share/doc/home-manager/options.html",
    ];

    for path_str in possible_paths {
        let expanded = shellexpand::full(path_str)
            .map(|s| s.into_owned())
            .unwrap_or_else(|_| path_str.to_string());
        
        let path = Path::new(&expanded);
        if path.exists() {
            return Ok(Some(path.to_path_buf()));
        }
    }

    Ok(None)
}

fn parse_options_from_docs(docs_path: &Path) -> Result<Vec<HMOption>> {
    let content = std::fs::read_to_string(docs_path)
        .context("Failed to read Home-Manager documentation")?;

    let mut options = Vec::new();
    let option_regex = Regex::new(r#"<dt[^>]*><code[^>]*>([^<]+)</code></dt>"#)?;

    for cap in option_regex.captures_iter(&content) {
        if let Some(option_name) = cap.get(1) {
            let name = option_name.as_str().to_string();
            
            let description = find_description_for_option(&content, &name)
                .unwrap_or_else(|| "No description available".to_string());

            let option_type = extract_type_from_docs(&content, &name)
                .unwrap_or_else(|| infer_option_type(&name));
            
            let default = extract_default_from_docs(&content, &name);
            // Note: try_eval_default is async and would require refactoring parse_options_from_docs
            // For now, we only use documentation-based defaults
            
            let example = extract_example_from_docs(&content, &name);

            let valid_values = extract_valid_values_from_docs(&content, &name);

            let option = HMOption {
                name: name.clone(),
                option_type,
                default,
                description,
                valid_values,
                example,
                module_source: extract_module_from_name(&name),
                documentation_url: generate_documentation_url(&name),
            };

            options.push(option);
        }
    }

    Ok(options)
}

fn extract_type_from_docs(content: &str, option_name: &str) -> Option<String> {
    let pattern = format!(
        r#"<dt[^>]*><code[^>]*>{}</code></dt>.*?<code[^>]*>Type:\s*([^<]+)</code>"#,
        regex::escape(option_name)
    );
    let regex = Regex::new(&pattern).ok()?;
    
    regex.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| strip_html_tags(m.as_str()))
}

fn extract_default_from_docs(content: &str, option_name: &str) -> Option<serde_json::Value> {
    let pattern = format!(
        r#"<dt[^>]*><code[^>]*>{}</code></dt>.*?<code[^>]*>Default:\s*([^<]+)</code>"#,
        regex::escape(option_name)
    );
    let regex = Regex::new(&pattern).ok()?;
    
    regex.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| {
            let default_str = strip_html_tags(m.as_str());
            parse_default_value(&default_str)
        })
}

fn extract_example_from_docs(content: &str, option_name: &str) -> Option<String> {
    let pattern = format!(
        r#"<dt[^>]*><code[^>]*>{}</code></dt>.*?<code[^>]*>Example:\s*([^<]+)</code>"#,
        regex::escape(option_name)
    );
    let regex = Regex::new(&pattern).ok()?;
    
    regex.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| strip_html_tags(m.as_str()))
}

fn extract_valid_values_from_docs(content: &str, option_name: &str) -> Option<Vec<String>> {
    let pattern = format!(
        r#"<dt[^>]*><code[^>]*>{}</code></dt>.*?<dd[^>]*>.*?Valid values:\s*([^<]+)</dd>"#,
        regex::escape(option_name)
    );
    let regex = Regex::new(&pattern).ok()?;
    
    regex.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| {
            let values_str = strip_html_tags(m.as_str());
            values_str.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
}

fn parse_default_value(default_str: &str) -> serde_json::Value {
    let trimmed = default_str.trim();
    
    if trimmed == "true" || trimmed == "false" {
        return serde_json::json!(trimmed == "true");
    }
    
    if let Ok(num) = trimmed.parse::<i64>() {
        return serde_json::json!(num);
    }
    
    if let Ok(num) = trimmed.parse::<f64>() {
        return serde_json::json!(num);
    }
    
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        return serde_json::json!(trimmed.trim_matches('"'));
    }
    
    if trimmed == "null" {
        return serde_json::Value::Null;
    }
    
    serde_json::json!(trimmed)
}

async fn try_eval_default(option_path: &str) -> Result<Option<serde_json::Value>> {
    nix::eval_option_default(option_path).await
}

fn generate_documentation_url(option_name: &str) -> String {
    let encoded = option_name.replace('.', "-").replace('_', "-");
    format!("https://nix-community.github.io/home-manager/options.html#opt-{}", encoded)
}

fn find_description_for_option(content: &str, option_name: &str) -> Option<String> {
    let pattern = format!(r#"<dt[^>]*><code[^>]*>{}</code></dt>\s*<dd[^>]*>(.*?)</dd>"#, regex::escape(option_name));
    let regex = Regex::new(&pattern).ok()?;
    
    regex.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| {
            let html = m.as_str();
            strip_html_tags(html)
        })
}

fn strip_html_tags(html: &str) -> String {
    let tag_regex = Regex::new(r"<[^>]+>")
        .expect("HTML tag regex should be valid");
    let text = tag_regex.replace_all(html, " ");
    text.trim().to_string()
}

fn infer_option_type(option_name: &str) -> String {
    if option_name.contains("enable") || option_name.contains("Enable") {
        "boolean".to_string()
    } else if option_name.contains("package") || option_name.contains("Package") {
        "package".to_string()
    } else if option_name.contains("path") || option_name.contains("Path") {
        "path".to_string()
    } else if option_name.contains("list") || option_name.contains("List") {
        "list".to_string()
    } else if option_name.contains("attrs") || option_name.contains("Attrs") {
        "attrs".to_string()
    } else {
        "string".to_string()
    }
}

fn extract_module_from_name(option_name: &str) -> String {
    option_name
        .split('.')
        .take(2)
        .collect::<Vec<_>>()
        .join(".")
}

