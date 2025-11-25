use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 Response structure for MCP protocol
///
/// All responses follow the JSON-RPC 2.0 specification with the `id` field
/// always present (never null) as required by the protocol.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    pub id: Value,  // Required, never null (JSON-RPC 2.0 requirement)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

/// JSON-RPC 2.0 Error Response
///
/// Contains error code, message, and optional data for debugging.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

/// JSON-RPC error codes
pub mod error_codes {
    /// Parse error - Invalid JSON was received
    pub const PARSE_ERROR: i32 = -32700;
    
    /// Invalid Request - The JSON sent is not a valid Request object
    pub const INVALID_REQUEST: i32 = -32600;
    
    /// Method not found - The method does not exist / is not available
    pub const METHOD_NOT_FOUND: i32 = -32601;
    
    /// Invalid params - Invalid method parameter(s)
    pub const INVALID_PARAMS: i32 = -32602;
    
    /// Internal error - Internal JSON-RPC error
    pub const INTERNAL_ERROR: i32 = -32603;
}

impl Response {
    /// Create a success response with the given result
    ///
    /// # Arguments
    /// * `id` - The request ID (must not be null)
    /// * `result` - The result value to return
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    ///
    /// # Arguments
    /// * `id` - The request ID (must not be null)
    /// * `code` - JSON-RPC error code (use constants from `error_codes`)
    /// * `message` - Human-readable error message
    /// * `data` - Optional additional error data
    pub fn error(id: Value, code: i32, message: String, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(ErrorResponse { code, message, data }),
        }
    }

    /// Extract response ID from request, converting null/missing to 0
    ///
    /// JSON-RPC 2.0 requires the `id` field to never be null. This function
    /// ensures compliance by converting null or missing IDs to the number 0.
    ///
    /// # Arguments
    /// * `request` - The JSON-RPC request value
    ///
    /// # Returns
    /// A `Value` that is guaranteed to be a string or number, never null
    pub fn extract_id(request: &Value) -> Value {
        match request.get("id") {
            Some(Value::Null) => Value::Number(serde_json::Number::from(0)),
            Some(v) => v.clone(),
            None => Value::Number(serde_json::Number::from(0)),
        }
    }
}

