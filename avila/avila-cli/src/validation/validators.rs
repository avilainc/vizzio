//! Common validator functions
//!
//! Provides pre-built validators for common validation scenarios.

use std::net::{IpAddr, SocketAddr};
use std::path::Path;

/// Validate that a value is a valid port number (1-65535)
pub fn validate_port(value: &str) -> Result<(), String> {
    value.parse::<u16>()
        .map(|_| ())
        .map_err(|_| "must be a valid port number (1-65535)".to_string())
}

/// Validate that a value is a valid IP address
pub fn validate_ip(value: &str) -> Result<(), String> {
    value.parse::<IpAddr>()
        .map(|_| ())
        .map_err(|_| "must be a valid IP address".to_string())
}

/// Validate that a value is a valid socket address (IP:PORT)
pub fn validate_socket_addr(value: &str) -> Result<(), String> {
    value.parse::<SocketAddr>()
        .map(|_| ())
        .map_err(|_| "must be a valid socket address (IP:PORT)".to_string())
}

/// Validate that a value is a valid URL
pub fn validate_url(value: &str) -> Result<(), String> {
    if value.starts_with("http://") || value.starts_with("https://") {
        Ok(())
    } else {
        Err("must be a valid URL starting with http:// or https://".to_string())
    }
}

/// Validate that a value is a valid email address (basic check)
pub fn validate_email(value: &str) -> Result<(), String> {
    if value.contains('@') && value.contains('.') {
        Ok(())
    } else {
        Err("must be a valid email address".to_string())
    }
}

/// Validate that a path exists
pub fn validate_path_exists(value: &str) -> Result<(), String> {
    if Path::new(value).exists() {
        Ok(())
    } else {
        Err(format!("path '{}' does not exist", value))
    }
}

/// Validate that a path is a file
pub fn validate_is_file(value: &str) -> Result<(), String> {
    let path = Path::new(value);
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("'{}' is not a file", value))
    }
}

/// Validate that a path is a directory
pub fn validate_is_dir(value: &str) -> Result<(), String> {
    let path = Path::new(value);
    if path.is_dir() {
        Ok(())
    } else {
        Err(format!("'{}' is not a directory", value))
    }
}

/// Validate that a value is not empty
pub fn validate_not_empty(value: &str) -> Result<(), String> {
    if !value.trim().is_empty() {
        Ok(())
    } else {
        Err("value cannot be empty".to_string())
    }
}

/// Validate that a value has a minimum length
pub fn validate_min_length(min: usize) -> impl Fn(&str) -> Result<(), String> {
    move |value: &str| {
        if value.len() >= min {
            Ok(())
        } else {
            Err(format!("must be at least {} characters long", min))
        }
    }
}

/// Validate that a value has a maximum length
pub fn validate_max_length(max: usize) -> impl Fn(&str) -> Result<(), String> {
    move |value: &str| {
        if value.len() <= max {
            Ok(())
        } else {
            Err(format!("must be at most {} characters long", max))
        }
    }
}

/// Validate that a value is within a numeric range
pub fn validate_range<T>(min: T, max: T) -> impl Fn(&str) -> Result<(), String>
where
    T: std::str::FromStr + std::cmp::PartialOrd + std::fmt::Display + Copy,
{
    move |value: &str| {
        match value.parse::<T>() {
            Ok(num) if num >= min && num <= max => Ok(()),
            Ok(_) => Err(format!("must be between {} and {}", min, max)),
            Err(_) => Err("must be a valid number".to_string()),
        }
    }
}

/// Validate that a value is valid JSON
pub fn validate_json(value: &str) -> Result<(), String> {
    // Simple JSON validation - checks basic structure
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err("JSON cannot be empty".to_string());
    }

    // Check if it starts and ends with proper brackets
    let valid_start = trimmed.starts_with('{') || trimmed.starts_with('[') ||
                      trimmed.starts_with('"') || trimmed.chars().next().unwrap().is_numeric() ||
                      trimmed.starts_with("true") || trimmed.starts_with("false") ||
                      trimmed.starts_with("null");

    if !valid_start {
        return Err("must be valid JSON".to_string());
    }

    // Basic bracket matching
    let mut stack = Vec::new();
    let mut in_string = false;
    let mut escape_next = false;

    for ch in trimmed.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' | '[' if !in_string => stack.push(ch),
            '}' if !in_string => {
                if stack.pop() != Some('{') {
                    return Err("mismatched brackets in JSON".to_string());
                }
            }
            ']' if !in_string => {
                if stack.pop() != Some('[') {
                    return Err("mismatched brackets in JSON".to_string());
                }
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        return Err("unclosed brackets in JSON".to_string());
    }

    if in_string {
        return Err("unclosed string in JSON".to_string());
    }

    Ok(())
}

