//! Low-level networking primitives
//!
//! Provides abstractions over TCP/UDP sockets, TLS, and connection pooling

pub mod tcp;
pub mod udp;
pub mod tls;
pub mod pooling;

pub use tcp::TcpStream;
pub use udp::UdpSocket;
pub use tls::TlsConnector;
pub use pooling::ConnectionPool;

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub connect_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub write_timeout_ms: u64,
    pub max_connections: usize,
    pub enable_keepalive: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            connect_timeout_ms: 10_000,
            read_timeout_ms: 30_000,
            write_timeout_ms: 30_000,
            max_connections: 100,
            enable_keepalive: true,
        }
    }
}
