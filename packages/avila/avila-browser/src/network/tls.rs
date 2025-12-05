//! TLS 1.3 connector

use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::time::Duration;
use super::TcpStream;

/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub verify_certificates: bool,
    pub certificate_pinning: bool,
    pub min_protocol_version: TlsVersion,
    pub supported_cipher_suites: Vec<CipherSuite>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
    Tls12,
    Tls13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherSuite {
    Tls13Aes128GcmSha256,
    Tls13Aes256GcmSha384,
    Tls13Chacha20Poly1305Sha256,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            verify_certificates: true,
            certificate_pinning: false,
            min_protocol_version: TlsVersion::Tls13,
            supported_cipher_suites: vec![
                CipherSuite::Tls13Aes256GcmSha384,
                CipherSuite::Tls13Chacha20Poly1305Sha256,
            ],
        }
    }
}

/// TLS connector
pub struct TlsConnector {
    config: TlsConfig,
}

impl TlsConnector {
    pub fn new(config: TlsConfig) -> Self {
        Self { config }
    }

    /// Connect to server with TLS
    pub fn connect(
        &self,
        domain: &str,
        addr: SocketAddr,
        timeout: Duration,
    ) -> io::Result<TlsStream> {
        // TODO: Implement TLS connection using rustls
        let tcp_stream = TcpStream::connect(addr, timeout)?;
        Ok(TlsStream {
            inner: tcp_stream,
            domain: domain.to_string(),
        })
    }
}

/// TLS encrypted stream
pub struct TlsStream {
    inner: TcpStream,
    domain: String,
}

impl Read for TlsStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
