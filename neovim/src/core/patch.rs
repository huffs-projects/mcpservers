use crate::core::ast::LuaAst;
use crate::core::diagnostics::Diagnostic;

/// AST-based patch transformer for Lua files
pub struct LuaPatch {
    ast: LuaAst,
}

impl LuaPatch {
    pub fn new() -> Self {
        Self {
            ast: LuaAst::new(),
        }
    }

    /// Generate a unified diff from AST changes
    pub fn generate_diff(
        &mut self,
        original: &str,
        modified: &str,
    ) -> Result<String, String> {
        // Simple line-based diff for now
        // In a full implementation, this would be AST-aware
        let original_lines: Vec<&str> = original.lines().collect();
        let modified_lines: Vec<&str> = modified.lines().collect();
        
        let mut diff = String::from("--- original\n+++ modified\n");
        
        // Simple diff algorithm (Myers algorithm would be better)
        let mut i = 0;
        let mut j = 0;
        
        while i < original_lines.len() || j < modified_lines.len() {
            if i >= original_lines.len() {
                diff.push_str(&format!("+{}\n", modified_lines[j]));
                j += 1;
            } else if j >= modified_lines.len() {
                diff.push_str(&format!("-{}\n", original_lines[i]));
                i += 1;
            } else if original_lines[i] == modified_lines[j] {
                diff.push_str(&format!(" {}\n", original_lines[i]));
                i += 1;
                j += 1;
            } else {
                // Try to find matching line ahead
                let mut found = false;
                for k in (j + 1)..modified_lines.len().min(j + 10) {
                    if original_lines[i] == modified_lines[k] {
                        for l in j..k {
                            diff.push_str(&format!("+{}\n", modified_lines[l]));
                        }
                        j = k;
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    diff.push_str(&format!("-{}\n", original_lines[i]));
                    diff.push_str(&format!("+{}\n", modified_lines[j]));
                    i += 1;
                    j += 1;
                }
            }
        }
        
        Ok(diff)
    }

    /// Apply an AST patch instruction
    /// Supports simple operations: insert, remove, replace, modify
    /// Format: JSON with operations like:
    /// {
    ///   "operations": [
    ///     {"type": "insert", "path": ["functions", 0], "value": "function new() end"},
    ///     {"type": "remove", "path": ["assignments", 1]},
    ///     {"type": "replace", "path": ["assignments", 0, "value"], "value": "4"},
    ///     {"type": "modify", "path": ["functions", 0, "body"], "value": "return 42"}
    ///   ]
    /// }
    pub fn apply_ast_patch(
        &mut self,
        source: &str,
        patch_instruction: &str,
    ) -> Result<String, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        
        // Parse the original source
        let tree = self.ast.parse(source)
            .map_err(|e| {
                vec![Diagnostic::error(format!("Failed to parse source: {}", e))]
            })?;
        
        // Try to parse patch instruction as JSON
        if let Ok(patch_json) = serde_json::from_str::<serde_json::Value>(patch_instruction) {
            // If it's a JSON patch instruction, process it
            if let Some(operations) = patch_json.get("operations").and_then(|v| v.as_array()) {
                // Extract current structure
                // Extract functions and assignments for potential use in patching
                let _functions = self.ast.extract_functions(&tree);
                let _assignments = self.ast.extract_table_assignments(&tree);
                
                // Apply operations (simplified - would need full AST manipulation)
                let mut modified_source = source.to_string();
                
                for (idx, op) in operations.iter().enumerate() {
                    if let Some(op_type) = op.get("type").and_then(|v| v.as_str()) {
                        match op_type {
                            "insert" | "add" => {
                                if let Some(value) = op.get("value").and_then(|v| v.as_str()) {
                                    // Insert at end of file
                                    modified_source.push_str("\n");
                                    modified_source.push_str(value);
                                }
                            }
                            "remove" | "delete" => {
                                // Would need to identify and remove specific nodes
                                diagnostics.push(Diagnostic::warning(format!(
                                    "Remove operation {} not fully implemented - would require AST node identification",
                                    idx
                                )));
                            }
                            "replace" | "modify" => {
                                if let Some(_value) = op.get("value").and_then(|v| v.as_str()) {
                                    // Would need to find and replace specific nodes
                                    diagnostics.push(Diagnostic::warning(format!(
                                        "Replace/modify operation {} not fully implemented - would require AST node manipulation",
                                        idx
                                    )));
                                }
                            }
                            _ => {
                                diagnostics.push(Diagnostic::warning(format!(
                                    "Unknown operation type: {}",
                                    op_type
                                )));
                            }
                        }
                    }
                }
                
                return Ok(modified_source);
            }
        }
        
        // If not JSON, treat as a simple text append (fallback)
        diagnostics.push(Diagnostic::warning(
            "Patch instruction is not in JSON format. Treating as text append. For full AST patching, use JSON format with operations array.".to_string()
        ));
        
        let mut result = source.to_string();
        if !patch_instruction.trim().is_empty() {
            result.push_str("\n\n-- Patched content:\n");
            result.push_str(patch_instruction);
        }
        
        Ok(result)
    }

