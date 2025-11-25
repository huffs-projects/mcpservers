use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaybarTemplate {
    pub name: String,
    pub json_snippet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_snippet: Option<String>,
    pub description: String,
    pub modules_used: Vec<String>,
    pub scripts_used: Vec<String>,
    pub style_selectors_used: Vec<String>,
}

impl WaybarTemplate {
    pub fn new(
        name: String,
        json_snippet: String,
        description: String,
    ) -> Self {
        Self {
            name,
            json_snippet,
            css_snippet: None,
            description,
            modules_used: Vec::new(),
            scripts_used: Vec::new(),
            style_selectors_used: Vec::new(),
        }
    }

    pub fn with_css(mut self, css: String) -> Self {
        self.css_snippet = Some(css);
        self
    }

    pub fn with_modules(mut self, modules: Vec<String>) -> Self {
        self.modules_used = modules;
        self
    }

    pub fn with_scripts(mut self, scripts: Vec<String>) -> Self {
        self.scripts_used = scripts;
        self
    }

    pub fn with_style_selectors(mut self, selectors: Vec<String>) -> Self {
        self.style_selectors_used = selectors;
        self
    }
}

