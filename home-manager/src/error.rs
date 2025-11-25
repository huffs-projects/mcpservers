use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Process error: {0}")]
    ProcessError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Security error: {0}")]
    SecurityError(String),
}

impl ServerError {
    pub fn jsonrpc_code(&self) -> i32 {
        match self {
            ServerError::ParseError(_) => -32700,
            ServerError::InvalidRequest(_) => -32600,
            ServerError::MethodNotFound(_) => -32601,
            ServerError::InvalidParams(_) => -32602,
            ServerError::InternalError(_) => -32603,
            ServerError::ValidationError(_) => -32602,
            ServerError::ProcessError(_) => -32603,
            ServerError::TimeoutError(_) => -32603,
            ServerError::SecurityError(_) => -32603,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            ServerError::ParseError(msg) => format!("Parse error: {}", msg),
            ServerError::InvalidRequest(msg) => format!("Invalid request: {}", msg),
            ServerError::MethodNotFound(msg) => format!("Method not found: {}", msg),
            ServerError::InvalidParams(msg) => format!("Invalid params: {}", msg),
            ServerError::InternalError(err) => format!("Internal error: {}", err),
            ServerError::ValidationError(msg) => format!("Validation error: {}", msg),
            ServerError::ProcessError(msg) => format!("Process error: {}", msg),
            ServerError::TimeoutError(msg) => format!("Timeout error: {}", msg),
            ServerError::SecurityError(msg) => format!("Security error: {}", msg),
        }
    }
}

