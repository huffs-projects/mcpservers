use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaybarStyleSnippet {
    pub selector: String,
    pub properties: HashMap<String, String>,
    pub documentation_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl WaybarStyleSnippet {
    pub fn new(
        selector: String,
        properties: HashMap<String, String>,
        documentation_url: String,
    ) -> Self {
        Self {
            selector,
            properties,
            documentation_url,
            notes: None,
        }
    }

    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
}

