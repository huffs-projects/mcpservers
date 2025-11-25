use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WofiStyleRule {
    pub selector: String,
    pub properties: HashMap<String, String>,
    pub description: String,
    pub source: String,
}

