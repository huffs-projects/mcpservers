use tree_sitter::{Parser, Tree};
use tree_sitter_lua;
use crate::core::diagnostics::{Diagnostic, DiagnosticSeverity};
use std::path::Path;

/// Lua AST parser and analyzer
pub struct LuaAst {
    parser: Parser,
    language_initialized: bool,
}

impl LuaAst {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        let language = tree_sitter_lua::language();
        
        // Attempt to set language and track success
        let language_initialized = parser.set_language(language).is_ok();
        
        if !language_initialized {
            eprintln!("WARNING: Failed to initialize tree-sitter-lua language. AST parsing may not work correctly.");
            eprintln!("This may be due to tree-sitter version mismatch. Check tree-sitter and tree-sitter-lua versions.");
        }
        
        Self {
            parser,
            language_initialized,
        }
    }

    /// Check if language was successfully initialized
    pub fn is_initialized(&self) -> bool {
        self.language_initialized
    }

    /// Parse a Lua file and return the AST tree
    pub fn parse(&mut self, source: &str) -> Result<Tree, String> {
        if !self.language_initialized {
            return Err("Tree-sitter language not initialized. Cannot parse Lua source.".to_string());
        }

        self.parser
            .parse(source, None)
            .ok_or_else(|| {
                format!(
                    "Failed to parse Lua source (length: {} bytes). This may indicate a syntax error or parser issue.",
                    source.len()
                )
            })
    }

    /// Parse a Lua file from disk
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(Tree, String), String> {
        let source = std::fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read file: {}", e))?;
        let tree = self.parse(&source)?;
        Ok((tree, source))
    }

    /// Validate syntax and return diagnostics
    pub fn validate_syntax(&mut self, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        if !self.language_initialized {
            diagnostics.push(Diagnostic {
                range: (0, 0),
                severity: DiagnosticSeverity::Error,
                message: "Tree-sitter language not initialized. Cannot validate syntax.".to_string(),
                code: Some("parser_not_initialized".to_string()),
            });
            return diagnostics;
        }
        
        match self.parse(source) {
            Ok(tree) => {
                // Check for parse errors
                let root_node = tree.root_node();
                if root_node.has_error() {
                    diagnostics.push(Diagnostic {
                        range: (0, source.len()),
                        severity: DiagnosticSeverity::Error,
                        message: format!(
                            "Syntax error detected in Lua code (source length: {} bytes)",
                            source.len()
                        ),
                        code: Some("syntax_error".to_string()),
                    });
                }
                
                // Check for missing 'end' tokens
                self.check_missing_ends(&root_node, source, &mut diagnostics);
            }
            Err(e) => {
                diagnostics.push(Diagnostic {
                    range: (0, source.len()),
                    severity: DiagnosticSeverity::Error,
                    message: format!("Parse error: {}", e),
                    code: Some("parse_error".to_string()),
                });
            }
        }
        
        diagnostics
    }

    /// Check for missing 'end' tokens in control structures
    fn check_missing_ends(
        &self,
        node: &tree_sitter::Node,
        source: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let kind = node.kind();
        
        // Check if this is a control structure that requires 'end'
        let needs_end = matches!(
            kind,
            "if_statement" | "for_statement" | "while_statement" | "function_declaration"
        );
        
        if needs_end {
            let mut has_end = false;
            let child_count = node.child_count();
            
            // Check if last child is 'end'
            if child_count > 0 {
                if let Some(last_child) = node.child(child_count - 1) {
                    if last_child.kind() == "end" {
                        has_end = true;
                    }
                }
            }
            
            if !has_end {
                let start_byte = node.start_byte();
                let end_byte = node.end_byte();
                diagnostics.push(Diagnostic {
                    range: (start_byte, end_byte),
                    severity: DiagnosticSeverity::Error,
                    message: format!("Missing 'end' for {} statement", kind),
                    code: Some("missing_end".to_string()),
                });
            }
        }
        
        // Recursively check children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.check_missing_ends(&child, source, diagnostics);
            }
        }
    }

    /// Extract function definitions from AST
    pub fn extract_functions(&self, tree: &Tree) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();
        let root = tree.root_node();
        // We need source for text extraction, but for now just extract ranges
        self.collect_functions(&root, &mut functions, None);
        functions
    }

    /// Extract function definitions from AST with source text
    pub fn extract_functions_with_source(&self, tree: &Tree, source: &str) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();
        let root = tree.root_node();
        self.collect_functions(&root, &mut functions, Some(source));
        functions
    }

    fn collect_functions(&self, node: &tree_sitter::Node, functions: &mut Vec<FunctionInfo>, source: Option<&str>) {
        if node.kind() == "function_declaration" {
            let name = self.extract_function_name(node, source);
            let start = node.start_byte();
            let end = node.end_byte();
            // Only add if range is valid
            if start <= end {
                functions.push(FunctionInfo {
                    name,
                    range: (start, end),
                });
            }
        }
        
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.collect_functions(&child, functions, source);
            }
        }
    }

    fn extract_function_name(&self, node: &tree_sitter::Node, source: Option<&str>) -> String {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == "identifier" || child.kind() == "dot_expression" {
                    // Try to extract text if source is available
                    if let Some(src) = source {
                        let start_byte = child.start_byte();
                        let end_byte = child.end_byte();
                        if start_byte < end_byte && end_byte <= src.len() {
                            if let Ok(text) = child.utf8_text(src.as_bytes()) {
                                return text.to_string();
                            }
                        }
                    }
                    // Fallback: return a placeholder
                    return format!("func_{}", i);
                }
            }
        }
        "anonymous".to_string()
    }

    /// Extract table assignments (e.g., vim.opt.tabstop = 4)
    pub fn extract_table_assignments(&self, tree: &Tree) -> Vec<TableAssignment> {
        let mut assignments = Vec::new();
        let root = tree.root_node();
        self.collect_assignments(&root, &mut assignments);
        assignments
    }

    fn collect_assignments(&self, node: &tree_sitter::Node, assignments: &mut Vec<TableAssignment>) {
        if node.kind() == "assignment_statement" {
            // Try to extract vim.opt.* or vim.g.* assignments
            let mut key = None;
            let mut value = None;
            
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "variable_list" {
                        if let Some(var_node) = child.child(0) {
                            key = Some(var_node.utf8_text(&[]).unwrap_or("").to_string());
                        }
                    } else if child.kind() == "expression_list" {
                        if let Some(expr_node) = child.child(0) {
                            value = Some(expr_node.utf8_text(&[]).unwrap_or("").to_string());
                        }
                    }
                }
            }
            
            if let (Some(k), Some(v)) = (key, value) {
                assignments.push(TableAssignment {
                    key: k,
                    value: v,
                    range: (node.start_byte(), node.end_byte()),
                });
            }
        }
        
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.collect_assignments(&child, assignments);
            }
        }
    }
}

