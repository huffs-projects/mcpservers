use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WofiMode {
    pub name: String,
    #[serde(rename = "type")]
    pub mode_type: String, // "builtin" or "custom"
    pub exec: Option<String>,
    pub stdin_format: Option<String>,
    pub stdout_format: Option<String>,
    pub cloudninja_section: String,
    pub manpage_section: String,
}

