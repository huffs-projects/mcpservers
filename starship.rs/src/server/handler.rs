use crate::endpoints::{
    starship_apply::{ApplyEndpoint, ApplyRequest},
    starship_options::{OptionsEndpoint, OptionsQuery},
    starship_presets::{PresetsEndpoint, PresetsQuery},
    starship_templates::{TemplatesEndpoint, TemplatesQuery},
    starship_validate::{ValidateEndpoint, ValidateRequest},
};
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::convert::Infallible;

use super::{MCPError, MCPResponse, MCPRequest};

/// Generic endpoint handler trait
trait EndpointHandler: Send + Sync {
    type Request: DeserializeOwned;
    type Response: Serialize;

    async fn handle(&self, params: Self::Request) -> Result<Self::Response>;
}

/// Handler for starship_options endpoint
struct OptionsHandler;

impl EndpointHandler for OptionsHandler {
    type Request = OptionsQuery;
    type Response = Vec<crate::models::StarshipOption>;

    async fn handle(&self, params: Self::Request) -> Result<Self::Response> {
        OptionsEndpoint::query(params).await
    }
}

/// Handler for starship_presets endpoint
struct PresetsHandler;

impl EndpointHandler for PresetsHandler {
    type Request = PresetsQuery;
    type Response = Vec<crate::models::StarshipPreset>;

    async fn handle(&self, params: Self::Request) -> Result<Self::Response> {
        PresetsEndpoint::query(params).await
    }
}

/// Handler for starship_templates endpoint
struct TemplatesHandler;

impl EndpointHandler for TemplatesHandler {
    type Request = TemplatesQuery;
    type Response = Vec<crate::models::TemplateOutput>;

    async fn handle(&self, params: Self::Request) -> Result<Self::Response> {
        TemplatesEndpoint::query(params).await
    }
}

/// Handler for starship_validate endpoint
struct ValidateHandler;

impl EndpointHandler for ValidateHandler {
    type Request = ValidateRequest;
    type Response = crate::models::ValidationResult;

    async fn handle(&self, params: Self::Request) -> Result<Self::Response> {
        ValidateEndpoint::execute(params).await
    }
}

/// Handler for starship_apply endpoint
struct ApplyHandler;

impl EndpointHandler for ApplyHandler {
    type Request = ApplyRequest;
    type Response = crate::models::ApplyResult;

    async fn handle(&self, params: Self::Request) -> Result<Self::Response> {
        ApplyEndpoint::execute(params).await
    }
}

/// Generic handler function that reduces code duplication
async fn handle_endpoint<H: EndpointHandler + Default>(
    params: Value,
) -> MCPResponse
where
    H::Request: DeserializeOwned,
    H::Response: Serialize,
{
    let handler = H::default();
    
    match serde_json::from_value::<H::Request>(params) {
        Ok(request) => {
            match handler.handle(request).await {
                Ok(result) => {
                    match serde_json::to_value(result) {
                        Ok(value) => MCPResponse {
                            result: Some(value),
                            error: None,
                        },
                        Err(e) => MCPResponse {
                            result: None,
                            error: Some(MCPError {
                                code: -32603,
                                message: format!("Serialization error: {}", e),
                            }),
                        },
                    }
                }
                Err(e) => MCPResponse {
                    result: None,
                    error: Some(MCPError {
                        code: -32603,
                        message: format!("Internal error: {}", e),
                    }),
                },
            }
        }
        Err(e) => MCPResponse {
            result: None,
            error: Some(MCPError {
                code: -32602,
                message: format!("Invalid params: {}", e),
            }),
        },
    }
}

impl Default for OptionsHandler {
    fn default() -> Self {
        Self
    }
}

impl Default for PresetsHandler {
    fn default() -> Self {
        Self
    }
}

impl Default for TemplatesHandler {
    fn default() -> Self {
        Self
    }
}

impl Default for ValidateHandler {
    fn default() -> Self {
        Self
    }
}

impl Default for ApplyHandler {
    fn default() -> Self {
        Self
    }
}

pub async fn handle_mcp_request(request: MCPRequest) -> Result<impl warp::Reply, Infallible> {
    let response = match request.method.as_str() {
        "starship_options" => handle_endpoint::<OptionsHandler>(request.params).await,
        "starship_presets" => handle_endpoint::<PresetsHandler>(request.params).await,
        "starship_templates" => handle_endpoint::<TemplatesHandler>(request.params).await,
        "starship_validate" => handle_endpoint::<ValidateHandler>(request.params).await,
        "starship_apply" => handle_endpoint::<ApplyHandler>(request.params).await,
        _ => MCPResponse {
            result: None,
            error: Some(MCPError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
            }),
        },
    };

    Ok(warp::reply::json(&response))
}

