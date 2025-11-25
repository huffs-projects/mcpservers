use crate::config::Config;
use crate::endpoints::{
    apply_patch, hm_build, hm_modules, hm_options, hm_templates, health,
};
use crate::error::ServerError;
use crate::metrics::{Metrics, RequestTimer};
use crate::utils::{rate_limit, validation};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum Request {
    #[serde(rename = "hm_options")]
    HmOptions {
        #[serde(default)]
        search_term: Option<String>,
        #[serde(default)]
        module_name: Option<String>,
    },
    #[serde(rename = "hm_modules")]
    HmModules,
    #[serde(rename = "hm_templates")]
    HmTemplates {
        #[serde(default)]
        program_name: Option<String>,
        #[serde(default)]
        use_case: Option<String>,
    },
    #[serde(rename = "hm_build")]
    HmBuild {
        config_path: String,
        #[serde(default = "default_true")]
        dry_run: bool,
        #[serde(default = "default_true")]
        check_deprecated: bool,
    },
    #[serde(rename = "apply_patch")]
    ApplyPatch {
        file_path: String,
        patch: String,
        #[serde(default = "default_true")]
        dry_run: bool,
        #[serde(default)]
        backup_path: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<McpError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

fn default_true() -> bool {
    true
}

pub struct Server {
    request_id: Arc<Mutex<u64>>,
    config: Config,
    metrics: Metrics,
    rate_limiter: Option<Arc<rate_limit::RateLimiterManager>>,
}

impl Server {
    pub fn new() -> Self {
        let config = Config::load(None).unwrap_or_else(|_| Config::default());
        let rate_limiter = if config.rate_limit.enabled {
            Some(Arc::new(rate_limit::RateLimiterManager::new(
                config.rate_limit.requests_per_second,
            )))
        } else {
            None
        };

        Self {
            request_id: Arc::new(Mutex::new(0)),
            config,
            metrics: Metrics::new(),
            rate_limiter,
        }
    }

    pub fn with_config(config: Config) -> Self {
        let rate_limiter = if config.rate_limit.enabled {
            Some(Arc::new(rate_limit::RateLimiterManager::new(
                config.rate_limit.requests_per_second,
            )))
        } else {
            None
        };

        let metrics = Metrics::new();
        crate::metrics::set_global_metrics(metrics.clone());
        
        Self {
            request_id: Arc::new(Mutex::new(0)),
            config,
            metrics,
            rate_limiter,
        }
    }

    pub async fn run(&self) -> Result<()> {
        info!("Home-Manager MCP Server starting...");

        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();
        let mut initialized = false;

        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }

            debug!("Received request: {}", line);

            // Handle MCP protocol initialization
            // Try to parse as JSON first to check if it's a notification (no id) or request
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&line) {
                let method = json_value.get("method").and_then(|m| m.as_str());
                
                if method == Some("initialize") {
                    // This is a request, parse it properly
                    if let Ok(mcp_req) = serde_json::from_str::<McpRequest>(&line) {
                        let response = self.handle_initialize(&mcp_req).await?;
                        let json = serde_json::to_string(&response)?;
                        println!("{}", json);
                        continue;
                    }
                } else if method == Some("initialized") {
                    // This is a notification (no id, no response)
                    initialized = true;
                    info!("MCP client initialized");
                    continue;
                }
            }

            if !initialized {
                warn!("Request received before initialization");
            }

            match self.handle_request(&line).await {
                Ok(response) => {
                    // Only send response if it has an id (not a notification)
                    if response.id.is_some() || response.error.is_some() {
                        let json = serde_json::to_string(&response)?;
                        println!("{}", json);
                    }
                }
                Err(e) => {
                    error!("Error handling request: {}", e);
                    self.metrics.record_error();
                    // Try to extract request ID from the line
                    let request_id = self.extract_request_id(&line);
                    let error_response = self.create_error_response(
                        request_id,
                        ServerError::InternalError(e),
                    );
                    let json = serde_json::to_string(&error_response)?;
                    println!("{}", json);
                }
            }
        }

        Ok(())
    }

    async fn handle_initialize(&self, request: &McpRequest) -> Result<McpResponse> {
        let tools = vec![
            serde_json::json!({
                "name": "hm_options",
                "description": "Query Home-Manager options by name or module",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "search_term": {"type": "string", "description": "Search term to filter options"},
                        "module_name": {"type": "string", "description": "Module name to filter by"}
                    }
                }
            }),
            serde_json::json!({
                "name": "hm_modules",
                "description": "List all Home-Manager modules",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }),
            serde_json::json!({
                "name": "hm_templates",
                "description": "Generate configuration templates for programs",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "program_name": {"type": "string", "description": "Filter by program name"},
                        "use_case": {"type": "string", "description": "Filter by use case"}
                    }
                }
            }),
            serde_json::json!({
                "name": "hm_build",
                "description": "Validate and build Home-Manager configuration",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "config_path": {"type": "string", "description": "Path to Home-Manager config file"},
                        "dry_run": {"type": "boolean", "description": "Perform dry-run (default: true)"},
                        "check_deprecated": {"type": "boolean", "description": "Check for deprecated options (default: true)"}
                    },
                    "required": ["config_path"]
                }
            }),
            serde_json::json!({
                "name": "apply_patch",
                "description": "Apply patches to configuration files",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "file_path": {"type": "string", "description": "Path to file to patch"},
                        "patch": {"type": "string", "description": "Patch content to apply"},
                        "dry_run": {"type": "boolean", "description": "Preview changes without applying (default: true)"},
                        "backup_path": {"type": "string", "description": "Custom backup path"}
                    },
                    "required": ["file_path", "patch"]
                }
            }),
            serde_json::json!({
                "name": "health",
                "description": "Check server health and dependencies",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }),
            serde_json::json!({
                "name": "metrics",
                "description": "Get server metrics and statistics",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }),
        ];

        let capabilities = serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "home-manager-mcp",
                "version": "1.0.0"
            },
            "tools": tools
        });

        Ok(McpResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id.clone(),
            result: Some(capabilities),
            error: None,
        })
    }

    fn extract_request_id(&self, line: &str) -> Option<Value> {
        serde_json::from_str::<McpRequest>(line)
            .ok()
            .and_then(|req| req.id)
    }

    fn create_error_response(&self, id: Option<Value>, error: ServerError) -> McpResponse {
        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(McpError {
                code: error.jsonrpc_code(),
                message: error.error_message(),
                data: None,
            }),
        }
    }

    async fn handle_request(&self, line: &str) -> Result<McpResponse> {
        // Check if this is a notification (no id field)
        let json_value: serde_json::Value = serde_json::from_str(line)
            .map_err(|e| ServerError::ParseError(format!("Failed to parse JSON: {}", e)))?;
        
        // If there's no id, it's a notification - don't respond
        if json_value.get("id").is_none() {
            debug!("Received notification (no id), skipping response");
            return Ok(McpResponse {
                jsonrpc: "2.0".to_string(),
                id: None,
                result: None,
                error: None,
            });
        }
        
        let mcp_req: McpRequest = serde_json::from_str(line)
            .map_err(|e| ServerError::ParseError(format!("Failed to parse JSON: {}", e)))?;

        if mcp_req.jsonrpc != "2.0" {
            return Ok(self.create_error_response(
                mcp_req.id.clone(),
                ServerError::InvalidRequest("jsonrpc must be '2.0'".to_string()),
            ));
        }

        let id = mcp_req.id.clone();
        
        // Track request ID for logging
        if let Some(req_id) = &id {
            debug!("Processing request ID: {}", req_id);
        }

        let result = match mcp_req.method.as_str() {
            "tools/list" => {
                // Return the list of tools
                let tools = vec![
                    serde_json::json!({
                        "name": "hm_options",
                        "description": "Query Home-Manager options by name or module",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "search_term": {"type": "string", "description": "Search term to filter options"},
                                "module_name": {"type": "string", "description": "Module name to filter by"}
                            }
                        }
                    }),
                    serde_json::json!({
                        "name": "hm_modules",
                        "description": "List all Home-Manager modules",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    }),
                    serde_json::json!({
                        "name": "hm_templates",
                        "description": "Generate configuration templates for programs",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "program_name": {"type": "string", "description": "Filter by program name"},
                                "use_case": {"type": "string", "description": "Filter by use case"}
                            }
                        }
                    }),
                    serde_json::json!({
                        "name": "hm_build",
                        "description": "Validate and build Home-Manager configuration",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "config_path": {"type": "string", "description": "Path to Home-Manager config file"},
                                "dry_run": {"type": "boolean", "description": "Perform dry-run (default: true)"},
                                "check_deprecated": {"type": "boolean", "description": "Check for deprecated options (default: true)"}
                            },
                            "required": ["config_path"]
                        }
                    }),
                    serde_json::json!({
                        "name": "apply_patch",
                        "description": "Apply patches to configuration files",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "file_path": {"type": "string", "description": "Path to file to patch"},
                                "patch": {"type": "string", "description": "Patch content to apply"},
                                "dry_run": {"type": "boolean", "description": "Preview changes without applying (default: true)"},
                                "backup_path": {"type": "string", "description": "Custom backup path"}
                            },
                            "required": ["file_path", "patch"]
                        }
                    }),
                    serde_json::json!({
                        "name": "health",
                        "description": "Check server health and dependencies",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    }),
                    serde_json::json!({
                        "name": "metrics",
                        "description": "Get server metrics and statistics",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    }),
                ];
                serde_json::json!({
                    "tools": tools
                })
            }
            "hm_options" => {
                let params: Value = mcp_req.params.unwrap_or(Value::Object(serde_json::Map::new()));
                validation::validate_json_params(&params)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                
                let search_term = validation::extract_string_param(&params, "search_term", Some(1000))
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                let module_name = validation::extract_string_param(&params, "module_name", Some(500))
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;

                let options = timeout(
                    Duration::from_secs(self.config.timeouts.options_query_seconds),
                    hm_options::query_options(
                        search_term.as_deref(),
                        module_name.as_deref(),
                    )
                )
                .await
                .map_err(|_| ServerError::TimeoutError("Options query timed out".to_string()))??;

                serde_json::to_value(options)?
            }
            "hm_modules" => {
                let modules = timeout(
                    Duration::from_secs(self.config.timeouts.modules_list_seconds),
                    hm_modules::list_modules()
                )
                .await
                .map_err(|_| ServerError::TimeoutError("Modules list timed out".to_string()))??;
                serde_json::to_value(modules)?
            }
            "health" => {
                let health_status = timeout(
                    Duration::from_secs(self.config.timeouts.health_seconds),
                    health::check_health()
                )
                .await
                .map_err(|_| ServerError::TimeoutError("Health check timed out".to_string()))??;
                serde_json::to_value(health_status)?
            }
            "metrics" => {
                let stats = self.metrics.get_stats();
                serde_json::to_value(stats)?
            }
            "hm_templates" => {
                let params: Value = mcp_req.params.unwrap_or(Value::Object(serde_json::Map::new()));
                validation::validate_json_params(&params)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                
                let program_name = validation::extract_string_param(&params, "program_name", Some(100))
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                let use_case = validation::extract_string_param(&params, "use_case", Some(200))
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;

                let templates = timeout(
                    Duration::from_secs(self.config.timeouts.templates_seconds),
                    hm_templates::generate_template(
                        program_name.as_deref(),
                        use_case.as_deref(),
                    )
                )
                .await
                .map_err(|_| ServerError::TimeoutError("Template generation timed out".to_string()))??;

                serde_json::to_value(templates)?
            }
            "hm_build" => {
                let params: Value = mcp_req.params
                    .ok_or_else(|| ServerError::InvalidParams("hm_build requires params".to_string()))?;
                
                validation::validate_json_params(&params)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                
                let config_path = validation::extract_required_string_param(&params, "config_path", Some(4096))
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                validation::validate_config_path(&config_path)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                
                let dry_run = validation::extract_bool_param(&params, "dry_run", true)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                let check_deprecated = validation::extract_bool_param(&params, "check_deprecated", true)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;

                let result = timeout(
                    Duration::from_secs(self.config.timeouts.build_seconds),
                    hm_build::build_config(
                        &PathBuf::from(config_path),
                        dry_run,
                        check_deprecated,
                    )
                )
                .await
                .map_err(|_| ServerError::TimeoutError("Build operation timed out".to_string()))??;

                serde_json::to_value(result)?
            }
            "apply_patch" => {
                let params: Value = mcp_req.params
                    .ok_or_else(|| ServerError::InvalidParams("apply_patch requires params".to_string()))?;
                
                validation::validate_json_params(&params)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                
                let file_path = validation::extract_required_string_param(&params, "file_path", Some(4096))
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                let patch = validation::extract_required_string_param(&params, "patch", Some(1_000_000))
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                validation::validate_patch_content(&patch)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                
                let dry_run = validation::extract_bool_param(&params, "dry_run", true)
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;
                let backup_path = validation::extract_string_param(&params, "backup_path", Some(4096))
                    .map_err(|e| ServerError::InvalidParams(e.to_string()))?;

                let backup_path_buf = backup_path.map(PathBuf::from);
                let result = timeout(
                    Duration::from_secs(self.config.timeouts.patch_seconds),
                    apply_patch::apply_patch(
                        &PathBuf::from(&file_path),
                        &patch,
                        dry_run,
                        backup_path_buf.as_ref().map(|p| p.as_path()),
                    )
                )
                .await
                .map_err(|_| ServerError::TimeoutError("Patch operation timed out".to_string()))??;

                serde_json::to_value(result)?
            }
            _ => {
                return Ok(self.create_error_response(
                    id,
                    ServerError::MethodNotFound(mcp_req.method),
                ));
            }
        };

        Ok(McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        })
    }
}

