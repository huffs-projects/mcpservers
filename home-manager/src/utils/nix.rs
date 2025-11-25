use anyhow::{Context, Result};
use std::path::Path;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, error};

pub async fn run_nix_command(args: &[&str]) -> Result<String> {
    run_nix_command_with_timeout(args, Duration::from_secs(300)).await
}

pub async fn run_nix_command_with_timeout(args: &[&str], timeout_duration: Duration) -> Result<String> {
    debug!("Running nix command: nix {}", args.join(" "));
    
    let mut cmd = Command::new("nix");
    cmd.args(args);
    
    let output = timeout(timeout_duration, cmd.output())
        .await
        .context("Nix command timed out")?
        .context("Failed to execute nix command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let exit_code = output.status.code().unwrap_or(-1);
        error!("Nix command failed with exit code {}: {}", exit_code, stderr);
        anyhow::bail!(
            "Nix command failed with exit code {}: {}",
            exit_code,
            stderr
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn run_home_manager_command(
    config_path: &Path,
    args: &[&str],
) -> Result<(bool, String, Vec<String>, Vec<String>)> {
    run_home_manager_command_with_timeout(config_path, args, Duration::from_secs(600)).await
}

pub async fn run_home_manager_command_with_timeout(
    config_path: &Path,
    args: &[&str],
    timeout_duration: Duration,
) -> Result<(bool, String, Vec<String>, Vec<String>)> {
    let mut full_args = vec!["home-manager"];
    full_args.extend(args);
    
    if let Some(path_str) = config_path.to_str() {
        full_args.push("-f");
        full_args.push(path_str);
    }

    debug!("Running home-manager command: {}", full_args.join(" "));

    let mut cmd = Command::new("home-manager");
    cmd.args(&full_args[1..]); // Skip the "home-manager" prefix

    let output = timeout(timeout_duration, cmd.output())
        .await
        .context("Home-manager command timed out")?
        .context("Failed to execute home-manager command")?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    let success = output.status.success();
    let exit_code = output.status.code();
    let logs = format!("{}\n{}", stdout, stderr);
    
    if !success {
        error!(
            "Home-manager command failed with exit code {:?}: {}",
            exit_code, stderr
        );
    }
    
    let errors = extract_errors(&logs);
    let warnings = extract_warnings(&logs);

    Ok((success, logs, errors, warnings))
}

fn extract_errors(logs: &str) -> Vec<String> {
    logs.lines()
        .filter(|line| {
            line.contains("error:") 
            || line.contains("Error:")
            || line.to_lowercase().contains("failed")
        })
        .map(|s| s.to_string())
        .collect()
}

fn extract_warnings(logs: &str) -> Vec<String> {
    logs.lines()
        .filter(|line| {
            line.contains("warning:") 
            || line.contains("Warning:")
            || line.contains("deprecated")
        })
        .map(|s| s.to_string())
        .collect()
}

pub async fn check_home_manager_installed() -> bool {
    match Command::new("home-manager")
        .arg("--version")
        .output()
        .await
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub async fn check_nix_installed() -> bool {
    match Command::new("nix")
        .arg("--version")
        .output()
        .await
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub async fn eval_option_type(option_path: &str) -> Result<Option<String>> {
    // Note: This is a placeholder implementation
    // Proper implementation would require accessing Home-Manager's option system
    // which is complex and requires nix evaluation context
    debug!("eval_option_type called for: {}", option_path);
    Ok(None)
}

pub async fn eval_option_default(option_path: &str) -> Result<Option<serde_json::Value>> {
    // Note: This is a placeholder implementation
    // Proper implementation would require accessing Home-Manager's option system
    // which is complex and requires nix evaluation context
    debug!("eval_option_default called for: {}", option_path);
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_errors() {
        let logs = "Some output\nerror: something went wrong\nMore output\nError: another error\nfailed to build";
        let errors = extract_errors(logs);
        
        assert!(errors.len() >= 3);
        assert!(errors.iter().any(|e| e.contains("error")));
        assert!(errors.iter().any(|e| e.contains("failed")));
    }

    #[test]
    fn test_extract_warnings() {
        let logs = "Some output\nwarning: deprecated option\nMore output\nWarning: another warning\ndeprecated feature";
        let warnings = extract_warnings(logs);
        
        assert!(warnings.len() >= 3);
        assert!(warnings.iter().any(|w| w.contains("warning")));
        assert!(warnings.iter().any(|w| w.contains("deprecated")));
    }

    #[test]
    fn test_extract_no_errors_or_warnings() {
        let logs = "Normal output\nEverything is fine\nNo issues here";
        let errors = extract_errors(logs);
        let warnings = extract_warnings(logs);
        
        assert_eq!(errors.len(), 0);
        assert_eq!(warnings.len(), 0);
    }

    #[tokio::test]
    async fn test_check_nix_installed() {
        let installed = check_nix_installed().await;
        assert!(installed, "Nix should be installed in test environment");
    }
}