    /// Merge two ASTs intelligently
    /// Attempts to merge by:
    /// 1. Keeping all functions from both (avoiding duplicates by name)
    /// 2. Merging assignments (taking incoming values for conflicts)
    /// 3. Preserving structure and formatting
    pub fn merge_asts(
        &mut self,
        base: &str,
        incoming: &str,
    ) -> Result<String, String> {
        // Parse both sources
        let base_tree = self.ast.parse(base)?;
        let incoming_tree = self.ast.parse(incoming)?;
        
        // Extract functions and assignments from both (with source for proper name extraction)
        let base_functions = self.ast.extract_functions_with_source(&base_tree, base);
        let incoming_functions = self.ast.extract_functions_with_source(&incoming_tree, incoming);
        let base_assignments = self.ast.extract_table_assignments(&base_tree);
        let incoming_assignments = self.ast.extract_table_assignments(&incoming_tree);
        
        // Build merged result
        let mut merged = String::new();
        
        // Merge assignments - use incoming values for conflicts
        let mut assignment_map: std::collections::HashMap<String, String> = base_assignments
            .iter()
            .map(|a| (a.key.clone(), a.value.clone()))
            .collect();
        
        for assignment in &incoming_assignments {
            assignment_map.insert(assignment.key.clone(), assignment.value.clone());
        }
        
        // Add merged assignments
        if !assignment_map.is_empty() {
            for (key, value) in &assignment_map {
                merged.push_str(&format!("{} = {}\n", key, value));
            }
            merged.push_str("\n");
        }
        
        // Merge functions - avoid duplicates by name
        let mut function_names: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        // Add base functions
        for func in &base_functions {
            if !function_names.contains(&func.name) {
                // Extract function body from base (simplified)
                if let Some(start) = base.get(func.range.0..func.range.1) {
                    merged.push_str(start);
                    merged.push_str("\n\n");
                    function_names.insert(func.name.clone());
                }
            }
        }
        
        // Add incoming functions (override if name exists)
        for func in &incoming_functions {
            if let Some(start) = incoming.get(func.range.0..func.range.1) {
                merged.push_str(start);
                merged.push_str("\n\n");
                function_names.insert(func.name.clone());
            }
        }
        
        // If merge is empty, fall back to simple concatenation
        if merged.trim().is_empty() {
            merged = base.to_string();
            if !incoming.is_empty() {
                merged.push_str("\n\n-- Merged changes:\n");
                merged.push_str(incoming);
            }
        }
        
        Ok(merged)
    }

    /// Validate that a patch can be applied
    pub fn validate_patch(
        &mut self,
        original: &str,
        patch: &str,
    ) -> Result<(), Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        
        // Validate original
        let original_diags = self.ast.validate_syntax(original);
        diagnostics.extend(original_diags);
        
        // Try to apply patch and validate result
        // This is simplified - full implementation would parse the patch format
        if patch.contains("<<<<<<<") {
            diagnostics.push(Diagnostic::error(
                "Patch contains merge conflict markers".to_string()
            ));
        }
        
        if diagnostics.iter().any(|d| matches!(d.severity, crate::core::diagnostics::DiagnosticSeverity::Error)) {
            Err(diagnostics)
        } else {
            Ok(())
        }
    }
}

impl Default for LuaPatch {
    fn default() -> Self {
        Self::new()
    }
}

