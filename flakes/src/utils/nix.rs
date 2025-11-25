use std::process::Command;
use anyhow::{Result, Context};
use serde_json::Value;

pub struct NixCommand;

impl NixCommand {
    pub fn flake_metadata(flake_path: &str) -> Result<Value> {
        let output = Command::new("nix")
            .args(&["flake", "metadata", "--json", flake_path])
            .output()
            .context("Failed to execute nix flake metadata")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("nix flake metadata failed: {}", stderr);
        }

        let json: Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse nix flake metadata JSON")?;

        Ok(json)
    }

    pub fn flake_show(flake_path: &str) -> Result<Value> {
        let output = Command::new("nix")
            .args(&["flake", "show", "--json", flake_path])
            .output()
            .context("Failed to execute nix flake show")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("nix flake show failed: {}", stderr);
        }

        let json: Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse nix flake show JSON")?;

        Ok(json)
    }

    pub fn eval(flake_path: &str, expression: &str, json_output: bool) -> Result<(String, String)> {
        let mut cmd = Command::new("nix");
        cmd.arg("eval");
        
        if json_output {
            cmd.arg("--json");
        }
        
        cmd.arg(&format!("{}#{}", flake_path, expression));

        let output = cmd
            .output()
            .context("Failed to execute nix eval")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            anyhow::bail!("nix eval failed: {}", stderr);
        }

        Ok((stdout, stderr))
    }

    pub fn build(flake_path: &str, outputs: &[String], dry_run: bool) -> Result<(bool, String, Vec<String>, Vec<String>)> {
        let mut cmd = Command::new("nix");
        
        if dry_run {
            cmd.args(&["build", "--dry-run"]);
        } else {
            cmd.arg("build");
        }

        for output in outputs {
            cmd.arg(&format!("{}#{}", flake_path, output));
        }

        let output = cmd
            .output()
            .context("Failed to execute nix build")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let logs = format!("{}\n{}", stdout, stderr);

        let success = output.status.success();
        let mut errors = Vec::new();
        let mut built_paths = Vec::new();

        if !success {
            errors.push(stderr.clone());
        } else if !dry_run {
            for line in stdout.lines() {
                if line.starts_with("/nix/store/") {
                    built_paths.push(line.trim().to_string());
                }
            }
        }

        Ok((success, logs, errors, built_paths))
    }

    pub fn flake_init(flake_path: &str) -> Result<String> {
        let output = Command::new("nix")
            .args(&["flake", "init"])
            .current_dir(flake_path)
            .output()
            .context("Failed to execute nix flake init")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let logs = format!("{}\n{}", stdout, stderr);

        if !output.status.success() {
            anyhow::bail!("nix flake init failed: {}", stderr);
        }

        Ok(logs)
    }

    pub fn flake_check(flake_path: &str) -> Result<(bool, String)> {
        let output = Command::new("nix")
            .args(&["flake", "check", flake_path])
            .output()
            .context("Failed to execute nix flake check")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let logs = format!("{}\n{}", stdout, stderr);

        Ok((output.status.success(), logs))
    }

    pub fn flake_update(flake_path: &str) -> Result<String> {
        let output = Command::new("nix")
            .args(&["flake", "update"])
            .current_dir(flake_path)
            .output()
            .context("Failed to execute nix flake update")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let logs = format!("{}\n{}", stdout, stderr);

        if !output.status.success() {
            anyhow::bail!("nix flake update failed: {}", stderr);
        }

        Ok(logs)
    }
}

