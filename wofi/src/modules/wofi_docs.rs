use crate::utils::doc_mapper;

/// Get documentation links for a keyword
pub fn get_docs(keyword: &str) -> String {
    doc_mapper::get_docs(keyword)
}

