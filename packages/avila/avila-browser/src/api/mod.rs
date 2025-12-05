//! Public APIs for browser control
//!
//! Provides REST API, gRPC, and WebDriver protocol support

pub mod rest;
pub mod grpc;
pub mod webdriver;

pub use rest::RestApi;
pub use grpc::GrpcServer;
pub use webdriver::WebDriverSession;

/// API configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub rest_enabled: bool,
    pub rest_port: u16,
    pub grpc_enabled: bool,
    pub grpc_port: u16,
    pub webdriver_enabled: bool,
    pub webdriver_port: u16,
    pub require_authentication: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            rest_enabled: true,
            rest_port: 8080,
            grpc_enabled: false,
            grpc_port: 50051,
            webdriver_enabled: true,
            webdriver_port: 4444,
            require_authentication: true,
        }
    }
}
