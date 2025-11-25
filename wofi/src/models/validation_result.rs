use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub invalid_css: Vec<String>,
    pub invalid_options: Vec<String>,
    pub invalid_modes: Vec<String>,
}

