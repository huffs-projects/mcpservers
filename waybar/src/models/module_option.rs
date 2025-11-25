use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaybarModuleOption {
    pub module_name: String,
    pub option_name: String,
    #[serde(rename = "type")]
    pub option_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub required: bool,
    pub description: String,
    pub documentation_url: String,
}

impl WaybarModuleOption {
    pub fn new(
        module_name: String,
        option_name: String,
        option_type: String,
        required: bool,
        description: String,
        documentation_url: String,
    ) -> Self {
        Self {
            module_name,
            option_name,
            option_type,
            default: None,
            required,
            description,
            documentation_url,
        }
    }

    pub fn with_default(mut self, default: String) -> Self {
        self.default = Some(default);
        self
    }
}

