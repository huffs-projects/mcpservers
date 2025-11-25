use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZshOption {
    pub name: String,
    pub scope: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub description: String,
    pub documentation_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZshTemplate {
    pub template_name: String,
    pub snippet: String,
    pub description: String,
    pub uses_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub logs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResult {
    pub success: bool,
    pub diff_applied: String,
    pub backup_created: bool,
}

