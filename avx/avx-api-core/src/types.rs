//! Core API types for requests and responses
//!
//! Provides standardized types for API communication - native implementation.

use crate::json::JsonValue;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Standard API response wrapper
#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    /// Indicates whether the operation was successful
    pub success: bool,
    /// Response data (present on success)
    pub data: Option<T>,
    /// Error information (present on failure)
    pub error: Option<ErrorInfo>,
    /// Optional metadata
    pub meta: Option<ResponseMeta>,
}

impl<T> ApiResponse<T> {
    /// Creates a successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: None,
        }
    }

    /// Creates a successful response with metadata
    pub fn success_with_meta(data: T, meta: ResponseMeta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(meta),
        }
    }

    /// Creates an error response
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorInfo {
                code: code.into(),
                message: message.into(),
                details: None,
            }),
            meta: None,
        }
    }
}

/// Error information structure
#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

/// Response metadata
#[derive(Debug, Clone)]
pub struct ResponseMeta {
    /// Request timestamp
    pub timestamp: Option<i64>,
    /// Processing duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Request ID for tracing
    pub request_id: Option<String>,
    /// API version
    pub version: Option<String>,
}

impl ResponseMeta {
    /// Creates new metadata with current timestamp
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .ok();

        Self {
            timestamp,
            duration_ms: None,
            request_id: None,
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }
    }

    /// Sets the request ID
    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = Some(id.into());
        self
    }

    /// Sets the processing duration
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }
}

impl Default for ResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

/// Health check status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Degraded => write!(f, "degraded"),
            Self::Unhealthy => write!(f, "unhealthy"),
        }
    }
}

/// Service status information
#[derive(Debug, Clone)]
pub struct StatusInfo {
    /// Service name
    pub service: String,
    /// Current health status
    pub status: String,
    /// Service version
    pub version: String,
    /// Optional uptime in seconds
    pub uptime_seconds: Option<u64>,
    /// Optional additional info
    pub details: Option<JsonValue>,
}

impl StatusInfo {
    /// Creates a new status info
    pub fn new(service: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            status: HealthStatus::Healthy.to_string(),
            version: version.into(),
            uptime_seconds: None,
            details: None,
        }
    }

    /// Sets the health status
    pub fn with_status(mut self, status: HealthStatus) -> Self {
        self.status = status.to_string();
        self
    }

    /// Sets the uptime
    pub fn with_uptime(mut self, seconds: u64) -> Self {
        self.uptime_seconds = Some(seconds);
        self
    }

    /// Adds additional details
    pub fn with_details(mut self, details: JsonValue) -> Self {
        self.details = Some(details);
        self
    }
}

/// Pagination information
#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub total_pages: usize,
}

impl Pagination {
    /// Creates pagination info
    pub fn new(page: usize, per_page: usize, total: usize) -> Self {
        let total_pages = (total + per_page - 1) / per_page;
        Self {
            page,
            per_page,
            total,
            total_pages,
        }
    }
}

/// Paginated response
#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    /// Creates a new paginated response
    pub fn new(items: Vec<T>, page: usize, per_page: usize, total: usize) -> Self {
        Self {
            items,
            pagination: Pagination::new(page, per_page, total),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = ApiResponse::success("test data");
        assert!(response.success);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_error_response() {
        let response: ApiResponse<()> = ApiResponse::error("TEST_ERROR", "Test failed");
        assert!(!response.success);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn test_status_info() {
        let status = StatusInfo::new("test-service", "1.0.0")
            .with_status(HealthStatus::Healthy)
            .with_uptime(3600);

        assert_eq!(status.service, "test-service");
        assert_eq!(status.status, "healthy");
        assert_eq!(status.uptime_seconds, Some(3600));
    }

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(1, 10, 95);
        assert_eq!(pagination.total_pages, 10);
        assert_eq!(pagination.page, 1);
    }
}