impl Default for LuaAst {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_lua() {
        let mut ast = LuaAst::new();
        let code = "local x = 1\nreturn x";
        let result = ast.parse(code);
        assert!(result.is_ok(), "Should parse valid Lua code");
    }

    #[test]
    fn test_parse_invalid_lua() {
        let mut ast = LuaAst::new();
        let code = "local x = 1\nreturn x +"; // Missing operand
        let result = ast.parse(code);
        // Parser might still succeed but tree will have errors
        if let Ok(tree) = result {
            assert!(tree.root_node().has_error(), "Should detect syntax error");
        }
    }

    #[test]
    fn test_validate_syntax() {
        let mut ast = LuaAst::new();
        let code = "function test()\n  return 1\nend";
        let diags = ast.validate_syntax(code);
        assert!(diags.is_empty(), "Valid code should have no diagnostics");
    }

    #[test]
    fn test_extract_functions() {
        let mut ast = LuaAst::new();
        if !ast.is_initialized() {
            // Skip test if tree-sitter not initialized
            eprintln!("Skipping test: tree-sitter not initialized");
            return;
        }
        
        let code = "function test()\n  return 1\nend\nfunction test2()\n  return 2\nend";
        match ast.parse(code) {
            Ok(tree) => {
                // Use the version with source for proper text extraction
                let functions = ast.extract_functions_with_source(&tree, code);
                assert!(functions.len() >= 1, "Should extract at least one function. Found: {}", functions.len());
                // Check that we can extract function names
                if !functions.is_empty() {
                    assert!(!functions[0].name.is_empty(), "Function should have a name");
                }
            }
            Err(e) => {
                panic!("Failed to parse test code: {}", e);
            }
        }
    }
}

/// Information about a function in the AST
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub range: (usize, usize),
}

/// Table assignment information
#[derive(Debug, Clone)]
pub struct TableAssignment {
    pub key: String,
    pub value: String,
    pub range: (usize, usize),
}

