use crate::core::model::NvimTemplate;
use crate::core::template::TemplateProvider;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Query parameters for nvim_templates endpoint
#[derive(Debug, Deserialize)]
pub struct TemplatesQuery {
    pub use_case: String,
    pub parameters: Option<HashMap<String, String>>,
}

/// Templates endpoint handler
#[derive(Clone)]
pub struct TemplatesEndpoint {
    provider: Arc<RwLock<TemplateProvider>>,
}

impl TemplatesEndpoint {
    pub fn new() -> Self {
        Self {
            provider: Arc::new(RwLock::new(TemplateProvider::new())),
        }
    }

    /// Handle templates query
    pub async fn handle_query(&self, query: TemplatesQuery) -> Result<Vec<NvimTemplate>, String> {
        let provider = self.provider.read().await;
        
        let templates = provider.search_templates(
            &query.use_case,
            query.parameters.as_ref(),
        );

        Ok(templates)
    }
}

impl Default for TemplatesEndpoint {
    fn default() -> Self {
        Self::new()
    }
}

