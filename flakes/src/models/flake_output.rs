use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakeOutput {
    pub attribute: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drv_path: Option<String>,
    pub r#type: OutputType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Package,
    App,
    Module,
}

