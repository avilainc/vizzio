//! AVX API Core
//!
//! Core API types, error handling, and utilities for AVX Gateway services.
//! Built with zero external dependencies (except avx-runtime).
//!
//! ## Features
//!
//! - Native HTTP/1.1 server implementation
//! - Custom JSON serialization/deserialization
//! - Type-safe request/response models
//! - Structured error handling with context
//! - Validation middleware for incoming requests
//! - Forecast utilities for time-series prediction
//! - Health check and metrics endpoints
//!
//! ## Example
//!
//! ```rust,no_run
//! use avx_api_core::{http::Server, http::Router, http::Response, http::StatusCode};
//!
//! let router = Router::new()
//!     .get("/ping", |_req| {
//!         Response::new(StatusCode::OK).with_text("pong")
//!     });
//!
//! let addr = "0.0.0.0:8081".parse().unwrap();
//! Server::bind(addr, router).unwrap().serve().unwrap();
//! ```

pub mod error;
pub mod forecast;
pub mod http;
pub mod json;
pub mod middleware;
pub mod types;
pub mod validation;

// Re-export commonly used types
pub use error::{ApiError, ApiResult};
pub use forecast::{ForecastRequest, ForecastResponse, ForecastService};
pub use http::{Request, Response, Router, Server, StatusCode, Method};
pub use json::{JsonValue, parse as parse_json};
pub use types::{ApiResponse, HealthStatus, StatusInfo};
pub use validation::Validator;

/// Service identification constant
pub const SERVICE_NAME: &str = "avx-api-core";

/// Default HTTP binding address
pub const DEFAULT_BIND_ADDR: &str = "0.0.0.0:8081";
