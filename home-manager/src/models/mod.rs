use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HMOption {
    pub name: String,
    pub option_type: String,
    pub default: Option<serde_json::Value>,
    pub description: String,
    pub valid_values: Option<Vec<String>>,
    pub example: Option<String>,
    pub module_source: String,
    pub documentation_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HMModule {
    pub module_name: String,
    pub path: String,
    pub imports: Vec<String>,
    pub arguments: Vec<String>,
    pub documentation_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchResult {
    pub success: bool,
    pub diff_applied: Option<String>,
    pub backup_created: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub success: bool,
    pub logs: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub changes_detected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateResult {
    pub program_name: String,
    pub snippet: String,
    pub description: String,
    pub required_options: Vec<String>,
    pub documentation_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hm_option_serialization() {
        let option = HMOption {
            name: "programs.git.enable".to_string(),
            option_type: "boolean".to_string(),
            default: Some(serde_json::json!(false)),
            description: "Enable Git program".to_string(),
            valid_values: None,
            example: Some("true".to_string()),
            module_source: "programs.git".to_string(),
            documentation_url: "https://example.com".to_string(),
        };

        let json = serde_json::to_string(&option).unwrap();
        let deserialized: HMOption = serde_json::from_str(&json).unwrap();
        
        assert_eq!(option.name, deserialized.name);
        assert_eq!(option.option_type, deserialized.option_type);
    }

    #[test]
    fn test_hm_module_serialization() {
        let module = HMModule {
            module_name: "git".to_string(),
            path: "/path/to/module.nix".to_string(),
            imports: vec!["import1.nix".to_string()],
            arguments: vec!["config".to_string(), "pkgs".to_string()],
            documentation_url: "https://example.com".to_string(),
        };

        let json = serde_json::to_string(&module).unwrap();
        let deserialized: HMModule = serde_json::from_str(&json).unwrap();
        
        assert_eq!(module.module_name, deserialized.module_name);
        assert_eq!(module.imports.len(), deserialized.imports.len());
    }

    #[test]
    fn test_patch_result_serialization() {
        let result = PatchResult {
            success: true,
            diff_applied: Some("--- original\n+++ modified".to_string()),
            backup_created: true,
            error: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: PatchResult = serde_json::from_str(&json).unwrap();
        
        assert!(deserialized.success);
        assert!(deserialized.backup_created);
    }

    #[test]
    fn test_build_result_serialization() {
        let result = BuildResult {
            success: true,
            logs: "Build successful".to_string(),
            errors: vec![],
            warnings: vec!["Deprecated option".to_string()],
            changes_detected: true,
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: BuildResult = serde_json::from_str(&json).unwrap();
        
        assert!(deserialized.success);
        assert_eq!(deserialized.warnings.len(), 1);
    }

    #[test]
    fn test_template_result_serialization() {
        let template = TemplateResult {
            program_name: "git".to_string(),
            snippet: "programs.git.enable = true;".to_string(),
            description: "Git configuration".to_string(),
            required_options: vec!["programs.git.enable".to_string()],
            documentation_url: "https://example.com".to_string(),
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: TemplateResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(template.program_name, deserialized.program_name);
        assert_eq!(template.required_options.len(), deserialized.required_options.len());
    }
}
