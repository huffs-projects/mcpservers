use anyhow::{Context, Result};
use serde_json::Value;

const MAX_STRING_LENGTH: usize = 10000;
const MAX_PATCH_SIZE: usize = 1_000_000;

pub fn validate_string_param(value: &str, max_length: Option<usize>) -> Result<()> {
    let max = max_length.unwrap_or(MAX_STRING_LENGTH);
    
    if value.len() > max {
        anyhow::bail!("String parameter exceeds maximum length of {} characters", max);
    }

    if value.contains('\0') {
        anyhow::bail!("String parameter contains null byte");
    }

    Ok(())
}

pub fn validate_json_params(params: &Value) -> Result<()> {
    if !params.is_object() {
        anyhow::bail!("Parameters must be a JSON object");
    }

    Ok(())
}

pub fn extract_string_param(params: &Value, key: &str, max_length: Option<usize>) -> Result<Option<String>> {
    if let Some(value) = params.get(key) {
        if let Some(str_value) = value.as_str() {
            validate_string_param(str_value, max_length)?;
            return Ok(Some(str_value.to_string()));
        } else if !value.is_null() {
            anyhow::bail!("Parameter '{}' must be a string", key);
        }
    }
    Ok(None)
}

pub fn extract_required_string_param(params: &Value, key: &str, max_length: Option<usize>) -> Result<String> {
    extract_string_param(params, key, max_length)?
        .context(format!("Required parameter '{}' is missing", key))
}

pub fn extract_bool_param(params: &Value, key: &str, default: bool) -> Result<bool> {
    if let Some(value) = params.get(key) {
        if let Some(bool_value) = value.as_bool() {
            return Ok(bool_value);
        } else if !value.is_null() {
            anyhow::bail!("Parameter '{}' must be a boolean", key);
        }
    }
    Ok(default)
}

pub fn validate_patch_content(patch: &str) -> Result<()> {
    validate_string_param(patch, Some(MAX_PATCH_SIZE))?;
    
    if patch.trim().is_empty() {
        anyhow::bail!("Patch content cannot be empty");
    }

    Ok(())
}

pub fn validate_config_path(path: &str) -> Result<()> {
    validate_string_param(path, Some(4096))?;
    
    if !path.ends_with(".nix") {
        anyhow::bail!("Configuration file must have .nix extension");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_string_param() {
        assert!(validate_string_param("test", None).is_ok());
        assert!(validate_string_param(&"a".repeat(10001), None).is_err());
    }

    #[test]
    fn test_extract_string_param() {
        let params = serde_json::json!({"key": "value"});
        assert_eq!(extract_string_param(&params, "key", None).unwrap(), Some("value".to_string()));
        assert_eq!(extract_string_param(&params, "missing", None).unwrap(), None);
    }

    #[test]
    fn test_extract_required_string_param() {
        let params = serde_json::json!({"key": "value"});
        assert_eq!(extract_required_string_param(&params, "key", None).unwrap(), "value");
        assert!(extract_required_string_param(&params, "missing", None).is_err());
    }

    #[test]
    fn test_extract_bool_param() {
        let params = serde_json::json!({"flag": true});
        assert_eq!(extract_bool_param(&params, "flag", false).unwrap(), true);
        assert_eq!(extract_bool_param(&params, "missing", false).unwrap(), false);
    }

    #[test]
    fn test_validate_patch_content() {
        assert!(validate_patch_content("--- a\n+++ b\n").is_ok());
        assert!(validate_patch_content("").is_err());
    }
}

