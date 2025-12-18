use std::path::PathBuf;
use thiserror::Error;

/// Custom error types for the fastfetch MCP server

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file not found: {path}")]
    NotFound { path: PathBuf },

    #[error("Failed to read config file {path}: {source}")]
    ReadError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to write config file {path}: {source}")]
    WriteError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse JSONC config file {path}: {message}")]
    ParseError {
        path: PathBuf,
        message: String,
    },

    #[error("Failed to serialize config: {source}")]
    SerializeError {
        source: serde_json::Error,
    },

    #[error("Failed to create config directory {path}: {source}")]
    DirectoryCreationError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Config directory not found")]
    ConfigDirNotFound,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Schema validation failed with {count} error(s)")]
    ValidationFailed {
        count: usize,
        errors: Vec<String>,
    },

    #[error("Failed to load schema: {source}")]
    SchemaLoadError {
        source: reqwest::Error,
    },

    #[error("Failed to parse schema JSON: {source}")]
    SchemaParseError {
        source: serde_json::Error,
    },

    #[error("Failed to compile schema: {message}")]
    SchemaCompileError {
        message: String,
    },

    #[error("Schema not available (network error or invalid URL)")]
    SchemaUnavailable,
}

#[derive(Error, Debug)]
pub enum FastfetchError {
    #[error("fastfetch command not found. Is fastfetch installed?")]
    CommandNotFound,

    #[error("fastfetch command failed: {stderr}")]
    CommandFailed { stderr: String },

    #[error("Failed to execute fastfetch: {source}")]
    ExecutionError {
        source: std::io::Error,
    },

    #[error("Failed to parse fastfetch output: {source}")]
    ParseOutputError {
        source: std::string::FromUtf8Error,
    },
}

#[derive(Error, Debug)]
pub enum McpServerError {
    #[error("Config error: {0}")]
    Config(#[from] ConfigError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Fastfetch error: {0}")]
    Fastfetch(#[from] FastfetchError),

    #[error("Unknown tool: {tool_name}")]
    UnknownTool { tool_name: String },

    #[error("Missing required parameter: {param}")]
    MissingParameter { param: String },

    #[error("Invalid parameter type: {param}")]
    InvalidParameterType { param: String },
}

/// Result type alias for MCP server operations
pub type McpResult<T> = std::result::Result<T, McpServerError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_error_display() {
        let error = ConfigError::NotFound {
            path: PathBuf::from("/test/path.jsonc"),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Config file not found"));
        assert!(msg.contains("/test/path.jsonc"));
    }

    #[test]
    fn test_config_error_conversion() {
        let config_error = ConfigError::ConfigDirNotFound;
        let mcp_error: McpServerError = config_error.into();
        
        match mcp_error {
            McpServerError::Config(ConfigError::ConfigDirNotFound) => {}
            _ => panic!("Expected Config error variant"),
        }
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::ValidationFailed {
            count: 2,
            errors: vec!["Error 1".to_string(), "Error 2".to_string()],
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Schema validation failed"));
        assert!(msg.contains("2"));
    }

    #[test]
    fn test_validation_error_conversion() {
        let validation_error = ValidationError::SchemaUnavailable;
        let mcp_error: McpServerError = validation_error.into();
        
        match mcp_error {
            McpServerError::Validation(ValidationError::SchemaUnavailable) => {}
            _ => panic!("Expected Validation error variant"),
        }
    }

    #[test]
    fn test_fastfetch_error_display() {
        let error = FastfetchError::CommandNotFound;
        let msg = format!("{}", error);
        assert!(msg.contains("fastfetch command not found"));
        
        let error2 = FastfetchError::CommandFailed {
            stderr: "test error".to_string(),
        };
        let msg2 = format!("{}", error2);
        assert!(msg2.contains("fastfetch command failed"));
        assert!(msg2.contains("test error"));
    }

    #[test]
    fn test_fastfetch_error_conversion() {
        let fastfetch_error = FastfetchError::CommandNotFound;
        let mcp_error: McpServerError = fastfetch_error.into();
        
        match mcp_error {
            McpServerError::Fastfetch(FastfetchError::CommandNotFound) => {}
            _ => panic!("Expected Fastfetch error variant"),
        }
    }

    #[test]
    fn test_mcp_server_error_unknown_tool() {
        let error = McpServerError::UnknownTool {
            tool_name: "test_tool".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Unknown tool"));
        assert!(msg.contains("test_tool"));
    }

    #[test]
    fn test_mcp_server_error_missing_parameter() {
        let error = McpServerError::MissingParameter {
            param: "config".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Missing required parameter"));
        assert!(msg.contains("config"));
    }

    #[test]
    fn test_mcp_server_error_invalid_parameter_type() {
        let error = McpServerError::InvalidParameterType {
            param: "path".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Invalid parameter type"));
        assert!(msg.contains("path"));
    }
}