/// Validate that a value is a valid UUID (v4 format)
pub fn validate_uuid(value: &str) -> Result<(), String> {
    let parts: Vec<&str> = value.split('-').collect();

    if parts.len() != 5 {
        return Err("UUID must have format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string());
    }

    if parts[0].len() != 8 || parts[1].len() != 4 || parts[2].len() != 4 ||
       parts[3].len() != 4 || parts[4].len() != 12 {
        return Err("UUID has invalid segment lengths".to_string());
    }

    for part in parts {
        if !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("UUID must contain only hexadecimal characters".to_string());
        }
    }

    Ok(())
}

/// Validate that a value is a valid hex color (with or without #)
pub fn validate_hex_color(value: &str) -> Result<(), String> {
    let hex = value.trim_start_matches('#');

    if hex.len() != 3 && hex.len() != 6 {
        return Err("hex color must be 3 or 6 characters (excluding #)".to_string());
    }

    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("hex color must contain only hexadecimal characters (0-9, A-F)".to_string());
    }

    Ok(())
}

/// Validate that a value matches a regex pattern
pub fn validate_regex(pattern: &str) -> impl Fn(&str) -> Result<(), String> + '_ {
    move |value: &str| {
        // Simple pattern matching without regex crate
        // Supports basic patterns: * (any chars), ? (single char)
        if pattern_matches(value, pattern) {
            Ok(())
        } else {
            Err(format!("must match pattern: {}", pattern))
        }
    }
}

/// Validate semantic version (semver) format
pub fn validate_semver(value: &str) -> Result<(), String> {
    let parts: Vec<&str> = value.split('.').collect();

    if parts.len() != 3 {
        return Err("version must be in format: MAJOR.MINOR.PATCH".to_string());
    }

    for (i, part) in parts.iter().enumerate() {
        // Handle pre-release and build metadata
        let clean_part = part.split(&['-', '+'][..]).next().unwrap_or(part);

        if clean_part.parse::<u32>().is_err() {
            let label = match i {
                0 => "MAJOR",
                1 => "MINOR",
                _ => "PATCH",
            };
            return Err(format!("{} version must be a number", label));
        }
    }

    Ok(())
}

/// Validate that a value is alphanumeric only
pub fn validate_alphanumeric(value: &str) -> Result<(), String> {
    if value.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        Err("must contain only letters and numbers".to_string())
    }
}

/// Validate that a value is alphabetic only
pub fn validate_alpha(value: &str) -> Result<(), String> {
    if value.chars().all(|c| c.is_alphabetic()) {
        Ok(())
    } else {
        Err("must contain only letters".to_string())
    }
}

/// Validate that a value is numeric only
pub fn validate_numeric(value: &str) -> Result<(), String> {
    if value.chars().all(|c| c.is_numeric()) {
        Ok(())
    } else {
        Err("must contain only numbers".to_string())
    }
}

/// Validate that a value contains a specific substring
pub fn validate_contains(substring: &str) -> impl Fn(&str) -> Result<(), String> + '_ {
    move |value: &str| {
        if value.contains(substring) {
            Ok(())
        } else {
            Err(format!("must contain '{}'", substring))
        }
    }
}

/// Validate that a value starts with a specific prefix
pub fn validate_starts_with(prefix: &str) -> impl Fn(&str) -> Result<(), String> + '_ {
    move |value: &str| {
        if value.starts_with(prefix) {
            Ok(())
        } else {
            Err(format!("must start with '{}'", prefix))
        }
    }
}

/// Validate that a value ends with a specific suffix
pub fn validate_ends_with(suffix: &str) -> impl Fn(&str) -> Result<(), String> + '_ {
    move |value: &str| {
        if value.ends_with(suffix) {
            Ok(())
        } else {
            Err(format!("must end with '{}'", suffix))
        }
    }
}

/// Combine multiple validators (all must pass)
pub fn validate_all<'a>(
    validators: Vec<Box<dyn Fn(&str) -> Result<(), String> + 'a>>
) -> impl Fn(&str) -> Result<(), String> + 'a {
    move |value: &str| {
        for validator in &validators {
            validator(value)?;
        }
        Ok(())
    }
}

/// Combine multiple validators (at least one must pass)
pub fn validate_any<'a>(
    validators: Vec<Box<dyn Fn(&str) -> Result<(), String> + 'a>>
) -> impl Fn(&str) -> Result<(), String> + 'a {
    move |value: &str| {
        let mut errors = Vec::new();

        for validator in &validators {
            match validator(value) {
                Ok(()) => return Ok(()),
                Err(e) => errors.push(e),
            }
        }

        Err(format!("none of the validations passed: {}", errors.join(", ")))
    }
}

