use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Neovim option definition with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvimOption {
    pub name: String,
    pub scope: String, // "global" | "window" | "buffer"
    #[serde(rename = "type")]
    pub option_type: String, // "string" | "number" | "boolean" | "list" | "map"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    pub documentation: String,
    pub help_tag: String,
    pub documentation_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_api: Option<u32>,
    #[serde(default)]
    pub deprecated: bool,
}

/// Template for generating Neovim config snippets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvimTemplate {
    pub template_name: String,
    #[serde(default = "default_language")]
    pub language: String,
    pub snippet: String,
    pub description: String,
    pub tags: Vec<String>,
    pub related_options: Vec<String>,
}

fn default_language() -> String {
    "lua".to_string()
}

/// Result of configuration validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub success: bool,
    pub syntax_errors: Vec<String>,
    pub semantic_errors: Vec<String>,
    pub warnings: Vec<String>,
    pub unresolved_plugins: Vec<String>,
    pub missing_runtime_paths: Vec<String>,
    pub analysis_logs: String,
}

/// Result of applying a configuration patch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResult {
    pub success: bool,
    pub diff_applied: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_path: Option<String>,
    pub warnings: Vec<String>,
}

/// Lua AST node types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuaNodeType {
    Chunk,
    Statement,
    Expression,
    Function,
    Table,
    String,
    Number,
    Boolean,
    Nil,
    Variable,
    Call,
    Assignment,
    Return,
    If,
    For,
    While,
    Comment,
}

/// Plugin dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub name: String,
    pub version: Option<String>,
    pub optional: bool,
}

/// Plugin event trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEvent {
    pub event: String, // e.g., "VeryLazy", "BufRead", "InsertEnter"
    pub pattern: Option<String>, // file pattern for filetype events
}

/// LazyVim plugin specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazyVimPlugin {
    pub name: String,
    pub spec: HashMap<String, serde_json::Value>, // Flexible plugin spec
    pub dependencies: Vec<PluginDependency>,
    pub events: Vec<PluginEvent>,
    pub config: Option<String>, // Lua config code
    pub enabled: bool,
}

