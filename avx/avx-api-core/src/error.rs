//! Error types for AVX API Core
//!
//! Provides structured error handling with rich context and HTTP status code mappings.

use crate::http::StatusCode;
use crate::json::JsonValue;
use std::fmt;

/// Result type alias for API operations
pub type ApiResult<T> = Result<T, ApiError>;

/// Comprehensive error type for API operations
#[derive(Debug, Clone)]
pub struct ApiError {
    /// HTTP status code
    pub status: StatusCode,
    /// Error code for programmatic handling
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Optional detailed context
    pub details: Option<String>,
    /// Request ID for tracing
    pub request_id: Option<String>,
}

impl ApiError {
    /// Creates a new API error
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status,
            code: code.into(),
            message: message.into(),
            details: None,
            request_id: None,
        }
    }

    /// Adds detailed context to the error
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Adds request ID for tracing
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Creates a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message)
    }

    /// Creates a not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            "NOT_FOUND",
            format!("Resource not found: {}", resource.into()),
        )
    }

    /// Creates an internal server error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", message)
    }

    /// Creates an unauthorized error
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", message)
    }

    /// Creates a forbidden error
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "FORBIDDEN", message)
    }

    /// Creates a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, "CONFLICT", message)
    }

    /// Creates a rate limit error
    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, "RATE_LIMIT_EXCEEDED", message)
    }

    /// Creates a service unavailable error
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, "SERVICE_UNAVAILABLE", message)
    }

    /// Converts error to JSON value
    pub fn to_json(&self) -> JsonValue {
        let mut obj = vec![
            ("code", JsonValue::String(self.code.clone())),
            ("message", JsonValue::String(self.message.clone())),
        ];

        if let Some(details) = &self.details {
            obj.push(("details", JsonValue::String(details.clone())));
        }

        if let Some(request_id) = &self.request_id {
            obj.push(("request_id", JsonValue::String(request_id.clone())));
        }

        JsonValue::object(obj)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;
        if let Some(details) = &self.details {
            write!(f, " - {}", details)?;
        }
        Ok(())
    }
}

impl std::error::Error for ApiError {}

/// Conversion from std::io::Error
impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::internal(format!("I/O error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = ApiError::validation("Invalid input");
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.code, "VALIDATION_ERROR");
        assert_eq!(err.message, "Invalid input");
    }

    #[test]
    fn test_error_with_details() {
        let err = ApiError::not_found("user")
            .with_details("User ID: 123")
            .with_request_id("req-xyz");

        assert!(err.details.is_some());
        assert!(err.request_id.is_some());
    }

    #[test]
    fn test_error_display() {
        let err = ApiError::internal("Database connection failed")
            .with_details("Connection timeout after 30s");

        let display = format!("{}", err);
        assert!(display.contains("INTERNAL_ERROR"));
        assert!(display.contains("Database connection failed"));
        assert!(display.contains("Connection timeout"));
    }
}
