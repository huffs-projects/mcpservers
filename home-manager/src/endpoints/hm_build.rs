use crate::models::BuildResult;
use crate::utils::{nix, security};
use anyhow::{Context, Result};
use std::path::Path;
use tracing::{debug, info};

pub async fn build_config(
    config_path: &Path,
    dry_run: bool,
    check_deprecated: bool,
) -> Result<BuildResult> {
    debug!(
        "Building config: path={}, dry_run={}, check_deprecated={}",
        config_path.display(),
        dry_run,
        check_deprecated
    );

    security::validate_path(config_path)
        .context("Invalid config path")?;
    
    security::validate_file_extension(config_path, &["nix"])
        .context("Config file must have .nix extension")?;

    if !config_path.exists() {
        anyhow::bail!("Configuration file does not exist: {}", config_path.display());
    }

    if !nix::check_home_manager_installed().await {
        anyhow::bail!("home-manager command not found. Please install Home-Manager first.");
    }

    let mut args = vec![];

    if dry_run {
        args.push("--dry-run");
    }

    if check_deprecated {
        args.push("--check");
    }

    args.push("switch");

    let (success, logs, mut errors, mut warnings) = nix::run_home_manager_command(config_path, &args)
        .await
        .context("Failed to execute home-manager build")?;

    // Enhance error and warning extraction
    let parsed_errors = parse_build_errors(&logs);
    let parsed_warnings = parse_build_warnings(&logs);
    
    errors.extend(parsed_errors);
    warnings.extend(parsed_warnings);
    
    // Remove duplicates
    errors.sort();
    errors.dedup();
    warnings.sort();
    warnings.dedup();

    let changes_detected = detect_changes(&logs);

    // Audit logging for build operations
    info!(
        "Build operation: path={}, dry_run={}, success={}, changes_detected={}",
        config_path.display(),
        dry_run,
        success,
        changes_detected
    );

    Ok(BuildResult {
        success,
        logs,
        errors,
        warnings,
        changes_detected,
    })
}

fn detect_changes(logs: &str) -> bool {
    let change_indicators = vec![
        "will be activated",
        "will be created",
        "will be modified",
        "will be removed",
        "Activating",
        "Creating",
        "Removing",
        "building",
        "copying",
        "linking",
        "symlink",
        "updating",
        "installing",
    ];
    
    change_indicators.iter().any(|indicator| {
        logs.to_lowercase().contains(&indicator.to_lowercase())
    })
}

fn parse_build_errors(logs: &str) -> Vec<String> {
    use regex::Regex;
    
    let mut errors = Vec::new();
    
    // Match error: patterns
    let error_regex = Regex::new(r"(?m)^error:\s*(.+)$")
        .expect("Error regex should be valid");
    for cap in error_regex.captures_iter(logs) {
        if let Some(error_msg) = cap.get(1) {
            errors.push(error_msg.as_str().trim().to_string());
        }
    }
    
    // Match evaluation errors
    let eval_error_regex = Regex::new(r"(?m)evaluation\s+error:\s*(.+?)(?:\n|$)")
        .expect("Eval error regex should be valid");
    for cap in eval_error_regex.captures_iter(logs) {
        if let Some(error_msg) = cap.get(1) {
            errors.push(error_msg.as_str().trim().to_string());
        }
    }
    
    // Match syntax errors
    let syntax_error_regex = Regex::new(r"(?m)syntax\s+error.*?:\s*(.+?)(?:\n|$)")
        .expect("Syntax error regex should be valid");
    for cap in syntax_error_regex.captures_iter(logs) {
        if let Some(error_msg) = cap.get(1) {
            errors.push(error_msg.as_str().trim().to_string());
        }
    }
    
    errors
}

fn parse_build_warnings(logs: &str) -> Vec<String> {
    use regex::Regex;
    
    let mut warnings = Vec::new();
    
    // Match warning: patterns
    let warning_regex = Regex::new(r"(?m)^warning:\s*(.+)$")
        .expect("Warning regex should be valid");
    for cap in warning_regex.captures_iter(logs) {
        if let Some(warning_msg) = cap.get(1) {
            warnings.push(warning_msg.as_str().trim().to_string());
        }
    }
    
    // Match deprecated warnings
    let deprecated_regex = Regex::new(r"(?m)deprecated.*?:\s*(.+?)(?:\n|$)")
        .expect("Deprecated regex should be valid");
    for cap in deprecated_regex.captures_iter(logs) {
        if let Some(warning_msg) = cap.get(1) {
            warnings.push(warning_msg.as_str().trim().to_string());
        }
    }
    
    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_changes() {
        assert!(detect_changes("will be activated"));
        assert!(detect_changes("will be created"));
        assert!(detect_changes("will be modified"));
        assert!(detect_changes("will be removed"));
        assert!(detect_changes("Activating home-manager"));
        assert!(detect_changes("Creating symlink"));
        assert!(detect_changes("Removing old files"));
        assert!(!detect_changes("No changes detected"));
    }

    #[tokio::test]
    async fn test_build_config_file_not_exists() {
        let path = Path::new("/nonexistent/config.nix");
        let result = build_config(path, true, true).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("does not exist") || err_msg.contains("Invalid"));
    }
}
