use crate::core::ast::LuaAst;
use crate::core::diagnostics::DiagnosticCollection;
use crate::core::model::ValidationResult;
use crate::core::runtime::NeovimRuntime;
use crate::plugins::lazyvim::LazyVimAnalyzer;
use crate::plugins::plugin_graph::PluginGraph;
use crate::plugins::registry::PluginRegistry;
use regex;
use serde::Deserialize;
use std::path::Path;
use walkdir::WalkDir;

/// Query parameters for nvim_validate endpoint
#[derive(Debug, Deserialize)]
pub struct ValidateQuery {
    pub config_roots: Vec<String>,
}

/// Validation endpoint handler
pub struct ValidateEndpoint {
    ast: LuaAst,
    runtime: NeovimRuntime,
}

impl ValidateEndpoint {
    pub fn new() -> Self {
        Self {
            ast: LuaAst::new(),
            runtime: NeovimRuntime::new(),
        }
    }

    /// Handle validation query
    pub async fn handle_query(&mut self, query: ValidateQuery) -> Result<ValidationResult, String> {
        if query.config_roots.is_empty() {
            return Err("No config roots provided for validation".to_string());
        }

        let mut collection = DiagnosticCollection::new();
        let mut analysis_logs = String::new();

        analysis_logs.push_str(&format!(
            "Starting validation for {} config root(s): {}\n",
            query.config_roots.len(),
            query.config_roots.join(", ")
        ));

        // Stage 1: Syntax validation
        analysis_logs.push_str("Stage 1: Syntax validation\n");
        for root in &query.config_roots {
            self.validate_syntax(root, &mut collection, &mut analysis_logs)
                .map_err(|e| format!(
                    "Syntax validation failed for root {}: {}",
                    root,
                    e
                ))?;
        }

        // Stage 2: Semantic validation
        analysis_logs.push_str(&format!(
            "\nStage 2: Semantic validation (found {} syntax errors so far)\n",
            collection.errors().len()
        ));
        for root in &query.config_roots {
            self.validate_semantics(root, &mut collection, &mut analysis_logs)
                .map_err(|e| format!(
                    "Semantic validation failed for root {}: {}",
                    root,
                    e
                ))?;
        }

        // Stage 3: LazyVim plugin validation
        analysis_logs.push_str(&format!(
            "\nStage 3: LazyVim plugin validation (found {} semantic errors so far)\n",
            collection.errors().len()
        ));
        let (_plugins, missing) = self.validate_plugins(&query.config_roots, &mut collection, &mut analysis_logs)
            .map_err(|e| format!(
                "Plugin validation failed: {}. Config roots: {}",
                e,
                query.config_roots.join(", ")
            ))?;
        
        let unresolved_plugins: Vec<String> = if !missing.is_empty() {
            analysis_logs.push_str(&format!(
                "Found {} unresolved plugin dependencies: {}\n",
                missing.len(),
                missing.join(", ")
            ));
            missing
        } else {
            Vec::new()
        };

        // Stage 4: Runtime path validation
        analysis_logs.push_str("\nStage 4: Runtime path validation\n");
        let missing_runtime_paths: Vec<String> = self.validate_runtime_paths(&query.config_roots, &mut analysis_logs);

        let success = !collection.has_errors();
        let error_count = collection.errors().len();
        let warning_count = collection.warnings().len();

        analysis_logs.push_str(&format!(
            "\nValidation complete: {} errors, {} warnings, {} unresolved plugins, {} missing runtime paths\n",
            error_count,
            warning_count,
            unresolved_plugins.len(),
            missing_runtime_paths.len()
        ));

        Ok(ValidationResult {
            success,
            syntax_errors: collection.errors().iter().map(|d| {
                format!(
                    "[{}] {} (range: {:?}, code: {})",
                    if d.range.0 == 0 && d.range.1 == 0 {
                        "GLOBAL".to_string()
                    } else {
                        format!("{}:{}", d.range.0, d.range.1)
                    },
                    d.message,
                    d.range,
                    d.code.as_ref().unwrap_or(&"unknown".to_string())
                )
            }).collect(),
            semantic_errors: collection
                .errors()
                .iter()
                .map(|d| {
                    format!(
                        "[{}] {} (code: {})",
                        if d.range.0 == 0 && d.range.1 == 0 {
                            "GLOBAL".to_string()
                        } else {
                            format!("{}:{}", d.range.0, d.range.1)
                        },
                        d.message,
                        d.code.as_ref().unwrap_or(&"unknown".to_string())
                    )
                })
                .collect(),
            warnings: collection.warnings().iter().map(|d| {
                format!(
                    "[{}] {} (code: {})",
                    if d.range.0 == 0 && d.range.1 == 0 {
                        "GLOBAL".to_string()
                    } else {
                        format!("{}:{}", d.range.0, d.range.1)
                    },
                    d.message,
                    d.code.as_ref().unwrap_or(&"unknown".to_string())
                )
            }).collect(),
            unresolved_plugins,
            missing_runtime_paths,
            analysis_logs,
        })
    }

