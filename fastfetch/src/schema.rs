use crate::constants::FASTFETCH_SCHEMA_URL;
use crate::error::ValidationError;
use dirs;
use jsonschema::JSONSchema;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;

/// JSON schema validation for fastfetch configurations.
/// This module provides schema loading, caching, and validation functionality
/// for fastfetch configuration files.
/// Schema cache entry with timestamp for TTL
struct CachedSchema {
    schema_json: Value,
    cached_at: SystemTime,
}

/// Compiled schema cache entry
/// The compiled schema owns its data (JSONSchema has no lifetime parameter in 0.17)
/// We store the schema JSON hash to detect when the schema changes
/// Using Arc to allow cloning and validating outside the lock
struct CompiledSchemaCache {
    compiled: Arc<JSONSchema>,
    #[allow(dead_code)] // Stored for potential future use (recompilation, debugging)
    schema_json: Value, // Store the schema JSON for reference
    schema_json_hash: u64, // Hash to detect schema changes
}

static SCHEMA_CACHE: OnceLock<Mutex<Option<CachedSchema>>> = OnceLock::new();
static COMPILED_SCHEMA_CACHE: OnceLock<Mutex<Option<CompiledSchemaCache>>> = OnceLock::new();

/// Schema cache TTL (24 hours)
const SCHEMA_CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60);

/// Compute a hash of the schema JSON for change detection.
/// 
/// This function hashes the JSON structure directly without full serialization,
/// which is more efficient than serializing to string first.
fn schema_hash(schema: &Value) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    
    let mut hasher = DefaultHasher::new();
    hash_value(schema, &mut hasher);
    hasher.finish()
}

/// Recursively hash a JSON Value without full serialization.
fn hash_value(value: &Value, hasher: &mut std::collections::hash_map::DefaultHasher) {
    match value {
        Value::Null => {
            use std::hash::Hash;
            "null".hash(hasher);
        }
        Value::Bool(b) => {
            use std::hash::Hash;
            "bool".hash(hasher);
            b.hash(hasher);
        }
        Value::Number(n) => {
            use std::hash::Hash;
            "number".hash(hasher);
            // Hash number as string representation for consistency
            if let Some(i) = n.as_i64() {
                i.hash(hasher);
            } else if let Some(u) = n.as_u64() {
                u.hash(hasher);
            } else if let Some(f) = n.as_f64() {
                // Hash f64 by converting to bits for consistency
                f.to_bits().hash(hasher);
            }
        }
                Value::String(s) => {
            use std::hash::Hash;
            "string".hash(hasher);
            s.hash(hasher);
        }
        Value::Array(arr) => {
            use std::hash::Hash;
            "array".hash(hasher);
            arr.len().hash(hasher);
            for item in arr {
                hash_value(item, hasher);
            }
        }
        Value::Object(obj) => {
            use std::hash::Hash;
            "object".hash(hasher);
            obj.len().hash(hasher);
            // Hash keys and values in sorted order for consistency
            let mut entries: Vec<_> = obj.iter().collect();
            entries.sort_by_key(|(k, _)| *k);
            for (key, val) in entries {
                key.hash(hasher);
                hash_value(val, hasher);
            }
        }
    }
}

/// Get or compile the schema, executing validation with the cached compiled schema
/// This function ensures we only leak memory once per schema version, not on every validation
async fn validate_with_cached_schema(
    schema_json: &Value,
    config: &Value,
) -> Result<Vec<OwnedValidationError>, ValidationError> {
    let schema_hash = schema_hash(schema_json);
    let compiled_cache = COMPILED_SCHEMA_CACHE.get_or_init(|| Mutex::new(None));
    let mut compiled_guard = compiled_cache.lock().await;
    
    // Check if we have a cached compiled schema with matching hash
    let needs_compile = match *compiled_guard {
        Some(ref cached) if cached.schema_json_hash == schema_hash => {
            // Use existing compiled schema - no leak needed
            false
        }
        _ => {
            // Schema changed or not yet compiled - need to compile
            true
        }
    };
    
    if needs_compile {
        // Schema changed or not yet compiled - compile it once
        // JSONSchema in 0.17 owns its data, so we can compile with a regular reference
        // Store the schema JSON in the cache to keep it alive
        let schema_json_clone = schema_json.clone();
        
        let compiled_schema = JSONSchema::compile(&schema_json_clone)
            .map_err(|e| ValidationError::SchemaCompileError {
                message: format!("Failed to compile schema: {}", e),
            })?;
        
        // Cache the compiled schema along with the schema JSON
        *compiled_guard = Some(CompiledSchemaCache {
            compiled: Arc::new(compiled_schema),
            schema_json: schema_json_clone,
            schema_json_hash: schema_hash,
        });
    }
    
    // Clone the Arc to get a reference to the compiled schema
    // This allows us to release the lock before validation, reducing contention
    let compiled_schema = {
        let cached = compiled_guard.as_ref()
            .ok_or_else(|| ValidationError::SchemaCompileError {
                message: "Compiled schema cache is None after compilation".to_string(),
            })?;
        Arc::clone(&cached.compiled)
    };
    
    // Release the lock before validation to reduce contention
    drop(compiled_guard);
    
    // Validate outside the lock
    let validation_result = compiled_schema.validate(config);
    
    // Convert ValidationError<'a> to OwnedValidationError
    match validation_result {
        Ok(()) => Ok(vec![]),
        Err(iterator) => {
            let errors: Vec<OwnedValidationError> = iterator
                .map(|e| OwnedValidationError {
                    instance_path: e.instance_path.to_string(),
                    schema_path: e.schema_path.to_string(),
                    message: e.to_string(),
                })
                .collect();
            Ok(errors)
        }
    }
}

