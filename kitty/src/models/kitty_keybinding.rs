use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KittyKeybinding {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    pub modifiers: Vec<String>,
    pub keys: String,
    pub documentation_url: String,
}

