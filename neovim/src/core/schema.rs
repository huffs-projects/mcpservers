use crate::core::model::NvimOption;
use serde_json::Value;
use std::collections::HashMap;

/// Typed schema for Neovim options derived from documentation
pub struct OptionSchema {
    schemas: HashMap<String, Value>,
}

impl OptionSchema {
    pub fn new() -> Self {
        let mut schema = Self {
            schemas: HashMap::new(),
        };
        schema.initialize_schemas();
        schema
    }

    fn initialize_schemas(&mut self) {
        // Define schemas for common option types
        let number_schema = serde_json::json!({
            "type": "number",
            "minimum": 0
        });

        let boolean_schema = serde_json::json!({
            "type": "boolean"
        });

        let string_schema = serde_json::json!({
            "type": "string"
        });

        // Apply schemas to option types
        self.schemas.insert("number".to_string(), number_schema);
        self.schemas.insert("boolean".to_string(), boolean_schema);
        self.schemas.insert("string".to_string(), string_schema);
    }

    /// Validate an option value against its schema
    pub fn validate_option_value(&self, option: &NvimOption, value: &str) -> Result<(), String> {
        match option.option_type.as_str() {
            "number" => {
                value.parse::<f64>()
                    .map_err(|_| format!("Expected number, got: {}", value))?;
            }
            "boolean" => {
                if value != "true" && value != "false" {
                    return Err(format!("Expected boolean (true/false), got: {}", value));
                }
            }
            "string" => {
                // String is always valid
            }
            _ => {
                // Unknown type, skip validation
            }
        }

        // Check valid_values if specified
        if let Some(ref valid_values) = option.valid_values {
            if !valid_values.contains(&value.to_string()) {
                return Err(format!(
                    "Value '{}' not in valid values: {:?}",
                    value, valid_values
                ));
            }
        }

        Ok(())
    }

    /// Get schema for an option type
    pub fn get_schema(&self, option_type: &str) -> Option<&Value> {
        self.schemas.get(option_type)
    }
}

impl Default for OptionSchema {
    fn default() -> Self {
        Self::new()
    }
}

