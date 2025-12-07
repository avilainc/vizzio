//! Request validation utilities
//!
//! Provides validation traits and helpers for API requests.

use crate::error::{ApiError, ApiResult};
use std::collections::HashMap;

/// Trait for validating request types
pub trait Validate {
    /// Validates the request and returns errors if any
    fn validate(&self) -> ApiResult<()>;
}

/// Validation error collector
#[derive(Debug, Default)]
pub struct ValidationErrors {
    errors: HashMap<String, Vec<String>>,
}

impl ValidationErrors {
    /// Creates a new validation error collector
    pub fn new() -> Self {
        Self {
            errors: HashMap::new(),
        }
    }

    /// Adds a validation error for a field
    pub fn add(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.errors
            .entry(field.into())
            .or_insert_with(Vec::new)
            .push(message.into());
    }

    /// Checks if there are any errors
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors
    pub fn len(&self) -> usize {
        self.errors.values().map(|v| v.len()).sum()
    }

    /// Converts to API error if there are validation errors
    pub fn into_result(self) -> ApiResult<()> {
        if self.is_empty() {
            Ok(())
        } else {
            let details = serde_json::to_string(&self.errors)
                .unwrap_or_else(|_| "Validation failed".to_string());
            Err(ApiError::validation("Request validation failed").with_details(details))
        }
    }
}

/// Validator helper
pub struct Validator;

impl Validator {
    /// Validates that a string is not empty
    pub fn not_empty(value: &str, field: &str, errors: &mut ValidationErrors) {
        if value.trim().is_empty() {
            errors.add(field, "must not be empty");
        }
    }

    /// Validates string length
    pub fn length_between(
        value: &str,
        min: usize,
        max: usize,
        field: &str,
        errors: &mut ValidationErrors,
    ) {
        let len = value.len();
        if len < min || len > max {
            errors.add(
                field,
                format!("length must be between {} and {}, got {}", min, max, len),
            );
        }
    }

    /// Validates that a number is in range
    pub fn in_range<T: PartialOrd + std::fmt::Display>(
        value: T,
        min: T,
        max: T,
        field: &str,
        errors: &mut ValidationErrors,
    ) {
        if value < min || value > max {
            errors.add(
                field,
                format!("must be between {} and {}, got {}", min, max, value),
            );
        }
    }

    /// Validates that a value is positive
    pub fn positive<T: PartialOrd + Default + std::fmt::Display>(
        value: T,
        field: &str,
        errors: &mut ValidationErrors,
    ) {
        if value <= T::default() {
            errors.add(field, format!("must be positive, got {}", value));
        }
    }

    /// Validates email format (simple check)
    pub fn email(value: &str, field: &str, errors: &mut ValidationErrors) {
        if !value.contains('@') || !value.contains('.') {
            errors.add(field, "invalid email format");
        }
    }

    /// Validates URL format (simple check)
    pub fn url(value: &str, field: &str, errors: &mut ValidationErrors) {
        if !value.starts_with("http://") && !value.starts_with("https://") {
            errors.add(field, "must be a valid URL starting with http:// or https://");
        }
    }

    /// Validates that a collection is not empty
    pub fn not_empty_vec<T>(value: &[T], field: &str, errors: &mut ValidationErrors) {
        if value.is_empty() {
            errors.add(field, "must not be empty");
        }
    }

    /// Validates collection size
    pub fn vec_size_between<T>(
        value: &[T],
        min: usize,
        max: usize,
        field: &str,
        errors: &mut ValidationErrors,
    ) {
        let len = value.len();
        if len < min || len > max {
            errors.add(
                field,
                format!("size must be between {} and {}, got {}", min, max, len),
            );
        }
    }

    /// Validates that all floats are finite
    pub fn all_finite(value: &[f64], field: &str, errors: &mut ValidationErrors) {
        if value.iter().any(|&x| !x.is_finite()) {
            errors.add(field, "contains invalid values (NaN or Infinity)");
        }
    }

    /// Validates that value is one of allowed options
    pub fn one_of<T: PartialEq + std::fmt::Display>(
        value: &T,
        allowed: &[T],
        field: &str,
        errors: &mut ValidationErrors,
    ) {
        if !allowed.contains(value) {
            errors.add(field, format!("must be one of the allowed values"));
        }
    }

    /// Validates against a simple pattern (basic wildcard support)
    pub fn matches_simple_pattern(
        value: &str,
        pattern: &str,
        field: &str,
        errors: &mut ValidationErrors,
    ) {
        // Simple pattern matching: * means any characters
        let matches = if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                value.starts_with(parts[0]) && value.ends_with(parts[1])
            } else {
                false
            }
        } else {
            value == pattern
        };

        if !matches {
            errors.add(field, format!("does not match pattern: {}", pattern));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_errors() {
        let mut errors = ValidationErrors::new();
        assert!(errors.is_empty());

        errors.add("field1", "error 1");
        errors.add("field1", "error 2");
        errors.add("field2", "error 3");

        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 3);

        let result = errors.into_result();
        assert!(result.is_err());
    }

    #[test]
    fn test_not_empty_validator() {
        let mut errors = ValidationErrors::new();

        Validator::not_empty("test", "field", &mut errors);
        assert!(errors.is_empty());

        Validator::not_empty("", "field", &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_length_validator() {
        let mut errors = ValidationErrors::new();

        Validator::length_between("test", 1, 10, "field", &mut errors);
        assert!(errors.is_empty());

        Validator::length_between("test", 10, 20, "field", &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_range_validator() {
        let mut errors = ValidationErrors::new();

        Validator::in_range(5, 1, 10, "field", &mut errors);
        assert!(errors.is_empty());

        Validator::in_range(15, 1, 10, "field", &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_positive_validator() {
        let mut errors = ValidationErrors::new();

        Validator::positive(5, "field", &mut errors);
        assert!(errors.is_empty());

        Validator::positive(-5, "field", &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_email_validator() {
        let mut errors = ValidationErrors::new();

        Validator::email("test@example.com", "field", &mut errors);
        assert!(errors.is_empty());

        Validator::email("invalid", "field", &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_vec_validators() {
        let mut errors = ValidationErrors::new();

        Validator::not_empty_vec(&[1, 2, 3], "field", &mut errors);
        assert!(errors.is_empty());

        Validator::not_empty_vec(&[] as &[i32], "field", &mut errors);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_finite_validator() {
        let mut errors = ValidationErrors::new();

        Validator::all_finite(&[1.0, 2.0, 3.0], "field", &mut errors);
        assert!(errors.is_empty());

        Validator::all_finite(&[1.0, f64::NAN, 3.0], "field", &mut errors);
        assert!(!errors.is_empty());
    }
}