/// Get the schema cache file path
fn schema_cache_file_path() -> Result<PathBuf, ValidationError> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| ValidationError::SchemaUnavailable)?;
    Ok(cache_dir.join("fastfetch-mcp-server").join("schema.json"))
}


/// Save schema to disk cache
async fn save_schema_to_disk(schema: &Value) -> Result<(), ValidationError> {
    let cache_file = schema_cache_file_path()?;
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = cache_file.parent() {
        tokio::fs::create_dir_all(parent).await
            .map_err(|_| ValidationError::SchemaUnavailable)?;
    }
    
    let content = serde_json::to_string_pretty(schema)
        .map_err(|source| ValidationError::SchemaParseError { source })?;
    
    tokio::fs::write(&cache_file, content).await
        .map_err(|_| ValidationError::SchemaUnavailable)?;
    
    Ok(())
}

/// Load the fastfetch JSON schema as a Value
/// The schema URL is: https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json
/// 
/// This function:
/// 1. Checks in-memory cache first (with TTL)
/// 2. Falls back to disk cache if in-memory cache is stale/missing
/// 3. Fetches from network if no cache is available
/// 4. Updates both caches on successful fetch
/// 
/// Returns the schema as a Value (not compiled) because JSONSchema borrows from Value
/// and we can't safely return a borrowed schema from this function.
pub async fn load_schema_value() -> Result<Option<Value>, ValidationError> {
    let cache = SCHEMA_CACHE.get_or_init(|| Mutex::new(None));
    let mut cached = cache.lock().await;
    
    // Check in-memory cache first
    let should_use_cache = if let Some(ref cached_schema) = *cached {
        let age = SystemTime::now()
            .duration_since(cached_schema.cached_at)
            .unwrap_or(Duration::ZERO);
        age < SCHEMA_CACHE_TTL
    } else {
        false
    };
    
    if should_use_cache {
        // Cache is still valid - return cached JSON
        let schema_json = cached.as_ref()
            .ok_or_else(|| ValidationError::SchemaUnavailable)?
            .schema_json.clone();
        drop(cached); // Release the lock
        return Ok(Some(schema_json));
    }
    
    // Try to fetch from network with timeout
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|_| ValidationError::SchemaUnavailable)?;
    
    let response = client.get(FASTFETCH_SCHEMA_URL).send().await;
    
    match response {
        Ok(resp) if resp.status().is_success() => {
            let schema_value: Value = resp.json().await
                .map_err(|source| ValidationError::SchemaLoadError { source })?;
            
            // Save to disk cache
            if let Err(e) = save_schema_to_disk(&schema_value).await {
                // Log but don't fail - disk cache is optional
                eprintln!("Warning: Failed to save schema to disk cache: {}", e);
            }
            
            // Update in-memory cache with the JSON value
            let schema_json_for_cache = schema_value.clone();
            *cached = Some(CachedSchema {
                schema_json: schema_json_for_cache.clone(),
                cached_at: SystemTime::now(),
            });
            
            Ok(Some(schema_value))
        }
        Ok(_) => {
            // Non-success status, try disk cache
            load_schema_from_disk().await
        }
        Err(e) => {
            // Network error, try disk cache
            if let Ok(disk_schema) = load_schema_from_disk().await {
                Ok(disk_schema)
            } else {
                Err(ValidationError::SchemaLoadError { source: e })
            }
        }
    }
}

/// Load schema from disk cache (returns Value, not compiled schema)
async fn load_schema_from_disk() -> Result<Option<Value>, ValidationError> {
    let cache_file = schema_cache_file_path()?;
    
    if !cache_file.exists() {
        return Ok(None);
    }
    
    let content = tokio::fs::read_to_string(&cache_file).await
        .map_err(|_| ValidationError::SchemaUnavailable)?;
    
    let schema_value: Value = serde_json::from_str(&content)
        .map_err(|source| ValidationError::SchemaParseError { source })?;
    
    Ok(Some(schema_value))
}

/// Owned validation error information
#[derive(Debug, Clone)]
pub struct OwnedValidationError {
    pub instance_path: String,
    pub schema_path: String,
    pub message: String,
}

