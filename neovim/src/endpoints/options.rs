use crate::core::model::NvimOption;
use crate::core::runtime::NeovimRuntime;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Query parameters for nvim_options endpoint
#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    pub search: Option<String>,
    pub scope: Option<String>,
}

/// Options endpoint handler
#[derive(Clone)]
pub struct OptionsEndpoint {
    runtime: Arc<RwLock<NeovimRuntime>>,
}

impl OptionsEndpoint {
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(RwLock::new(NeovimRuntime::new())),
        }
    }

    /// Handle options query
    pub async fn handle_query(&self, query: OptionsQuery) -> Result<Vec<NvimOption>, String> {
        let runtime = self.runtime.read().await;

        let options = if let Some(ref search) = query.search {
            runtime.search_options(search)
        } else if let Some(ref scope) = query.scope {
            runtime.get_all_options(Some(scope))
        } else {
            runtime.get_all_options(None)
        };

        Ok(options.into_iter().cloned().collect())
    }
}

impl Default for OptionsEndpoint {
    fn default() -> Self {
        Self::new()
    }
}

