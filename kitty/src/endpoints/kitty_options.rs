use crate::models::KittyOption;
use crate::utils::KittySchema;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    pub search_term: Option<String>,
    pub category: Option<String>,
}

pub async fn handle_kitty_options(query: OptionsQuery) -> Vec<KittyOption> {
    let schema = KittySchema::global();
    
    if let Some(search) = &query.search_term {
        schema
            .search_options(search, query.category.as_deref())
            .into_iter()
            .cloned()
            .collect()
    } else if let Some(category) = &query.category {
        schema
            .get_all_options()
            .into_iter()
            .filter(|opt| opt.category.to_lowercase() == category.to_lowercase())
            .cloned()
            .collect()
    } else {
        schema
            .get_all_options()
            .into_iter()
            .cloned()
            .collect()
    }
}

