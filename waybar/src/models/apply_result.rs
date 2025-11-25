use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResult {
    pub success: bool,
    pub diff_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_css: Option<String>,
    pub backup_created: bool,
    pub applied_modules: Vec<String>,
    pub applied_scripts: Vec<String>,
    pub applied_styles: Vec<String>,
}

impl ApplyResult {
    pub fn new() -> Self {
        Self {
            success: false,
            diff_json: String::new(),
            diff_css: None,
            backup_created: false,
            applied_modules: Vec::new(),
            applied_scripts: Vec::new(),
            applied_styles: Vec::new(),
        }
    }

    pub fn success() -> Self {
        Self {
            success: true,
            diff_json: String::new(),
            diff_css: None,
            backup_created: false,
            applied_modules: Vec::new(),
            applied_scripts: Vec::new(),
            applied_styles: Vec::new(),
        }
    }
}

impl Default for ApplyResult {
    fn default() -> Self {
        Self::new()
    }
}