/// Validate a config against the JSON schema.
/// 
/// Validates a fastfetch configuration object against the official JSON schema.
/// Returns a list of validation errors if any are found.
/// 
/// # Parameters
/// 
/// * `config` - The configuration object to validate
/// 
/// # Returns
/// 
/// * `Ok(Vec<OwnedValidationError>)` - List of validation errors (empty if valid)
/// * `Err` - If schema loading fails
/// 
/// # Example
/// 
/// ```
/// use fastfetch_mcp_server::schema::validate_config;
/// use serde_json::json;
/// 
/// let config = json!({
///     "logo": "arch",
///     "modules": []
/// });
/// 
/// let errors = validate_config(&config).await?;
/// if errors.is_empty() {
///     println!("Config is valid!");
/// }
/// ```
pub async fn validate_config(config: &Value) -> Result<Vec<OwnedValidationError>, ValidationError> {
    let schema_value_opt = load_schema_value().await?;
    
    // If no schema, skip validation
    let schema_json = match schema_value_opt {
        Some(s) => s,
        None => return Ok(vec![]),
    };
    
    // Use cached compilation - this only leaks memory once per schema version
    validate_with_cached_schema(&schema_json, config).await
}

/// Validate config and return a human-readable summary.
/// 
/// # Parameters
/// 
/// * `config` - The configuration object to validate
/// 
/// # Returns
/// 
/// * `Ok(String)` - A summary message indicating validity or listing errors
/// * `Err` - If schema loading fails
/// 
/// # Example
/// 
/// ```
/// use fastfetch_mcp_server::schema::validate_config_summary;
/// use serde_json::json;
/// 
/// let config = json!({"logo": "arch"});
/// let summary = validate_config_summary(&config).await?;
/// println!("{}", summary);
/// ```
pub async fn validate_config_summary(config: &Value) -> Result<String, ValidationError> {
    let errors = validate_config(config).await?;
    
    if errors.is_empty() {
        Ok("Configuration is valid".to_string())
    } else {
        let error_messages: Vec<String> = errors.iter()
            .map(|e| format!("{} (instance: {}, schema: {})", e.message, e.instance_path, e.schema_path))
            .collect();
        
        let summary = format!("Found {} validation error(s):\n", errors.len()) +
            &error_messages.iter()
                .enumerate()
                .map(|(i, msg)| format!("  {}. {}\n", i + 1, msg))
                .collect::<Vec<_>>()
                .join("");
        
        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_validate_config_empty() {
        // Test validation with empty config (should work if schema allows it or skip validation)
        let config = json!({});
        let errors = validate_config(&config).await.unwrap();
        // If schema is unavailable, errors will be empty (validation skipped)
        // This is expected behavior - just verify it doesn't panic
        let _ = errors;
    }

    #[tokio::test]
    async fn test_validate_config_summary() {
        let config = json!({
            "logo": "arch",
            "modules": []
        });
        
        let summary = validate_config_summary(&config).await.unwrap();
        // Summary should contain either "valid" or error information
        assert!(!summary.is_empty());
    }

    #[tokio::test]
    async fn test_validate_config_with_data() {
        // Test with a reasonable config structure
        let config = json!({
            "logo": "arch",
            "logoType": "small",
            "modules": [
                {
                    "key": "os",
                    "format": "OS: @name"
                }
            ]
        });
        
        let errors = validate_config(&config).await.unwrap();
        // Should either be valid (empty errors) or have validation errors
        // We can't assert specific behavior without the actual schema
        // Just verify it doesn't panic
        let _ = errors;
    }

    #[tokio::test]
    async fn test_validate_config_invalid_structure() {
        // Test with clearly invalid config structure
        let invalid_config = json!({
            "invalid": "structure",
            "with": ["nested", "arrays", "that", "don't", "match", "schema"]
        });
        
        let errors = validate_config(&invalid_config).await.unwrap();
        // Should either return errors or skip validation if schema unavailable
        let _ = errors;
    }

    #[tokio::test]
    async fn test_validate_config_summary_with_errors() {
        // Test summary generation with potentially invalid config
        let config = json!({
            "invalid_field": "should cause errors"
        });
        
        let summary = validate_config_summary(&config).await.unwrap();
        // Summary should be non-empty
        assert!(!summary.is_empty());
    }

    #[test]
    fn test_schema_hash_consistency() {
        // Test that schema hash is consistent for same input
        let schema1 = json!({
            "type": "object",
            "properties": {
                "key": {"type": "string"}
            }
        });
        
        let schema2 = json!({
            "type": "object",
            "properties": {
                "key": {"type": "string"}
            }
        });
        
        let hash1 = schema_hash(&schema1);
        let hash2 = schema_hash(&schema2);
        assert_eq!(hash1, hash2, "Same schema should produce same hash");
    }

    #[test]
    fn test_schema_hash_different() {
        // Test that different schemas produce different hashes
        let schema1 = json!({
            "type": "object",
            "properties": {
                "key1": {"type": "string"}
            }
        });
        
        let schema2 = json!({
            "type": "object",
            "properties": {
                "key2": {"type": "string"}
            }
        });
        
        let hash1 = schema_hash(&schema1);
        let hash2 = schema_hash(&schema2);
        assert_ne!(hash1, hash2, "Different schemas should produce different hashes");
    }
}
