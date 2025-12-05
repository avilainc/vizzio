//! TCP client with TLS support

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use rustls::ClientConfig;
use tokio_rustls::{TlsConnector, client::TlsStream};

/// TCP client that can upgrade to TLS
pub struct TcpClient {
    stream: Option<TcpStream>,
    tls_stream: Option<TlsStream<TcpStream>>,
    addr: SocketAddr,
}

impl TcpClient {
    /// Connect to a TCP server
    pub async fn connect(addr: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self {
            stream: Some(stream),
            tls_stream: None,
            addr,
        })
    }

    /// Upgrade connection to TLS
    pub async fn upgrade_to_tls(&mut self, domain: &str) -> io::Result<()> {
        let stream = self.stream.take().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "No TCP stream available")
        })?;

        // Create TLS config with Mozilla root certificates
        let mut root_store = rustls::RootCertStore::empty();
        root_store.extend(
            webpki_roots::TLS_SERVER_ROOTS
                .iter()
                .cloned()
        );

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(config));

        let server_name = rustls::pki_types::ServerName::try_from(domain)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid DNS name"))?
            .to_owned();

        let tls_stream = connector.connect(server_name, stream).await?;
        self.tls_stream = Some(tls_stream);

        Ok(())
    }

    /// Check if connection is using TLS
    pub fn is_tls(&self) -> bool {
        self.tls_stream.is_some()
    }

    /// Write data to the connection
    pub async fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        if let Some(ref mut tls) = self.tls_stream {
            tls.write_all(data).await
        } else if let Some(ref mut tcp) = self.stream {
            tcp.write_all(data).await
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "No active connection"))
        }
    }

    /// Read data from the connection
    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut tls) = self.tls_stream {
            tls.read(buf).await
        } else if let Some(ref mut tcp) = self.stream {
            tcp.read(buf).await
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "No active connection"))
        }
    }

    /// Read until buffer is full or EOF
    pub async fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut tls) = self.tls_stream {
            tls.read_exact(buf).await?;
            Ok(buf.len())
        } else if let Some(ref mut tcp) = self.stream {
            tcp.read_exact(buf).await?;
            Ok(buf.len())
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "No active connection"))
        }
    }

    /// Flush the connection
    pub async fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut tls) = self.tls_stream {
            tls.flush().await
        } else if let Some(ref mut tcp) = self.stream {
            tcp.flush().await
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "No active connection"))
        }
    }

    /// Get the peer address
    pub fn peer_addr(&self) -> SocketAddr {
        self.addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tcp_connect() {
        // Test requires a running server
        // This is a placeholder for integration tests
    }
}
