use thiserror::Error;
use serde_json::Value;

/// Custom error types for the MCP server
/// 
/// These error types provide structured error handling throughout the server.
/// Each error can be converted to an MCP error response with appropriate
/// error codes and messages.
/// 
/// # Example
/// ```
/// use kitty_mcp_server::error::MCPError;
/// 
/// let error = MCPError::ToolExecution("Something went wrong".to_string());
/// let (code, message, data) = error.to_mcp_error();
/// ```
#[derive(Error, Debug)]
pub enum MCPError {
    #[error("Tool execution failed: {0}")]
    ToolExecution(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("File operation failed: {0}")]
    FileOperation(#[from] std::io::Error),
    
    #[error("Config validation failed: {0}")]
    Validation(String),
    
    #[error("Unknown tool: {0}")]
    UnknownTool(String),
}

/// Convert MCPError to MCP error response
impl MCPError {
    /// Convert this error to MCP error response format
    /// 
    /// # Returns
    /// A tuple containing:
    /// - Error code (i32): JSON-RPC error code
    /// - Error message (String): Human-readable error message
    /// - Error data (Option<Value>): Optional additional error data
    pub fn to_mcp_error(&self) -> (i32, String, Option<Value>) {
        match self {
            MCPError::ToolExecution(msg) => (-32603, format!("Tool execution failed: {}", msg), None),
            MCPError::Serialization(msg) => (-32603, format!("Serialization error: {}", msg), None),
            MCPError::InvalidArgument(msg) => (-32602, format!("Invalid argument: {}", msg), None),
            MCPError::FileOperation(e) => (-32000, format!("File operation failed: {}", e), None),
            MCPError::Validation(msg) => (-32000, format!("Validation failed: {}", msg), None),
            MCPError::UnknownTool(name) => (-32601, format!("Unknown tool: {}", name), None),
        }
    }
}

