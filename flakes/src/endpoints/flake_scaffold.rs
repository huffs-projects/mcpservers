use serde::{Deserialize, Serialize};
use warp::Reply;
use anyhow::Result;
use crate::models::{ScaffoldResult, ScaffoldType, TemplateType};
use crate::utils::{TemplateRenderer, NixCommand};
use crate::templates::{package_template, devshell_template, nixos_template, multi_template};

#[derive(Debug, Deserialize)]
pub struct FlakeScaffoldRequest {
    pub scaffold_type: ScaffoldType,
    pub template: TemplateType,
    pub target_path: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub inputs: Option<Vec<crate::models::scaffold_result::InputSpec>>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FlakeScaffoldResponse {
    pub result: ScaffoldResult,
}

pub async fn handle_flake_scaffold(req: FlakeScaffoldRequest) -> Result<impl Reply, warp::Rejection> {
    let result = handle_flake_scaffold_internal(req).await
        .map_err(|e| warp::reject::custom(EndpointError::ScaffoldError(e.to_string())))?;

    let response = FlakeScaffoldResponse { result };
    Ok(warp::reply::json(&response))
}

pub async fn handle_flake_scaffold_internal(req: FlakeScaffoldRequest) -> anyhow::Result<ScaffoldResult> {
    let name = req.name.as_deref().unwrap_or("my-flake");
    let description = req.description.as_deref().unwrap_or("A Nix flake");
    let version = req.version.as_deref().unwrap_or("0.1.0");
    let overwrite = req.overwrite.unwrap_or(false);
    let custom_inputs = req.inputs.as_deref().unwrap_or(&[]);

    let path_buf = TemplateRenderer::validate_flake_path(&req.target_path)?;
    let mut files_created = Vec::new();
    let mut logs = String::new();
    let mut errors = Vec::new();

    let template_content = if req.scaffold_type != ScaffoldType::AddInput {
        Some(crate::templates::render_template_with_inputs(
            req.template.clone(),
            name,
            description,
            version,
            custom_inputs,
        ))
    } else {
        None
    };

    match req.scaffold_type {
        ScaffoldType::Init => {
            match TemplateRenderer::generate_project_structure(&path_buf, name, description, req.author.as_deref()) {
                Ok(mut created) => {
                    files_created.append(&mut created);
                    logs.push_str("Created project structure\n");
                }
                Err(e) => {
                    errors.push(format!("Failed to create project structure: {}", e));
                }
            }

            if let Some(ref content) = template_content {
                match TemplateRenderer::generate_flake_file(&path_buf, content, overwrite) {
                    Ok(file_path) => {
                        files_created.push(file_path.clone());
                        logs.push_str(&format!("Created flake.nix at {}\n", file_path));
                        
                        match NixCommand::flake_update(&path_buf.to_string_lossy()) {
                            Ok(update_logs) => {
                                logs.push_str(&format!("Generated flake.lock\n"));
                                logs.push_str(&update_logs);
                                if path_buf.join("flake.lock").exists() {
                                    files_created.push(path_buf.join("flake.lock").to_string_lossy().to_string());
                                }
                            }
                            Err(e) => {
                                logs.push_str(&format!("Warning: Could not generate flake.lock: {}\n", e));
                            }
                        }

                        match NixCommand::flake_check(&path_buf.to_string_lossy()) {
                            Ok((check_success, check_logs)) => {
                                if check_success {
                                    logs.push_str("Flake validation passed\n");
                                } else {
                                    logs.push_str(&format!("Flake validation warnings: {}\n", check_logs));
                                }
                            }
                            Err(e) => {
                                logs.push_str(&format!("Warning: Could not validate flake: {}\n", e));
                            }
                        }
                    }
                    Err(e) => {
                        errors.push(format!("Failed to create flake.nix: {}", e));
                    }
                }
            }
        }
        ScaffoldType::Generate => {
            if let Some(ref content) = template_content {
                match TemplateRenderer::generate_flake_file(&path_buf, content, overwrite) {
                    Ok(file_path) => {
                        files_created.push(file_path.clone());
                        logs.push_str(&format!("Generated flake.nix at {}\n", file_path));
                        
                        if let Some(parent) = path_buf.parent() {
                            match NixCommand::flake_update(&parent.to_string_lossy()) {
                                Ok(update_logs) => {
                                    logs.push_str("Generated flake.lock\n");
                                    logs.push_str(&update_logs);
                                    if parent.join("flake.lock").exists() {
                                        files_created.push(parent.join("flake.lock").to_string_lossy().to_string());
                                    }
                                }
                                Err(e) => {
                                    logs.push_str(&format!("Warning: Could not generate flake.lock: {}\n", e));
                                }
                            }

                            match NixCommand::flake_check(&parent.to_string_lossy()) {
                                Ok((check_success, check_logs)) => {
                                    if check_success {
                                        logs.push_str("Flake validation passed\n");
                                    } else {
                                        logs.push_str(&format!("Flake validation warnings: {}\n", check_logs));
                                    }
                                }
                                Err(e) => {
                                    logs.push_str(&format!("Warning: Could not validate flake: {}\n", e));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        errors.push(format!("Failed to generate flake.nix: {}", e));
                    }
                }
            }
        }
        ScaffoldType::AddOutput => {
            let flake_path = if path_buf.ends_with("flake.nix") {
                path_buf.clone()
            } else {
                path_buf.join("flake.nix")
            };

            if !flake_path.exists() {
                errors.push(format!("flake.nix not found at {:?}", flake_path));
            } else {
                let (output_name, output_code) = match req.template {
                    TemplateType::Package => {
                        let code = format!(
                            "packages = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all (system: {{\n      default = nixpkgs.legacyPackages.${{system}}.stdenv.mkDerivation {{\n        pname = \"{}\";\n        version = \"{}\";\n        src = ./.;\n      }};\n    }});",
                            name, version
                        );
                        ("packages", code)
                    }
                    TemplateType::DevShell => {
                        let code = format!(
                            "devShells = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all (system: {{\n      default = nixpkgs.legacyPackages.${{system}}.mkShell {{\n        name = \"{}\";\n        buildInputs = [];\n      }};\n    }});",
                            name
                        );
                        ("devShells", code)
                    }
                    TemplateType::NixOS => {
                        let code = "nixosModules.default = { config, pkgs, ... }: {\n      # NixOS module configuration\n    };".to_string();
                        ("nixosModules", code)
                    }
                    TemplateType::Multi => {
                        let code = format!(
                            "packages = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all (system: {{\n      default = nixpkgs.legacyPackages.${{system}}.stdenv.mkDerivation {{\n        pname = \"{}\";\n        version = \"{}\";\n        src = ./.;\n      }};\n    }});\n    apps = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all (system: {{\n      default = {{ type = \"app\"; program = \"${{self.packages.${{system}}.default}}/bin/{}\"; }};\n    }});\n    devShells = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all (system: {{\n      default = nixpkgs.legacyPackages.${{system}}.mkShell {{\n        name = \"{}\";\n        buildInputs = [];\n      }};\n    }});",
                            name, version, name, name
                        );
                        ("packages", code)
                    }
                };

                match TemplateRenderer::add_output_to_existing_flake(&flake_path, output_name, &output_code) {
                    Ok(_updated_content) => {
                        logs.push_str(&format!("Added {} output to existing flake.nix\n", output_name));
                        files_created.push(flake_path.to_string_lossy().to_string());
                    }
                    Err(e) => {
                        errors.push(format!("Failed to add output: {}", e));
                    }
                }
            }
        }
        ScaffoldType::AddInput => {
            let flake_path = if path_buf.ends_with("flake.nix") {
                path_buf.clone()
            } else {
                path_buf.join("flake.nix")
            };

            if !flake_path.exists() {
                errors.push(format!("flake.nix not found at {:?}", flake_path));
            } else if custom_inputs.is_empty() {
                errors.push("No inputs specified to add".to_string());
            } else {
                for input in custom_inputs {
                    match TemplateRenderer::add_input_to_existing_flake(&flake_path, &input.name, &input.url) {
                        Ok(_) => {
                            logs.push_str(&format!("Added input {} to existing flake.nix\n", input.name));
                        }
                        Err(e) => {
                            errors.push(format!("Failed to add input {}: {}", input.name, e));
                        }
                    }
                }
                if errors.is_empty() {
                    files_created.push(flake_path.to_string_lossy().to_string());
                }
            }
        }
    }

    let success = errors.is_empty();
    let flake_content = if success && (req.scaffold_type == ScaffoldType::Generate || req.scaffold_type == ScaffoldType::Init) {
        template_content
    } else {
        None
    };

    let validation_passed = if success && (req.scaffold_type == ScaffoldType::Generate || req.scaffold_type == ScaffoldType::Init) {
        let check_path = if path_buf.is_dir() {
            path_buf.to_string_lossy().to_string()
        } else if let Some(parent) = path_buf.parent() {
            parent.to_string_lossy().to_string()
        } else {
            path_buf.to_string_lossy().to_string()
        };
        
        match NixCommand::flake_check(&check_path) {
            Ok((check_success, check_logs)) => {
                if !check_success {
                    logs.push_str(&format!("Flake validation: {}\n", check_logs));
                }
                Some(check_success)
            }
            Err(_) => None,
        }
    } else {
        None
    };

    Ok(ScaffoldResult {
        success,
        files_created,
        flake_content,
        logs: if logs.is_empty() { "No operations performed".to_string() } else { logs },
        errors,
        validation_passed,
    })
}

#[derive(Debug)]
pub enum EndpointError {
    ScaffoldError(String),
}

impl warp::reject::Reject for EndpointError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ScaffoldType, TemplateType};
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_scaffold_init_package() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("test-project").to_string_lossy().to_string();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::Init,
            template: TemplateType::Package,
            target_path,
            name: Some("test-package".to_string()),
            description: Some("Test package".to_string()),
            inputs: None,
            overwrite: Some(false),
            version: Some("1.0.0".to_string()),
            author: Some("Test Author".to_string()),
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        assert!(!result.files_created.is_empty());
        assert!(result.flake_content.is_some());
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_scaffold_generate_devshell() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("flake.nix").to_string_lossy().to_string();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::Generate,
            template: TemplateType::DevShell,
            target_path,
            name: Some("dev-env".to_string()),
            description: Some("Development environment".to_string()),
            inputs: None,
            overwrite: Some(false),
            version: None,
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        assert_eq!(result.files_created.len(), 1);
        assert!(result.flake_content.is_some());
        assert!(result.flake_content.unwrap().contains("devShells"));
    }

    #[tokio::test]
    async fn test_scaffold_add_output() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  outputs = { self, nixpkgs }: {
    packages = {};
  };
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::AddOutput,
            template: TemplateType::DevShell,
            target_path: flake_path.to_string_lossy().to_string(),
            name: Some("test-dev".to_string()),
            description: None,
            inputs: None,
            overwrite: None,
            version: Some("1.0.0".to_string()),
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        
        let updated_content = fs::read_to_string(&flake_path).unwrap();
        assert!(updated_content.contains("devShells"));
        assert!(updated_content.contains("packages"));
    }

