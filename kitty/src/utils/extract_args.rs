use serde_json::Value;

/// Extract a string argument from JSON arguments object
/// 
/// # Arguments
/// * `args` - The JSON object containing arguments
/// * `key` - The key to extract
/// 
/// # Returns
/// * `Some(String)` - The string value if found and valid
/// * `None` - If the key doesn't exist or value is not a string
/// 
/// # Example
/// ```
/// use kitty_mcp_server::utils::extract_args_mod as extract_args;
/// use serde_json::json;
/// 
/// let args = json!({"name": "test"});
/// assert_eq!(extract_args::extract_string(&args, "name"), Some("test".to_string()));
/// ```
pub fn extract_string(args: &Value, key: &str) -> Option<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract a boolean argument from JSON arguments object
/// 
/// # Arguments
/// * `args` - The JSON object containing arguments
/// * `key` - The key to extract
/// 
/// # Returns
/// * `Some(bool)` - The boolean value if found and valid
/// * `None` - If the key doesn't exist or value is not a boolean
pub fn extract_bool(args: &Value, key: &str) -> Option<bool> {
    args.get(key)
        .and_then(|v| v.as_bool())
}

/// Extract an integer argument from JSON arguments object
/// 
/// # Arguments
/// * `args` - The JSON object containing arguments
/// * `key` - The key to extract
/// 
/// # Returns
/// * `Some(i64)` - The integer value if found and valid
/// * `None` - If the key doesn't exist or value is not an integer
pub fn extract_int(args: &Value, key: &str) -> Option<i64> {
    args.get(key)
        .and_then(|v| v.as_i64())
}

/// Extract a float argument from JSON arguments object
/// 
/// # Arguments
/// * `args` - The JSON object containing arguments
/// * `key` - The key to extract
/// 
/// # Returns
/// * `Some(f64)` - The float value if found and valid
/// * `None` - If the key doesn't exist or value is not a number
pub fn extract_float(args: &Value, key: &str) -> Option<f64> {
    args.get(key)
        .and_then(|v| v.as_f64())
}