// Helper function for simple pattern matching
fn pattern_matches(text: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    let text_vec: Vec<char> = text.chars().collect();
    let pattern_vec: Vec<char> = pattern.chars().collect();

    fn match_at(text: &[char], pattern: &[char], ti: usize, pi: usize) -> bool {
        if pi >= pattern.len() {
            return ti >= text.len();
        }

        match pattern[pi] {
            '*' => {
                // Match zero characters
                if match_at(text, pattern, ti, pi + 1) {
                    return true;
                }
                // Match one or more characters
                if ti < text.len() {
                    return match_at(text, pattern, ti + 1, pi);
                }
                false
            }
            '?' => {
                if ti >= text.len() {
                    return false;
                }
                match_at(text, pattern, ti + 1, pi + 1)
            }
            c => {
                if ti >= text.len() || text[ti] != c {
                    return false;
                }
                match_at(text, pattern, ti + 1, pi + 1)
            }
        }
    }

    match_at(&text_vec, &pattern_vec, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_port() {
        assert!(validate_port("8080").is_ok());
        assert!(validate_port("65535").is_ok());
        assert!(validate_port("0").is_ok());
        assert!(validate_port("abc").is_err());
        assert!(validate_port("70000").is_err());
    }

    #[test]
    fn test_validate_ip() {
        assert!(validate_ip("127.0.0.1").is_ok());
        assert!(validate_ip("::1").is_ok());
        assert!(validate_ip("invalid").is_err());
    }

    #[test]
    fn test_validate_url() {
        assert!(validate_url("http://example.com").is_ok());
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("ftp://example.com").is_err());
        assert!(validate_url("example.com").is_err());
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
    }

    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty("text").is_ok());
        assert!(validate_not_empty("").is_err());
        assert!(validate_not_empty("   ").is_err());
    }

    #[test]
    fn test_validate_range() {
        let validator = validate_range(1, 100);
        assert!(validator("50").is_ok());
        assert!(validator("1").is_ok());
        assert!(validator("100").is_ok());
        assert!(validator("0").is_err());
        assert!(validator("101").is_err());
        assert!(validator("abc").is_err());
    }

    #[test]
    fn test_validate_json() {
        assert!(validate_json(r#"{"key": "value"}"#).is_ok());
        assert!(validate_json(r#"[1, 2, 3]"#).is_ok());
        assert!(validate_json(r#""string""#).is_ok());
        assert!(validate_json("123").is_ok());
        assert!(validate_json("true").is_ok());
        assert!(validate_json("null").is_ok());
        assert!(validate_json("{unclosed").is_err());
        assert!(validate_json("").is_err());
    }

    #[test]
    fn test_validate_uuid() {
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
        assert!(validate_uuid("invalid-uuid").is_err());
        assert!(validate_uuid("550e8400-e29b-41d4-a716").is_err());
    }

    #[test]
    fn test_validate_hex_color() {
        assert!(validate_hex_color("#FF0000").is_ok());
        assert!(validate_hex_color("#F00").is_ok());
        assert!(validate_hex_color("FF0000").is_ok());
        assert!(validate_hex_color("F00").is_ok());
        assert!(validate_hex_color("#GGGGGG").is_err());
        assert!(validate_hex_color("#FF").is_err());
    }

    #[test]
    fn test_validate_semver() {
        assert!(validate_semver("1.0.0").is_ok());
        assert!(validate_semver("1.2.3").is_ok());
        assert!(validate_semver("1.0.0-alpha").is_ok());
        assert!(validate_semver("1.0.0+build").is_ok());
        assert!(validate_semver("1.0").is_err());
        assert!(validate_semver("1.a.0").is_err());
    }

    #[test]
    fn test_validate_alphanumeric() {
        assert!(validate_alphanumeric("abc123").is_ok());
        assert!(validate_alphanumeric("ABC123").is_ok());
        assert!(validate_alphanumeric("abc-123").is_err());
        assert!(validate_alphanumeric("abc 123").is_err());
    }

    #[test]
    fn test_validate_alpha() {
        assert!(validate_alpha("abc").is_ok());
        assert!(validate_alpha("ABC").is_ok());
        assert!(validate_alpha("abc123").is_err());
    }

    #[test]
    fn test_validate_numeric() {
        assert!(validate_numeric("123").is_ok());
        assert!(validate_numeric("000").is_ok());
        assert!(validate_numeric("abc").is_err());
        assert!(validate_numeric("12.3").is_err());
    }

    #[test]
    fn test_validate_contains() {
        let validator = validate_contains("test");
        assert!(validator("this is a test").is_ok());
        assert!(validator("testing").is_ok());
        assert!(validator("no match").is_err());
    }

    #[test]
    fn test_validate_starts_with() {
        let validator = validate_starts_with("http");
        assert!(validator("http://example.com").is_ok());
        assert!(validator("https://example.com").is_ok());
        assert!(validator("ftp://example.com").is_err());
    }

    #[test]
    fn test_validate_ends_with() {
        let validator = validate_ends_with(".txt");
        assert!(validator("file.txt").is_ok());
        assert!(validator("document.txt").is_ok());
        assert!(validator("file.pdf").is_err());
    }

    #[test]
    fn test_pattern_matching() {
        let validator = validate_regex("test*");
        assert!(validator("test").is_ok());
        assert!(validator("testing").is_ok());
        assert!(validator("test123").is_ok());

        let validator2 = validate_regex("file?.txt");
        assert!(validator2("file1.txt").is_ok());
        assert!(validator2("filea.txt").is_ok());
        assert!(validator2("file12.txt").is_err());
    }
}

