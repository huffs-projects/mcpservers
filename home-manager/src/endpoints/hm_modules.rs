use crate::models::HMModule;
use crate::utils::cache;
use crate::metrics;
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;
use tracing::debug;

static MODULES_CACHE: OnceLock<cache::Cache<Vec<HMModule>>> = OnceLock::new();

fn get_modules_cache() -> &'static cache::Cache<Vec<HMModule>> {
    MODULES_CACHE.get_or_init(|| cache::Cache::new(Duration::from_secs(3600)))
}

pub async fn list_modules() -> Result<Vec<HMModule>> {
    debug!("Listing Home-Manager modules");

    let cache = get_modules_cache();
    let cache_key = "all_modules".to_string();
    
    if let Some(cached) = cache.get(&cache_key) {
        debug!("Using cached modules");
        metrics::get_global_metrics().record_cache_hit();
        return Ok(cached);
    }
    
    metrics::get_global_metrics().record_cache_miss();

    let modules_path = find_modules_directory()?;
    
    if modules_path.is_none() {
        return Ok(vec![]);
    }

    let modules_dir = modules_path
        .ok_or_else(|| anyhow::anyhow!("Home-Manager modules directory not found"))?;
    debug!("Loading modules from: {}", modules_dir.display());

    let modules = scan_modules_directory(&modules_dir)?;
    cache.set(cache_key, modules.clone());
    Ok(modules)
}

fn find_modules_directory() -> Result<Option<std::path::PathBuf>> {
    let possible_paths = vec![
        "~/.nix-profile/share/home-manager/modules",
        "/nix/var/nix/profiles/per-user/$USER/home-manager/share/home-manager/modules",
        "/etc/nixos/home-manager/modules",
    ];

    for path_str in possible_paths {
        let expanded = shellexpand::full(path_str)
            .map(|s| s.into_owned())
            .unwrap_or_else(|_| path_str.to_string());
        
        let path = Path::new(&expanded);
        if path.exists() && path.is_dir() {
            return Ok(Some(path.to_path_buf()));
        }
    }

    Ok(None)
}

fn scan_modules_directory(dir: &Path) -> Result<Vec<HMModule>> {
    let mut modules = Vec::new();

    if !dir.exists() {
        return Ok(modules);
    }

    scan_directory_recursive(dir, dir, &mut modules)?;

    Ok(modules)
}

fn scan_directory_recursive(
    base_dir: &Path,
    current_dir: &Path,
    modules: &mut Vec<HMModule>,
) -> Result<()> {
    let entries = std::fs::read_dir(current_dir)
        .with_context(|| format!("Failed to read directory: {}", current_dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            scan_directory_recursive(base_dir, &path, modules)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("nix") {
            if let Some(module) = parse_module_file(&path, base_dir)? {
                modules.push(module);
            }
        }
    }

    Ok(())
}

fn parse_module_file(file_path: &Path, base_dir: &Path) -> Result<Option<HMModule>> {
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read module file: {}", file_path.display()))?;

    let relative_path = pathdiff::diff_paths(file_path, base_dir)
        .unwrap_or_else(|| file_path.to_path_buf());

    let module_name = relative_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let imports = extract_imports(&content);
    let arguments = extract_arguments(&content);

    let module = HMModule {
        module_name: format_module_name(&relative_path),
        path: file_path.display().to_string(),
        imports,
        arguments,
        documentation_url: generate_module_documentation_url(&module_name),
    };

    Ok(Some(module))
}

fn format_module_name(relative_path: &Path) -> String {
    relative_path
        .components()
        .filter_map(|c| {
            if let std::path::Component::Normal(name) = c {
                name.to_str()
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(".")
        .trim_end_matches(".nix")
        .to_string()
}

fn generate_module_documentation_url(module_name: &str) -> String {
    format!("https://github.com/nix-community/home-manager/blob/master/modules/{}", module_name)
}

fn extract_imports(content: &str) -> Vec<String> {
    use regex::Regex;
    let import_regex = Regex::new(r#"import\s+([^\s;]+)"#)
        .expect("Import regex should be valid");
    
    import_regex
        .captures_iter(content)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str().trim_matches('"').to_string())
        .collect()
}

fn extract_arguments(content: &str) -> Vec<String> {
    use regex::Regex;
    
    let mut arguments = std::collections::HashSet::new();
    
    // Extract function arguments: { config, pkgs, lib, ... }:
    let function_arg_regex = Regex::new(r#"\{(.*?)\}\s*:"#)
        .expect("Function arg regex should be valid");
    for cap in function_arg_regex.captures_iter(content) {
        if let Some(args_str) = cap.get(1) {
            for arg in args_str.as_str().split(',') {
                let trimmed = arg.trim().split_whitespace().next().unwrap_or("").to_string();
                // Note: unwrap_or is safe here as it provides a default value
                if !trimmed.is_empty() && trimmed != "..." {
                    arguments.insert(trimmed);
                }
            }
        }
    }
    
    // Extract common Home-Manager arguments
    let common_args = vec!["config", "pkgs", "lib", "options", "home", "system"];
    for arg in common_args {
        if content.contains(arg) {
            arguments.insert(arg.to_string());
        }
    }
    
    arguments.into_iter().collect()
}

