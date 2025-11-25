use crate::models::ApplyResult;
use crate::utils::{DiffGenerator, FileOps, WaybarParser};
use anyhow::{Context, Result};
use json_patch::patch;
use serde_json::Value;

pub fn apply_patches(
    config_path: &str,
    css_path: Option<&str>,
    patch_json: &str,
    patch_css: Option<&str>,
    dry_run: bool,
    backup_path: Option<&str>,
) -> Result<ApplyResult> {
    let mut result = ApplyResult::new();

    // Expand and validate paths
    let expanded_config = FileOps::validate_file_path(config_path)?;
    let config_path_str = expanded_config.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid path encoding"))?;
    
    let expanded_css = if let Some(css) = css_path {
        Some(FileOps::validate_file_path(css)?)
    } else {
        None
    };
    
    let css_path_str = expanded_css.as_ref()
        .and_then(|p| p.to_str());

    // Read existing configs
    let old_json = FileOps::read_file(config_path_str)
        .with_context(|| format!("Failed to read config: {}", config_path_str))?;
    
    let old_css = if let Some(css) = css_path_str {
        Some(FileOps::read_file(css).with_context(|| format!("Failed to read CSS: {}", css))?)
    } else {
        None
    };

    // Parse patches
    let json_patch: Value = serde_json::from_str(patch_json)
        .context("Failed to parse JSON patch")?;

    // Apply JSON patch
    let mut config: Value = serde_json::from_str(&old_json)
        .context("Failed to parse existing JSON config")?;

    // If patch is an object, merge it; if it's an array, use json-patch
    let new_config = if json_patch.is_object() {
        merge_json_objects(&config, &json_patch)
    } else if json_patch.is_array() {
        // Use json-patch for RFC 6902 patches
        let patches: json_patch::Patch = serde_json::from_value(json_patch.clone())
            .context("Failed to parse JSON patch array")?;
        let mut patched = config.clone();
        patch(&mut patched, &patches)
            .context("Failed to apply JSON patch")?;
        patched
    } else {
        return Err(anyhow::anyhow!("Invalid patch format"));
    };

    let new_json = serde_json::to_string_pretty(&new_config)
        .context("Failed to serialize new config")?;

    // Generate diff
    result.diff_json = DiffGenerator::generate_json_diff(&old_json, &new_json);

    // Apply CSS patch if provided
    let new_css = if let Some(css_patch) = patch_css {
        if let Some(ref old) = old_css {
            let merged = merge_css(&old, css_patch);
            result.diff_css = Some(DiffGenerator::generate_css_diff(old, &merged));
            Some(merged)
        } else {
            result.diff_css = Some(format!("+{}\n", css_patch));
            Some(css_patch.to_string())
        }
    } else {
        None
    };

    // Extract applied modules/scripts/styles
    result.applied_modules = WaybarParser::extract_modules(&new_config);
    result.applied_scripts = WaybarParser::extract_custom_scripts(&new_config)
        .iter()
        .map(|(name, _)| name.clone())
        .collect();

    if !dry_run {
        // Create backup
        if let Some(backup_dir) = backup_path {
            let expanded_backup_dir = FileOps::expand_path(backup_dir)?;
            FileOps::ensure_directory(expanded_backup_dir.to_str().unwrap())?;
            let backup = FileOps::create_backup(config_path_str, Some(expanded_backup_dir.to_str().unwrap()))?;
            result.backup_created = true;
            result.add_log(format!("Backup created: {}", backup));
        } else {
            let backup = FileOps::create_backup(config_path_str, None)?;
            result.backup_created = true;
            result.add_log(format!("Backup created: {}", backup));
        }

        // Write files atomically
        FileOps::atomic_write(config_path_str, &new_json)
            .context("Failed to write JSON config")?;

        if let Some(css) = css_path_str {
            if let Some(ref new_css_content) = new_css {
                FileOps::atomic_write(css, new_css_content)
                    .context("Failed to write CSS")?;
            }
        }

        result.success = true;
        result.add_log("Patches applied successfully".to_string());
    } else {
        result.add_log("Dry run: no changes applied".to_string());
    }

    Ok(result)
}

fn merge_json_objects(base: &Value, patch: &Value) -> Value {
    match (base, patch) {
        (Value::Object(base_map), Value::Object(patch_map)) => {
            let mut merged = base_map.clone();
            for (key, patch_value) in patch_map {
                if let Some(base_value) = merged.get(key) {
                    if base_value.is_object() && patch_value.is_object() {
                        merged.insert(key.clone(), merge_json_objects(base_value, patch_value));
                    } else {
                        merged.insert(key.clone(), patch_value.clone());
                    }
                } else {
                    merged.insert(key.clone(), patch_value.clone());
                }
            }
            Value::Object(merged)
        }
        _ => patch.clone(),
    }
}

fn merge_css(old: &str, new: &str) -> String {
    // Simple CSS merge: append new rules
    format!("{}\n\n/* Added by patch */\n{}", old, new)
}

impl ApplyResult {
    fn add_log(&mut self, log: String) {
        // Simple log storage for now
        if !self.diff_json.is_empty() {
            self.diff_json.push_str("\n");
        }
        self.diff_json.push_str(&format!("LOG: {}\n", log));
    }
}

