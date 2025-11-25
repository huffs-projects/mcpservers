use crate::tools::traits::Tool;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry that holds all available MCP tools
/// 
/// The tool registry maintains a collection of all available tools and provides
/// methods to list them and execute them. Tools are registered at startup and
/// can be accessed by name.
/// 
/// # Example
/// ```
/// use kitty_mcp_server::tools::ToolRegistry;
/// 
/// let registry = ToolRegistry::new();
/// let tools = registry.list_tools();
/// println!("Available tools: {}", tools.len());
/// ```
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    /// Create a new tool registry with all tools registered
    /// 
    /// Initializes the registry and registers all available tools.
    /// This is typically called once at startup.
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        registry.register_all_tools();
        registry
    }
    
    /// Register a tool in the registry
    fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }
    
    /// Get a tool by name
    /// 
    /// # Arguments
    /// * `name` - The name of the tool to retrieve
    /// 
    /// # Returns
    /// * `Some(&Arc<dyn Tool>)` - The tool if found
    /// * `None` - If no tool with that name exists
    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }
    
    /// Get all tools as MCP Tool format
    /// 
    /// Returns a vector of JSON values representing all registered tools
    /// in the format expected by the MCP protocol.
    /// 
    /// # Returns
    /// A vector of JSON objects, each containing:
    /// - `name`: Tool identifier
    /// - `description`: Human-readable description
    /// - `inputSchema`: JSON schema for tool parameters
    pub fn list_tools(&self) -> Vec<Value> {
        use serde_json::json;
        self.tools
            .values()
            .map(|tool| {
                json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "inputSchema": tool.input_schema()
                })
            })
            .collect()
    }
    
    /// Register all available tools
    fn register_all_tools(&mut self) {
        use crate::tools::implementations::*;
        
        self.register(Arc::new(KittyOptionsTool));
        self.register(Arc::new(KittyThemingTool));
        self.register(Arc::new(KittyKeybindingsTool));
        self.register(Arc::new(KittyTemplatesTool));
        self.register(Arc::new(KittyValidateTool));
        self.register(Arc::new(KittyApplyTool));
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

