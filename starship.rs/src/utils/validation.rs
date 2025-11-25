use anyhow::Result;

/// Validates string input parameters
pub struct InputValidator;

impl InputValidator {
    /// Maximum length for search terms and category names
    const MAX_SEARCH_LENGTH: usize = 256;
    /// Maximum length for preset/template names
    const MAX_NAME_LENGTH: usize = 128;
    /// Maximum size for TOML patch content (10MB)
    const MAX_PATCH_SIZE: usize = 10 * 1024 * 1024;

    /// Validates a search term
    pub fn validate_search_term(term: &str) -> Result<()> {
        if term.len() > Self::MAX_SEARCH_LENGTH {
            return Err(anyhow::anyhow!(
                "Search term exceeds maximum length of {} characters",
                Self::MAX_SEARCH_LENGTH
            ));
        }

        // Check for null bytes
        if term.contains('\0') {
            return Err(anyhow::anyhow!("Search term cannot contain null bytes"));
        }

        Ok(())
    }

    /// Validates a category name
    pub fn validate_category(category: &str) -> Result<()> {
        if category.len() > Self::MAX_SEARCH_LENGTH {
            return Err(anyhow::anyhow!(
                "Category name exceeds maximum length of {} characters",
                Self::MAX_SEARCH_LENGTH
            ));
        }

        // Category should only contain alphanumeric, underscore, and hyphen
        if !category.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(anyhow::anyhow!(
                "Category name contains invalid characters. Only alphanumeric, underscore, and hyphen are allowed"
            ));
        }

        Ok(())
    }

    /// Validates a preset or template name
    pub fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow::anyhow!("Name cannot be empty"));
        }

        if name.len() > Self::MAX_NAME_LENGTH {
            return Err(anyhow::anyhow!(
                "Name exceeds maximum length of {} characters",
                Self::MAX_NAME_LENGTH
            ));
        }

        // Name should only contain alphanumeric, underscore, and hyphen
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(anyhow::anyhow!(
                "Name contains invalid characters. Only alphanumeric, underscore, and hyphen are allowed"
            ));
        }

        Ok(())
    }

    /// Validates TOML patch content
    pub fn validate_patch(patch: &str) -> Result<()> {
        if patch.is_empty() {
            return Err(anyhow::anyhow!("Patch cannot be empty"));
        }

        if patch.len() > Self::MAX_PATCH_SIZE {
            return Err(anyhow::anyhow!(
                "Patch exceeds maximum size of {} bytes",
                Self::MAX_PATCH_SIZE
            ));
        }

        // Check for null bytes
        if patch.contains('\0') {
            return Err(anyhow::anyhow!("Patch cannot contain null bytes"));
        }

        // Basic TOML validation - check for balanced brackets
        let open_brackets = patch.matches('[').count();
        let close_brackets = patch.matches(']').count();
        if open_brackets != close_brackets {
            return Err(anyhow::anyhow!(
                "Patch has unbalanced brackets: {} opening, {} closing",
                open_brackets,
                close_brackets
            ));
        }

        Ok(())
    }

    /// Validates a string length
    #[allow(dead_code)]
    pub fn validate_string_length(s: &str, max_length: usize, field_name: &str) -> Result<()> {
        if s.len() > max_length {
            return Err(anyhow::anyhow!(
                "{} exceeds maximum length of {} characters",
                field_name,
                max_length
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_search_term_valid() {
        assert!(InputValidator::validate_search_term("git").is_ok());
    }

    #[test]
    fn test_validate_search_term_too_long() {
        let long_term = "a".repeat(300);
        assert!(InputValidator::validate_search_term(&long_term).is_err());
    }

    #[test]
    fn test_validate_category_valid() {
        assert!(InputValidator::validate_category("module").is_ok());
        assert!(InputValidator::validate_category("general").is_ok());
        assert!(InputValidator::validate_category("test-category").is_ok());
        assert!(InputValidator::validate_category("test_category").is_ok());
    }

    #[test]
    fn test_validate_category_invalid() {
        assert!(InputValidator::validate_category("invalid category").is_err());
        assert!(InputValidator::validate_category("invalid@category").is_err());
    }

    #[test]
    fn test_validate_name_valid() {
        assert!(InputValidator::validate_name("test-preset").is_ok());
    }

    #[test]
    fn test_validate_name_empty() {
        assert!(InputValidator::validate_name("").is_err());
    }

    #[test]
    fn test_validate_patch_valid() {
        assert!(InputValidator::validate_patch("[git_branch]\nformat = \"$branch\"").is_ok());
    }

    #[test]
    fn test_validate_patch_empty() {
        assert!(InputValidator::validate_patch("").is_err());
    }

    #[test]
    fn test_validate_patch_unbalanced() {
        assert!(InputValidator::validate_patch("[git_branch\nformat = \"$branch\"").is_err());
    }
}
