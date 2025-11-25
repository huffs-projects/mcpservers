use crate::models::ZshOption;
use crate::utils::schema;

pub fn query_options(search_term: Option<String>, scope: Option<String>) -> Vec<ZshOption> {
    let all_options = schema::get_all_options();
    schema::filter_options(
        &all_options,
        search_term.as_deref(),
        scope.as_deref(),
    )
}

