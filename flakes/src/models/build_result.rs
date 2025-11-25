use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub success: bool,
    pub logs: String,
    pub errors: Vec<String>,
    pub built_paths: Vec<String>,
}

