use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

pub struct TemplateRenderer;

impl TemplateRenderer {
    pub fn render_template(template: &str, name: &str, description: &str) -> String {
        template
            .replace("{{NAME}}", name)
            .replace("{{DESCRIPTION}}", description)
    }

    pub fn validate_flake_path(path: &str) -> Result<PathBuf> {
        let path_buf = PathBuf::from(path);
        
        if path_buf.exists() && !path_buf.is_dir() && !path_buf.ends_with("flake.nix") {
            anyhow::bail!("Path exists but is not a directory or flake.nix file");
        }

        Ok(path_buf)
    }

    pub fn generate_flake_file(path: &Path, content: &str, overwrite: bool) -> Result<String> {
        let flake_path = if path.is_dir() {
            path.join("flake.nix")
        } else if path.ends_with("flake.nix") {
            path.to_path_buf()
        } else {
            path.join("flake.nix")
        };

        if flake_path.exists() && !overwrite {
            anyhow::bail!("flake.nix already exists at {:?}. Set overwrite=true to replace.", flake_path);
        }

        if let Some(parent) = flake_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory: {:?}", parent))?;
        }

        fs::write(&flake_path, content)
            .context(format!("Failed to write flake.nix to {:?}", flake_path))?;

        Ok(flake_path.to_string_lossy().to_string())
    }

    pub fn generate_project_structure(base_path: &Path, name: &str, description: &str, author: Option<&str>) -> Result<Vec<String>> {
        let mut created_files = Vec::new();

        fs::create_dir_all(base_path)
            .context(format!("Failed to create base directory: {:?}", base_path))?;

        let gitignore_path = base_path.join(".gitignore");
        if !gitignore_path.exists() {
            let gitignore_content = "/result\nresult-*\n.direnv\n.envrc\n*.swp\n*.swo\n*~\n";
            fs::write(&gitignore_path, gitignore_content)
                .context("Failed to write .gitignore")?;
            created_files.push(gitignore_path.to_string_lossy().to_string());
        }

        let src_dir = base_path.join("src");
        if !src_dir.exists() {
            fs::create_dir_all(&src_dir)
                .context("Failed to create src directory")?;
            created_files.push(src_dir.to_string_lossy().to_string());
        }

        let readme_path = base_path.join("README.md");
        if !readme_path.exists() {
            let author_line = author.map(|a| format!("\nAuthor: {}\n", a)).unwrap_or_default();
            let readme_content = format!(
                "# {}\n\n{}\n{}\n\n## Building\n\n```bash\nnix build\n```\n\n## Development\n\n```bash\nnix develop\n```\n",
                name, description, author_line
            );
            fs::write(&readme_path, readme_content)
                .context("Failed to write README.md")?;
            created_files.push(readme_path.to_string_lossy().to_string());
        }

        let default_nix_path = base_path.join("default.nix");
        if !default_nix_path.exists() {
            let default_nix_content = format!(
                r#"# This file is kept for compatibility with non-flake Nix commands
# The main flake.nix is the source of truth

(import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz")) {{
  # Add your package configuration here
}}
"#
            );
            fs::write(&default_nix_path, default_nix_content)
                .context("Failed to write default.nix")?;
            created_files.push(default_nix_path.to_string_lossy().to_string());
        }

        Ok(created_files)
    }

    pub fn add_output_to_existing_flake(flake_path: &Path, output_name: &str, output_code: &str) -> Result<String> {
        let content = fs::read_to_string(flake_path)
            .context(format!("Failed to read existing flake.nix: {:?}", flake_path))?;

        if content.contains(&format!("{} =", output_name)) {
            anyhow::bail!("Output '{}' already exists in flake.nix", output_name);
        }

        let updated_content = if let Some(outputs_pos) = content.find("outputs =") {
            let after_outputs = &content[outputs_pos + "outputs =".len()..];
            if let Some(open_brace) = after_outputs.find('{') {
                let brace_pos = outputs_pos + "outputs =".len() + open_brace;
                let before_brace = &content[..brace_pos + 1];
                let after_brace = &content[brace_pos + 1..];
                if let Some(close_brace) = after_brace.rfind('}') {
                    let inside_outputs = &after_brace[..close_brace];
                    let indent = if inside_outputs.trim().is_empty() { "    " } else { "\n    " };
                    format!("{}{}{}{}{}", before_brace, inside_outputs, indent, output_code, &after_brace[close_brace..])
                } else {
                    format!("{}\n    {}\n  }}", content.trim_end(), output_code)
                }
            } else {
                format!("{}\n    {}\n  }}", content.trim_end(), output_code)
            }
        } else {
            format!("{}\n\n  outputs = {{\n    {}\n  }};\n", content.trim_end(), output_code)
        };

        let updated_content_clone = updated_content.clone();
        fs::write(flake_path, updated_content)
            .context("Failed to write updated flake.nix")?;

        Ok(updated_content_clone)
    }

    pub fn add_input_to_existing_flake(flake_path: &Path, input_name: &str, input_url: &str) -> Result<String> {
        let content = fs::read_to_string(flake_path)
            .context(format!("Failed to read existing flake.nix: {:?}", flake_path))?;

        if content.contains(&format!("{}.url", input_name)) {
            anyhow::bail!("Input '{}' already exists in flake.nix", input_name);
        }

        let updated_content = if let Some(inputs_pos) = content.find("inputs =") {
            let after_inputs = &content[inputs_pos + "inputs =".len()..];
            if let Some(open_brace) = after_inputs.find('{') {
                let brace_pos = inputs_pos + "inputs =".len() + open_brace;
                let before_brace = &content[..brace_pos + 1];
                let after_brace = &content[brace_pos + 1..];
                if let Some(close_brace) = after_brace.rfind('}') {
                    let inside_inputs = &after_brace[..close_brace];
                    let indent = if inside_inputs.trim().is_empty() { "    " } else { "\n    " };
                    format!("{}{}{}    {}.url = \"{}\";{}", before_brace, inside_inputs, indent, input_name, input_url, &after_brace[close_brace..])
                } else {
                    format!("{}\n    {}.url = \"{}\";\n  }}", content.trim_end(), input_name, input_url)
                }
            } else {
                format!("{}\n    {}.url = \"{}\";\n  }}", content.trim_end(), input_name, input_url)
            }
        } else {
            format!("{}\n\n  inputs = {{\n    {}.url = \"{}\";\n  }};\n", content.trim_end(), input_name, input_url)
        };

        let updated_content_clone = updated_content.clone();
        fs::write(flake_path, updated_content)
            .context("Failed to write updated flake.nix")?;

        Ok(updated_content_clone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_render_template() {
        let template = "Name: {{NAME}}, Description: {{DESCRIPTION}}";
        let result = TemplateRenderer::render_template(template, "test", "test desc");
        assert_eq!(result, "Name: test, Description: test desc");
    }

    #[test]
    fn test_validate_flake_path() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_string_lossy().to_string();
        
        let result = TemplateRenderer::validate_flake_path(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_flake_file() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("test-flake");
        let content = "{ description = \"test\"; }";

        let result = TemplateRenderer::generate_flake_file(&flake_path, content, false);
        assert!(result.is_ok());
        
        let file_path = result.unwrap();
        assert!(Path::new(&file_path).exists());
        
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, content);
    }

    #[test]
    fn test_generate_flake_file_no_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path();
        let content = "{ description = \"test\"; }";

        TemplateRenderer::generate_flake_file(flake_path, content, false).unwrap();
        
        let result = TemplateRenderer::generate_flake_file(flake_path, "different content", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_flake_file_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path();
        let content1 = "{ description = \"test1\"; }";
        let content2 = "{ description = \"test2\"; }";

        TemplateRenderer::generate_flake_file(flake_path, content1, false).unwrap();
        TemplateRenderer::generate_flake_file(flake_path, content2, true).unwrap();
        
        let file_path = flake_path.join("flake.nix");
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, content2);
    }

    #[test]
    fn test_generate_project_structure() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test-project");

        let result = TemplateRenderer::generate_project_structure(&base_path, "test", "Test project", None);
        assert!(result.is_ok());
        
        let created = result.unwrap();
        assert!(!created.is_empty());
        assert!(base_path.join(".gitignore").exists());
        assert!(base_path.join("src").exists());
        assert!(base_path.join("README.md").exists());
    }

    #[test]
    fn test_add_output_to_existing_flake() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  outputs = { self, nixpkgs }: {
    packages = {};
  };
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();
        
        let result = TemplateRenderer::add_output_to_existing_flake(&flake_path, "devShells", "devShells = {};");
        assert!(result.is_ok());
        
        let updated = fs::read_to_string(&flake_path).unwrap();
        assert!(updated.contains("devShells = {};"));
        assert!(updated.contains("packages = {};"));
    }

    #[test]
    fn test_add_input_to_existing_flake() {
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
        
        let result = TemplateRenderer::add_input_to_existing_flake(&flake_path, "flake-utils", "github:numtide/flake-utils");
        assert!(result.is_ok());
        
        let updated = fs::read_to_string(&flake_path).unwrap();
        assert!(updated.contains("flake-utils.url"));
        assert!(updated.contains("github:numtide/flake-utils"));
    }

    #[test]
    fn test_add_input_to_flake_without_inputs() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  outputs = { self, nixpkgs }: {};
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();
        
        let result = TemplateRenderer::add_input_to_existing_flake(&flake_path, "nixpkgs", "github:NixOS/nixpkgs");
        assert!(result.is_ok());
        
        let updated = fs::read_to_string(&flake_path).unwrap();
        assert!(updated.contains("inputs ="));
        assert!(updated.contains("nixpkgs.url"));
    }

    #[test]
    fn test_add_output_preserves_existing() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  outputs = { self, nixpkgs }: {
    packages = {};
  };
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();
        
        let result = TemplateRenderer::add_output_to_existing_flake(&flake_path, "devShells", "devShells = {};");
        assert!(result.is_ok());
        
        let updated = fs::read_to_string(&flake_path).unwrap();
        assert!(updated.contains("devShells = {};"));
        assert!(updated.contains("packages = {};"));
    }

    #[test]
    fn test_add_output_duplicate_error() {
        let temp_dir = TempDir::new().unwrap();
        let flake_path = temp_dir.path().join("flake.nix");
        
        let initial_content = r#"{
  description = "test";
  outputs = { self, nixpkgs }: {
    packages = {};
  };
}"#;
        
        fs::write(&flake_path, initial_content).unwrap();
        
        let result = TemplateRenderer::add_output_to_existing_flake(&flake_path, "packages", "packages = {};");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_generate_project_structure_with_author() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path().join("test-project");

        let result = TemplateRenderer::generate_project_structure(&base_path, "test", "Test project", Some("John Doe"));
        assert!(result.is_ok());
        
        let created = result.unwrap();
        assert!(base_path.join(".gitignore").exists());
        assert!(base_path.join("src").exists());
        assert!(base_path.join("README.md").exists());
        assert!(base_path.join("default.nix").exists());
        
        let readme = fs::read_to_string(base_path.join("README.md")).unwrap();
        assert!(readme.contains("John Doe"));
    }
}
