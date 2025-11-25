use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MCPError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Invalid params: {0}")]
    InvalidParams(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("Tool error: {0}")]
    ToolError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl MCPError {
    pub fn to_jsonrpc_code(&self) -> i32 {
        match self {
            MCPError::ParseError(_) => -32700,
            MCPError::InvalidRequest(_) => -32600,
            MCPError::MethodNotFound(_) => -32601,
            MCPError::InvalidParams(_) => -32602,
            MCPError::ValidationError(_) | MCPError::ToolError(_) => -32602,
            MCPError::FileError(_) | MCPError::InternalError(_) => -32603,
            MCPError::JsonError(_) => -32700,
        }
    }

    pub fn to_jsonrpc_error(&self) -> JSONRPCError {
        JSONRPCError {
            code: self.to_jsonrpc_code(),
            message: self.to_string(),
            data: Some(Value::String(format!("{:?}", self))),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct JSONRPCError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

pub type Result<T> = std::result::Result<T, MCPError>;

