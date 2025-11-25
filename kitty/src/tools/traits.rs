use async_trait::async_trait;
use serde_json::Value;

/// Trait that all MCP tools must implement
#[async_trait]
pub trait Tool: Send + Sync {
    /// Returns the tool name (used in MCP tool calls)
    fn name(&self) -> &str;
    
    /// Returns a human-readable description of what the tool does
    fn description(&self) -> &str;
    
    /// Returns the JSON schema for the tool's input parameters
    fn input_schema(&self) -> Value;
    
    /// Executes the tool with the given arguments
    /// Returns the result as a JSON Value, or an error message
    async fn execute(&self, arguments: Value) -> Result<Value, String>;
}