    fn validate_syntax(
        &mut self,
        root: &str,
        collection: &mut DiagnosticCollection,
        logs: &mut String,
    ) -> Result<(), String> {
        let root_path = Path::new(root);
        if !root_path.exists() {
            collection.add_error(format!("Config root does not exist: {}", root));
            return Ok(());
        }

        for entry in WalkDir::new(root_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|s| s == "lua").unwrap_or(false))
        {
            let path = entry.path();
            if let Ok(content) = std::fs::read_to_string(path) {
                let diags = self.ast.validate_syntax(&content);
                for diag in diags {
                    collection.add(diag);
                }
                logs.push_str(&format!("Validated: {}\n", path.display()));
            }
        }

        Ok(())
    }

    fn validate_semantics(
        &mut self,
        root: &str,
        collection: &mut DiagnosticCollection,
        _logs: &mut String,
    ) -> Result<(), String> {
        // Check for invalid API calls and unknown options
        let root_path = Path::new(root);
        for entry in WalkDir::new(root_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|s| s == "lua").unwrap_or(false))
        {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                // Check for vim.opt.* assignments
                let opt_re = regex::Regex::new(r#"vim\.opt\.(\w+)"#).unwrap();
                for cap in opt_re.captures_iter(&content) {
                    if let Some(opt_name) = cap.get(1) {
                        if self.runtime.get_option(opt_name.as_str()).is_none() {
                            collection.add_warning(format!(
                                "Unknown option: {} in {}",
                                opt_name.as_str(),
                                entry.path().display()
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn validate_plugins(
        &mut self,
        roots: &[String],
        collection: &mut DiagnosticCollection,
        _logs: &mut String,
    ) -> Result<(PluginRegistry, Vec<String>), String> {
        let mut registry = PluginRegistry::new();
        let mut analyzer = LazyVimAnalyzer::new();

        // Find and parse plugin files
        for root in roots {
            let plugins_dir = Path::new(root).join("lua/plugins");
            if plugins_dir.exists() {
                for entry in WalkDir::new(&plugins_dir)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().map(|s| s == "lua").unwrap_or(false))
                {
                    match analyzer.parse_plugin_file(entry.path()) {
                        Ok(plugin) => {
                            let errors = analyzer.validate_plugin(&plugin);
                            for error in errors {
                                collection.add_error(format!(
                                    "Plugin {}: {}",
                                    plugin.name, error
                                ));
                            }
                            registry.register(plugin);
                        }
                        Err(e) => {
                            collection.add_error(format!(
                                "Failed to parse plugin file {}: {}",
                                entry.path().display(),
                                e
                            ));
                        }
                    }
                }
            }
        }

        // Build plugin graph and check for cycles
        let graph = PluginGraph::from_registry(&registry);
        let cycles = graph.detect_cycles();
        if !cycles.is_empty() {
            collection.add_error(format!("Cyclic dependencies detected: {:?}", cycles));
        }

        // Find missing dependencies
        let mut missing = Vec::new();
        for plugin in registry.get_all_plugins() {
            let missing_deps = registry.find_missing_dependencies(&plugin.name);
            missing.extend(missing_deps);
        }

        Ok((registry, missing))
    }

    fn validate_runtime_paths(&self, roots: &[String], logs: &mut String) -> Vec<String> {
        let mut missing = Vec::new();
        let _runtime_paths = self.runtime.get_runtime_paths();

        for root in roots {
            let root_path = Path::new(root);
            if !self.runtime.validate_runtime_path(&root_path.to_path_buf()) {
                missing.push(root.to_string());
            }
            logs.push_str(&format!("Checked runtime path: {}\n", root));
        }

        missing
    }
}

impl Default for ValidateEndpoint {
    fn default() -> Self {
        Self::new()
    }
}

