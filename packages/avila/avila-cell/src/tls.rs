//! TLS configuration and utilities

use std::sync::Arc;
use rustls::ClientConfig;

/// TLS security mode for SMTP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmtpSecurity {
    /// No TLS (insecure, not recommended)
    None,
    /// STARTTLS - Start with plaintext, upgrade to TLS
    StartTls,
    /// Direct TLS connection (like port 465)
    Tls,
}

/// Create a default TLS client configuration
pub fn default_tls_config() -> ClientConfig {
    let mut root_store = rustls::RootCertStore::empty();
    root_store.extend(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned()
    );

    ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}

/// Create a TLS connector
pub fn create_tls_connector() -> tokio_rustls::TlsConnector {
    tokio_rustls::TlsConnector::from(Arc::new(default_tls_config()))
}
