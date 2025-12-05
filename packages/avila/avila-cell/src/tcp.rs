//! TCP client implementation

use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;

/// TCP client
pub struct TcpClient {
    stream: TcpStream,
    addr: SocketAddr,
}

impl TcpClient {
    /// Connect to a TCP server
    pub async fn connect(addr: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self { stream, addr })
    }

    /// Write data to the connection
    pub async fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_all(data).await
    }

    /// Read data from the connection
    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf).await
    }

    /// Read until buffer is full or EOF
    pub async fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read_exact(buf).await?;
        Ok(buf.len())
    }

    /// Flush the connection
    pub async fn flush(&mut self) -> io::Result<()> {
        self.stream.flush().await
    }

    /// Get the peer address
    pub fn peer_addr(&self) -> SocketAddr {
        self.addr
    }
}#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tcp_connect() {
        // Test requires a running server
        // This is a placeholder for integration tests
    }
}
