use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResult {
    pub result: String,
    pub success: bool,
    pub logs: String,
}

