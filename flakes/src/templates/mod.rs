pub mod package;
pub mod devshell;
pub mod nixos;
pub mod multi;

pub use package::package_template;
pub use devshell::devshell_template;
pub use nixos::nixos_template;
pub use multi::multi_template;

use crate::models::scaffold_result::InputSpec;

pub fn render_template_with_inputs(
    template_type: crate::models::scaffold_result::TemplateType,
    name: &str,
    description: &str,
    version: &str,
    custom_inputs: &[InputSpec],
) -> String {
    match template_type {
        crate::models::scaffold_result::TemplateType::Package => {
            package_template(name, description, version, custom_inputs)
        }
        crate::models::scaffold_result::TemplateType::DevShell => {
            devshell_template(name, description, custom_inputs)
        }
        crate::models::scaffold_result::TemplateType::NixOS => {
            nixos_template(name, description, custom_inputs)
        }
        crate::models::scaffold_result::TemplateType::Multi => {
            multi_template(name, description, version, custom_inputs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_template() {
        let result = package_template("test-package", "Test description", "1.0.0", &[]);
        assert!(result.contains("test-package"));
        assert!(result.contains("Test description"));
        assert!(result.contains("packages ="));
        assert!(result.contains("nixpkgs"));
        assert!(result.contains("1.0.0"));
    }

    #[test]
    fn test_devshell_template() {
        let result = devshell_template("test-dev", "Dev shell description", &[]);
        assert!(result.contains("test-dev"));
        assert!(result.contains("Dev shell description"));
        assert!(result.contains("devShells"));
        assert!(result.contains("flake-utils"));
    }

    #[test]
    fn test_nixos_template() {
        let result = nixos_template("test-module", "NixOS module description", &[]);
        assert!(result.contains("NixOS module description"));
        assert!(result.contains("nixosModules"));
    }

    #[test]
    fn test_multi_template() {
        let result = multi_template("test-multi", "Multi output description", "1.0.0", &[]);
        assert!(result.contains("test-multi"));
        assert!(result.contains("Multi output description"));
        assert!(result.contains("packages"));
        assert!(result.contains("apps"));
        assert!(result.contains("devShells"));
    }

    #[test]
    fn test_package_template_with_custom_inputs() {
        use crate::models::scaffold_result::InputSpec;
        let custom_inputs = vec![
            InputSpec {
                name: "flake-utils".to_string(),
                url: "github:numtide/flake-utils".to_string(),
                flake: None,
            },
            InputSpec {
                name: "rust-overlay".to_string(),
                url: "github:oxalica/rust-overlay".to_string(),
                flake: None,
            },
        ];
        let result = package_template("test-pkg", "Test", "1.0.0", &custom_inputs);
        assert!(result.contains("flake-utils"));
        assert!(result.contains("rust-overlay"));
        assert!(result.contains("github:numtide/flake-utils"));
        assert!(result.contains("github:oxalica/rust-overlay"));
    }

    #[test]
    fn test_devshell_template_with_custom_inputs() {
        use crate::models::scaffold_result::InputSpec;
        let custom_inputs = vec![
            InputSpec {
                name: "rust-overlay".to_string(),
                url: "github:oxalica/rust-overlay".to_string(),
                flake: None,
            },
        ];
        let result = devshell_template("test-dev", "Test", &custom_inputs);
        assert!(result.contains("rust-overlay"));
        assert!(result.contains("rust-overlay.url"));
        assert!(result.contains("self"));
        assert!(result.contains("nixpkgs"));
        assert!(result.contains("flake-utils"));
        assert!(result.contains("rust-overlay"));
    }

    #[test]
    fn test_render_template_with_inputs() {
        use crate::models::scaffold_result::{TemplateType, InputSpec};
        let custom_inputs = vec![
            InputSpec {
                name: "flake-utils".to_string(),
                url: "github:numtide/flake-utils".to_string(),
                flake: None,
            },
        ];
        let result = render_template_with_inputs(
            TemplateType::Package,
            "test",
            "desc",
            "1.0.0",
            &custom_inputs,
        );
        assert!(result.contains("test"));
        assert!(result.contains("desc"));
        assert!(result.contains("1.0.0"));
    }
}
