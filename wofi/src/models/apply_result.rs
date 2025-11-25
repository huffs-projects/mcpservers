use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResult {
    pub success: bool,
    pub diff_config: String,
    pub diff_css: Option<String>,
    pub backup_path: String,
}

