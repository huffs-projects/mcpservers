use thiserror::Error;

/// Custom error types for the Starship MCP Server
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Path validation failed: {0}")]
    PathValidation(String),

    #[error("Input validation failed: {0}")]
    InputValidation(String),

    #[error("File operation failed: {0}")]
    FileOperation(String),

    #[error("TOML parsing failed: {0}")]
    TomlParse(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

/// Result type alias using our custom Error
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TomlParse(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::TomlParse(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::FileOperation(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::InvalidRequest(err.to_string())
    }
}

