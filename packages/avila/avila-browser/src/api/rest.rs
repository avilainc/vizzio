//! REST API implementation

use std::net::SocketAddr;

/// REST API server
pub struct RestApi {
    addr: SocketAddr,
}

impl RestApi {
    pub fn new(port: u16) -> Self {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        Self { addr }
    }

    /// Start the REST API server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting REST API on {}", self.addr);
        // TODO: Implement REST API using axum or actix-web
        Ok(())
    }

    /// Stop the REST API server
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement graceful shutdown
        Ok(())
    }
}

/// REST API endpoints:
///
/// GET    /api/v1/status          - Get browser status
/// POST   /api/v1/navigate        - Navigate to URL
/// GET    /api/v1/screenshot      - Take screenshot
/// POST   /api/v1/execute         - Execute JavaScript
/// GET    /api/v1/cookies         - Get cookies
/// POST   /api/v1/cookies         - Set cookie
/// DELETE /api/v1/cookies/:name   - Delete cookie
/// GET    /api/v1/layers          - Get active security layers
/// POST   /api/v1/layers          - Enable/disable layers
