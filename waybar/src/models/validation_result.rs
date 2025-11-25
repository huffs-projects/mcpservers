use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub logs: String,
    pub missing_required_keys: Vec<String>,
    pub invalid_css_properties: Vec<String>,
    pub invalid_script_commands: Vec<String>,
}

impl ValidationResult {
    pub fn success() -> Self {
        Self {
            success: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            logs: String::new(),
            missing_required_keys: Vec::new(),
            invalid_css_properties: Vec::new(),
            invalid_script_commands: Vec::new(),
        }
    }

    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            success: false,
            errors,
            warnings: Vec::new(),
            logs: String::new(),
            missing_required_keys: Vec::new(),
            invalid_css_properties: Vec::new(),
            invalid_script_commands: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.success = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn add_log(&mut self, log: String) {
        if !self.logs.is_empty() {
            self.logs.push('\n');
        }
        self.logs.push_str(&log);
    }
}

