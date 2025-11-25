use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaybarScript {
    pub name: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    pub documentation_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl WaybarScript {
    pub fn new(
        name: String,
        command: String,
        documentation_url: String,
    ) -> Self {
        Self {
            name,
            command,
            interval: None,
            output_format: None,
            documentation_url,
            notes: None,
        }
    }

    pub fn with_interval(mut self, interval: u64) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn with_output_format(mut self, format: String) -> Self {
        self.output_format = Some(format);
        self
    }

    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
}

