//! Middleware components for request processing
//!
//! Native middleware implementation without external dependencies.

use crate::http::{Request, Response};
use std::time::{SystemTime, UNIX_EPOCH};

/// Request ID header name
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Timing header name
pub const TIMING_HEADER: &str = "x-response-time";

/// Generates a simple request ID
pub fn generate_request_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros())
        .unwrap_or(0);
    
    let random = (timestamp % 1000000) as u32;
    format!("req-{:016x}-{:06x}", timestamp, random)
}

/// Extracts request ID from headers or generates a new one
pub fn get_or_generate_request_id(req: &Request) -> String {
    req.header(REQUEST_ID_HEADER)
        .map(|s| s.clone())
        .unwrap_or_else(generate_request_id)
}

/// Adds CORS headers to response
pub fn add_cors_headers(mut response: Response) -> Response {
    response
        .with_header("access-control-allow-origin", "*")
        .with_header("access-control-allow-methods", "GET, POST, PUT, DELETE, OPTIONS")
        .with_header("access-control-allow-headers", "content-type, authorization, x-request-id")
}

/// Adds security headers to response
pub fn add_security_headers(mut response: Response) -> Response {
    response
        .with_header("x-frame-options", "DENY")
        .with_header("x-content-type-options", "nosniff")
        .with_header("strict-transport-security", "max-age=31536000; includeSubDomains")
        .with_header("content-security-policy", "default-src 'self'")
}

/// Request tracing middleware
///
/// Adds request ID and logs request/response information
pub async fn request_tracing(mut request: Request, next: Next) -> Response {
    let start = Instant::now();

    // Get or generate request ID
    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();

    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        version = ?version,
        "incoming request"
    );

    // Insert request ID into extensions for handler access
    request.extensions_mut().insert(RequestId(request_id.clone()));

    let mut response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    // Add timing and request ID headers
    response.headers_mut().insert(
        REQUEST_ID_HEADER,
        request_id.parse().unwrap_or_else(|_| {
            warn!("Failed to parse request ID as header value");
            "invalid".parse().unwrap()
        }),
    );

    response.headers_mut().insert(
        TIMING_HEADER,
        format!("{}ms", duration.as_millis())
            .parse()
            .unwrap_or_else(|_| "0ms".parse().unwrap()),
    );

    if status.is_server_error() {

/// Rate limiting configuration
#[derive(Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: usize,
    pub burst_size: usize,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_size: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id_generation() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        
        assert!(id1.starts_with("req-"));
        assert!(id2.starts_with("req-"));
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.burst_size, 10);
    }
}
