use serde_json::Value;
use crate::error::{McpError, McpResult};

/// Extract a required string parameter from JSON arguments
pub fn extract_string_param(args: Option<&Value>, param_name: &str) -> McpResult<String> {
    args.and_then(|a| a.get(param_name))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| McpError::ParameterError {
            message: format!("Missing required parameter: {}", param_name),
            parameter: Some(param_name.to_string()),
        })
}

/// Extract an optional string parameter from JSON arguments
pub fn extract_optional_string_param(args: Option<&Value>, param_name: &str) -> Option<String> {
    args.and_then(|a| a.get(param_name))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract a required number parameter from JSON arguments
pub fn extract_number_param<T>(args: Option<&Value>, param_name: &str) -> McpResult<T>
where
    T: TryFrom<u64>,
    <T as TryFrom<u64>>::Error: std::fmt::Display,
{
    args.and_then(|a| a.get(param_name))
        .and_then(|v| v.as_u64())
        .and_then(|v| T::try_from(v).ok())
        .ok_or_else(|| McpError::ParameterError {
            message: format!("Missing or invalid number parameter: {}", param_name),
            parameter: Some(param_name.to_string()),
        })
}

/// Extract an optional number parameter from JSON arguments
pub fn extract_optional_number_param<T>(args: Option<&Value>, param_name: &str) -> Option<T>
where
    T: TryFrom<u64>,
{
    args.and_then(|a| a.get(param_name))
        .and_then(|v| v.as_u64())
        .and_then(|v| T::try_from(v).ok())
}

/// Extract a required boolean parameter from JSON arguments
pub fn extract_bool_param(args: Option<&Value>, param_name: &str) -> McpResult<bool> {
    args.and_then(|a| a.get(param_name))
        .and_then(|v| v.as_bool())
        .ok_or_else(|| McpError::ParameterError {
            message: format!("Missing or invalid boolean parameter: {}", param_name),
            parameter: Some(param_name.to_string()),
        })
}

/// Extract an optional boolean parameter from JSON arguments
pub fn extract_optional_bool_param(args: Option<&Value>, param_name: &str) -> Option<bool> {
    args.and_then(|a| a.get(param_name))
        .and_then(|v| v.as_bool())
}

/// Validate email address format (basic validation)
pub fn validate_email(email: &str) -> bool {
    // Basic email validation - checks for @ and basic structure
    email.contains('@') && 
    email.split('@').count() == 2 &&
    !email.starts_with('@') &&
    !email.ends_with('@') &&
    email.len() > 3
}

/// Validate hostname format (basic validation)
pub fn validate_hostname(hostname: &str) -> bool {
    // Basic hostname validation
    !hostname.is_empty() &&
    hostname.len() < 256 &&
    !hostname.starts_with('.') &&
    !hostname.ends_with('.') &&
    hostname.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-')
}

/// Validate port number
pub fn validate_port(port: u16) -> bool {
    port > 0 // Port must be > 0 (u16 is always <= 65535 by type definition)
}

/// Sanitize file path to prevent directory traversal
pub fn sanitize_path(path: &str) -> McpResult<String> {
    use std::path::Path;
    let path_buf = Path::new(path);
    
    // Check for directory traversal attempts
    if path_buf.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return Err(McpError::IoError {
            message: "Path contains parent directory references".to_string(),
            path: Some(path.to_string()),
        });
    }
    
    Ok(path.to_string())
}

