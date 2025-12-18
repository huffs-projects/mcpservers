use std::fmt;

/// Custom error types for the NeoMutt MCP server
#[derive(Debug)]
pub enum McpError {
    /// Configuration parsing error
    ParseError {
        line: usize,
        message: String,
        context: Option<String>,
    },
    /// Configuration validation error
    ValidationError {
        message: String,
        field: Option<String>,
    },
    /// Network/HTTP error
    NetworkError {
        message: String,
        url: Option<String>,
    },
    /// File I/O error
    IoError {
        message: String,
        path: Option<String>,
    },
    /// Invalid parameter error
    ParameterError {
        message: String,
        parameter: Option<String>,
    },
    /// Unknown tool or method error
    UnknownMethod {
        method: String,
    },
    /// Internal error
    InternalError {
        message: String,
    },
}

impl fmt::Display for McpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            McpError::ParseError { line, message, context } => {
                write!(f, "Parse error at line {}: {}", line, message)?;
                if let Some(ctx) = context {
                    write!(f, " (context: {})", ctx)?;
                }
                Ok(())
            }
            McpError::ValidationError { message, field } => {
                write!(f, "Validation error: {}", message)?;
                if let Some(field_name) = field {
                    write!(f, " (field: {})", field_name)?;
                }
                Ok(())
            }
            McpError::NetworkError { message, url } => {
                write!(f, "Network error: {}", message)?;
                if let Some(u) = url {
                    write!(f, " (URL: {})", u)?;
                }
                Ok(())
            }
            McpError::IoError { message, path } => {
                write!(f, "I/O error: {}", message)?;
                if let Some(p) = path {
                    write!(f, " (path: {})", p)?;
                }
                Ok(())
            }
            McpError::ParameterError { message, parameter } => {
                write!(f, "Parameter error: {}", message)?;
                if let Some(param_name) = parameter {
                    write!(f, " (parameter: {})", param_name)?;
                }
                Ok(())
            }
            McpError::UnknownMethod { method } => {
                write!(f, "Unknown method: {}", method)
            }
            McpError::InternalError { message } => {
                write!(f, "Internal error: {}", message)
            }
        }
    }
}

impl std::error::Error for McpError {}

impl From<std::io::Error> for McpError {
    fn from(err: std::io::Error) -> Self {
        McpError::IoError {
            message: err.to_string(),
            path: None,
        }
    }
}

impl From<reqwest::Error> for McpError {
    fn from(err: reqwest::Error) -> Self {
        McpError::NetworkError {
            message: err.to_string(),
            url: err.url().map(|u| u.to_string()),
        }
    }
}

impl From<serde_json::Error> for McpError {
    fn from(err: serde_json::Error) -> Self {
        McpError::ParseError {
            line: err.line(),
            message: err.to_string(),
            context: None,
        }
    }
}

impl From<anyhow::Error> for McpError {
    fn from(err: anyhow::Error) -> Self {
        McpError::InternalError {
            message: err.to_string(),
        }
    }
}

/// Result type alias for convenience
pub type McpResult<T> = Result<T, McpError>;

