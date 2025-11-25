use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KittyTheme {
    pub theme_name: String,
    pub snippet: String,
    pub description: String,
    pub palette: HashMap<String, String>,
    pub documentation_url: String,
}

