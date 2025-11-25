/// Validation utilities for Mako configuration values

/// Validate hex color format (#RRGGBB or #RRGGBBAA)
pub fn validate_color(value: &str) -> bool {
    if let Some(hex_part) = value.strip_prefix('#') {
        (hex_part.len() == 6 || hex_part.len() == 8)
            && hex_part.chars().all(|c| c.is_ascii_hexdigit())
    } else if value.starts_with("over ") {
        // Handle "over #color #color" format for progress-color
        let parts: Vec<&str> = value.split_whitespace().collect();
        parts.len() == 3
            && parts[1].starts_with('#')
            && parts[2].starts_with('#')
            && validate_color(parts[1])
            && validate_color(parts[2])
    } else {
        false
    }
}

/// Validate that a path is a valid file path (basic check)
pub fn validate_path(value: &str) -> bool {
    !value.is_empty() && !value.contains('\0')
}

/// Validate integer range
pub fn validate_integer_range(value: &str, min: i32, max: i32) -> Result<i32, String> {
    match value.parse::<i32>() {
        Ok(n) => {
            if n >= min && n <= max {
                Ok(n)
            } else {
                Err(format!("Value {} is out of range [{}, {}]", n, min, max))
            }
        }
        Err(e) => Err(format!("Invalid integer: {}", e)),
    }
}

/// Validate positive integer
pub fn validate_positive_integer(value: &str) -> Result<i32, String> {
    validate_integer_range(value, 1, i32::MAX)
}

/// Validate non-negative integer
pub fn validate_non_negative_integer(value: &str) -> Result<i32, String> {
    validate_integer_range(value, 0, i32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_color_hex() {
        assert!(validate_color("#285577"));
        assert!(validate_color("#ffffff"));
        assert!(validate_color("#000000"));
        assert!(validate_color("#12345678")); // With alpha
        assert!(!validate_color("285577"));
        assert!(!validate_color("#gggggg"));
        assert!(!validate_color("#12345"));
    }

    #[test]
    fn test_validate_color_over() {
        assert!(validate_color("over #00ff00 #00ff00"));
        assert!(!validate_color("over #00ff00"));
        assert!(!validate_color("over invalid"));
    }

    #[test]
    fn test_validate_path() {
        assert!(validate_path("/path/to/file"));
        assert!(validate_path("file.txt"));
        assert!(!validate_path(""));
        assert!(!validate_path("path\0with\0null"));
    }

    #[test]
    fn test_validate_integer_range() {
        assert_eq!(validate_integer_range("5", 1, 10), Ok(5));
        assert_eq!(validate_integer_range("0", 0, 10), Ok(0));
        assert!(validate_integer_range("15", 1, 10).is_err());
        assert!(validate_integer_range("abc", 1, 10).is_err());
    }
}