    #[tokio::test]
    async fn test_scaffold_add_output_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("nonexistent").join("flake.nix");

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::AddOutput,
            template: TemplateType::Package,
            target_path: flake_path.to_string_lossy().to_string(),
            name: None,
            description: None,
            inputs: None,
            overwrite: None,
            version: None,
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(!result.success);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_scaffold_add_input() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };
  outputs = { self, nixpkgs }: {};
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();

        use crate::models::scaffold_result::InputSpec;
        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::AddInput,
            template: TemplateType::Package,
            target_path: flake_path.to_string_lossy().to_string(),
            name: None,
            description: None,
            inputs: Some(vec![
                InputSpec {
                    name: "flake-utils".to_string(),
                    url: "github:numtide/flake-utils".to_string(),
                    flake: None,
                }
            ]),
            overwrite: None,
            version: None,
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        
        let updated_content = fs::read_to_string(&flake_path).unwrap();
        assert!(updated_content.contains("flake-utils.url"));
        assert!(updated_content.contains("github:numtide/flake-utils"));
    }

    #[tokio::test]
    async fn test_scaffold_init_with_custom_inputs() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("test-project").to_string_lossy().to_string();

        use crate::models::scaffold_result::InputSpec;
        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::Init,
            template: TemplateType::Multi,
            target_path,
            name: Some("test-package".to_string()),
            description: Some("Test package".to_string()),
            inputs: Some(vec![
                InputSpec {
                    name: "rust-overlay".to_string(),
                    url: "github:oxalica/rust-overlay".to_string(),
                    flake: None,
                }
            ]),
            overwrite: Some(false),
            version: Some("2.0.0".to_string()),
            author: Some("Test Author".to_string()),
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        assert!(!result.files_created.is_empty());
        assert!(result.flake_content.is_some());
        
        let content = result.flake_content.unwrap();
        assert!(content.contains("rust-overlay"));
        assert!(content.contains("github:oxalica/rust-overlay"));
        assert!(content.contains("2.0.0"));
    }

    #[tokio::test]
    async fn test_scaffold_generate_nixos() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("flake.nix").to_string_lossy().to_string();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::Generate,
            template: TemplateType::NixOS,
            target_path,
            name: Some("nixos-module".to_string()),
            description: Some("NixOS configuration module".to_string()),
            inputs: None,
            overwrite: Some(false),
            version: None,
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        assert!(result.flake_content.is_some());
        assert!(result.flake_content.unwrap().contains("nixosModules"));
    }

    #[tokio::test]
    async fn test_scaffold_generate_multi() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("flake.nix").to_string_lossy().to_string();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::Generate,
            template: TemplateType::Multi,
            target_path,
            name: Some("multi-flake".to_string()),
            description: Some("Multi-output flake".to_string()),
            inputs: None,
            overwrite: Some(false),
            version: Some("1.5.0".to_string()),
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        let content = result.flake_content.unwrap();
        assert!(content.contains("packages"));
        assert!(content.contains("apps"));
        assert!(content.contains("devShells"));
        assert!(content.contains("1.5.0"));
    }

    #[tokio::test]
    async fn test_scaffold_add_output_package() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  outputs = { self, nixpkgs }: {
    devShells = {};
  };
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::AddOutput,
            template: TemplateType::Package,
            target_path: flake_path.to_string_lossy().to_string(),
            name: Some("my-package".to_string()),
            description: None,
            inputs: None,
            overwrite: None,
            version: Some("1.2.3".to_string()),
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        
        let updated_content = fs::read_to_string(&flake_path).unwrap();
        assert!(updated_content.contains("packages"));
        assert!(updated_content.contains("my-package"));
        assert!(updated_content.contains("1.2.3"));
        assert!(updated_content.contains("devShells"));
    }

    #[tokio::test]
    async fn test_scaffold_add_output_nixos() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  outputs = { self, nixpkgs }: {
    packages = {};
  };
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::AddOutput,
            template: TemplateType::NixOS,
            target_path: flake_path.to_string_lossy().to_string(),
            name: None,
            description: None,
            inputs: None,
            overwrite: None,
            version: None,
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        
        let updated_content = fs::read_to_string(&flake_path).unwrap();
        assert!(updated_content.contains("nixosModules"));
        assert!(updated_content.contains("packages"));
    }

    #[tokio::test]
    async fn test_scaffold_add_input_multiple() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };
  outputs = { self, nixpkgs }: {};
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();

        use crate::models::scaffold_result::InputSpec;
        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::AddInput,
            template: TemplateType::Package,
            target_path: flake_path.to_string_lossy().to_string(),
            name: None,
            description: None,
            inputs: Some(vec![
                InputSpec {
                    name: "flake-utils".to_string(),
                    url: "github:numtide/flake-utils".to_string(),
                    flake: None,
                },
                InputSpec {
                    name: "rust-overlay".to_string(),
                    url: "github:oxalica/rust-overlay".to_string(),
                    flake: None,
                }
            ]),
            overwrite: None,
            version: None,
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        
        let updated_content = fs::read_to_string(&flake_path).unwrap();
        assert!(updated_content.contains("flake-utils.url"));
        assert!(updated_content.contains("rust-overlay.url"));
    }

    #[tokio::test]
    async fn test_scaffold_add_input_no_inputs_error() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };
  outputs = { self, nixpkgs }: {};
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::AddInput,
            template: TemplateType::Package,
            target_path: flake_path.to_string_lossy().to_string(),
            name: None,
            description: None,
            inputs: None,
            overwrite: None,
            version: None,
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(!result.success);
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].contains("No inputs specified"));
    }

    #[tokio::test]
    async fn test_scaffold_generate_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        fs::write(&flake_path, "old content").unwrap();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::Generate,
            template: TemplateType::Package,
            target_path: flake_path.to_string_lossy().to_string(),
            name: Some("new-package".to_string()),
            description: Some("New package".to_string()),
            inputs: None,
            overwrite: Some(true),
            version: Some("1.0.0".to_string()),
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(result.success);
        
        let content = fs::read_to_string(&flake_path).unwrap();
        assert!(content.contains("new-package"));
        assert!(!content.contains("old content"));
    }

    #[tokio::test]
    async fn test_scaffold_generate_no_overwrite_error() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        fs::write(&flake_path, "existing content").unwrap();

        let req = FlakeScaffoldRequest {
            scaffold_type: ScaffoldType::Generate,
            template: TemplateType::Package,
            target_path: flake_path.to_string_lossy().to_string(),
            name: Some("new-package".to_string()),
            description: Some("New package".to_string()),
            inputs: None,
            overwrite: Some(false),
            version: Some("1.0.0".to_string()),
            author: None,
            license: None,
        };

        let result = handle_flake_scaffold_internal(req).await.unwrap();
        assert!(!result.success);
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].contains("already exists"));
    }
}
