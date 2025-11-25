use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarshipOption {
    pub name: String,
    #[serde(rename = "type")]
    pub option_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub category: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    pub documentation_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarshipPreset {
    pub preset_name: String,
    pub snippet: String,
    pub description: String,
    pub documentation_url: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateOutput {
    pub template_name: String,
    pub snippet: String,
    pub description: String,
    pub documentation_url: String,
}

