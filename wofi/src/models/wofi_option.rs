use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WofiOption {
    pub name: String,
    #[serde(rename = "type")]
    pub option_type: String,
    pub default: Option<String>,
    pub description: String,
    pub source: String, // "sr.ht" | "man" | "cloudninja" | "combined"
    pub manpage_section: String,
    pub srht_anchor: String,
    pub cloudninja_topic: String,
}

