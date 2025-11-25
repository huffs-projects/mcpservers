use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldRequest {
    pub scaffold_type: ScaffoldType,
    pub template: TemplateType,
    pub target_path: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub inputs: Option<Vec<InputSpec>>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSpec {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub flake: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ScaffoldType {
    Init,
    Generate,
    AddOutput,
    AddInput,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TemplateType {
    Package,
    DevShell,
    NixOS,
    Multi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldResult {
    pub success: bool,
    pub files_created: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flake_content: Option<String>,
    pub logs: String,
    pub errors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_passed: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaffold_type_serialization() {
        let init = ScaffoldType::Init;
        let json = serde_json::to_string(&init).unwrap();
        assert_eq!(json, "\"init\"");
        
        let add_input = ScaffoldType::AddInput;
        let json = serde_json::to_string(&add_input).unwrap();
        assert_eq!(json, "\"addinput\"");
    }

    #[test]
    fn test_template_type_serialization() {
        let package = TemplateType::Package;
        let json = serde_json::to_string(&package).unwrap();
        assert_eq!(json, "\"package\"");
        
        let multi = TemplateType::Multi;
        let json = serde_json::to_string(&multi).unwrap();
        assert_eq!(json, "\"multi\"");
    }

    #[test]
    fn test_input_spec_serialization() {
        let input = InputSpec {
            name: "test-input".to_string(),
            url: "github:test/repo".to_string(),
            flake: Some(true),
        };
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("test-input"));
        assert!(json.contains("github:test/repo"));
    }

    #[test]
    fn test_scaffold_result_with_validation() {
        let result = ScaffoldResult {
            success: true,
            files_created: vec!["file1".to_string()],
            flake_content: Some("content".to_string()),
            logs: "logs".to_string(),
            errors: vec![],
            validation_passed: Some(true),
        };
        
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("validation_passed"));
        assert!(json.contains("true"));
    }
}
