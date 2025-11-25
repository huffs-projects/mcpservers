use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WofiTemplate {
    pub name: String,
    pub description: String,
    pub config_snippet: String,
    pub css_snippet: Option<String>,
    pub modes_used: Vec<String>,
    pub source_documents: Vec<String>,
}

