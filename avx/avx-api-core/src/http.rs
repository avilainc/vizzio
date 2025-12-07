//! HTTP Server Implementation
//!
//! Native HTTP/1.1 server without external dependencies (except avx-runtime).

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;

/// HTTP request method
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
}

impl Method {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "PATCH" => Some(Method::PATCH),
            "OPTIONS" => Some(Method::OPTIONS),
            "HEAD" => Some(Method::HEAD),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::OPTIONS => "OPTIONS",
            Method::HEAD => "HEAD",
        }
    }
}

/// HTTP status code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(pub u16);

impl StatusCode {
    pub const OK: StatusCode = StatusCode(200);
    pub const CREATED: StatusCode = StatusCode(201);
    pub const NO_CONTENT: StatusCode = StatusCode(204);
    pub const BAD_REQUEST: StatusCode = StatusCode(400);
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);
    pub const FORBIDDEN: StatusCode = StatusCode(403);
    pub const NOT_FOUND: StatusCode = StatusCode(404);
    pub const CONFLICT: StatusCode = StatusCode(409);
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode(429);
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);

    pub fn reason_phrase(&self) -> &'static str {
        match self.0 {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            409 => "Conflict",
            429 => "Too Many Requests",
            500 => "Internal Server Error",
            503 => "Service Unavailable",
            _ => "Unknown",
        }
    }

    pub fn is_success(&self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    pub fn is_client_error(&self) -> bool {
        self.0 >= 400 && self.0 < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.0 >= 500 && self.0 < 600
    }
}

/// HTTP headers
pub type Headers = HashMap<String, String>;

/// HTTP request
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Request {
    /// Parse HTTP request from TCP stream
    pub fn parse(stream: &mut TcpStream) -> Result<Self, String> {
        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        // Read request line
        reader.read_line(&mut line).map_err(|e| e.to_string())?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() != 3 {
            return Err("Invalid request line".to_string());
        }

        let method = Method::from_str(parts[0])
            .ok_or_else(|| "Invalid method".to_string())?;
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        // Read headers
        let mut headers = Headers::new();
        loop {
            line.clear();
            reader.read_line(&mut line).map_err(|e| e.to_string())?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                break;
            }

            if let Some(pos) = trimmed.find(':') {
                let key = trimmed[..pos].trim().to_lowercase();
                let value = trimmed[pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        // Read body if Content-Length present
        let body = if let Some(len_str) = headers.get("content-length") {
            let len: usize = len_str.parse().unwrap_or(0);
            let mut buf = vec![0u8; len];
            reader.read_exact(&mut buf).map_err(|e| e.to_string())?;
            buf
        } else {
            Vec::new()
        };

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(&name.to_lowercase())
    }
}

/// HTTP response
#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        let mut headers = Headers::new();
        headers.insert("server".to_string(), "AVX-Core/1.0".to_string());
        headers.insert("connection".to_string(), "close".to_string());

        Self {
            status,
            headers,
            body: Vec::new(),
        }
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.headers.insert("content-length".to_string(), body.len().to_string());
        self.body = body;
        self
    }

    pub fn with_json(mut self, json: &str) -> Self {
        self.headers.insert("content-type".to_string(), "application/json".to_string());
        self.with_body(json.as_bytes().to_vec())
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.headers.insert("content-type".to_string(), "text/plain".to_string());
        self.with_body(text.as_bytes().to_vec())
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_lowercase(), value.to_string());
        self
    }

    pub fn write_to(&self, stream: &mut TcpStream) -> Result<(), String> {
        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.0,
            self.status.reason_phrase()
        );
        stream.write_all(status_line.as_bytes()).map_err(|e| e.to_string())?;

        for (key, value) in &self.headers {
            let header_line = format!("{}: {}\r\n", key, value);
            stream.write_all(header_line.as_bytes()).map_err(|e| e.to_string())?;
        }

        stream.write_all(b"\r\n").map_err(|e| e.to_string())?;
        stream.write_all(&self.body).map_err(|e| e.to_string())?;
        stream.flush().map_err(|e| e.to_string())?;

        Ok(())
    }
}

/// Route handler function type
pub type Handler = Arc<dyn Fn(&Request) -> Response + Send + Sync>;

/// HTTP router
pub struct Router {
    routes: HashMap<(Method, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn route<F>(mut self, method: Method, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.insert((method, path.to_string()), Arc::new(handler));
        self
    }

    pub fn get<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.route(Method::GET, path, handler)
    }

    pub fn post<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.route(Method::POST, path, handler)
    }

    pub fn handle(&self, request: &Request) -> Response {
        let key = (request.method.clone(), request.path.clone());

        if let Some(handler) = self.routes.get(&key) {
            handler(request)
        } else {
            Response::new(StatusCode::NOT_FOUND)
                .with_json(r#"{"error": "Not Found"}"#)
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP server
pub struct Server {
    listener: TcpListener,
    router: Arc<Router>,
}

impl Server {
    pub fn bind(addr: SocketAddr, router: Router) -> Result<Self, String> {
        let listener = TcpListener::bind(addr).map_err(|e| e.to_string())?;
        listener.set_nonblocking(false).map_err(|e| e.to_string())?;

        Ok(Self {
            listener,
            router: Arc::new(router),
        })
    }

    pub fn serve(self) -> Result<(), String> {
        println!("Server listening on {}", self.listener.local_addr().unwrap());

        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let router = Arc::clone(&self.router);

                    // Parse request
                    match Request::parse(&mut stream) {
                        Ok(request) => {
                            let response = router.handle(&request);
                            let _ = response.write_to(&mut stream);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse request: {}", e);
                            let response = Response::new(StatusCode::BAD_REQUEST)
                                .with_text("Bad Request");
                            let _ = response.write_to(&mut stream);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_parsing() {
        assert_eq!(Method::from_str("GET"), Some(Method::GET));
        assert_eq!(Method::from_str("POST"), Some(Method::POST));
        assert_eq!(Method::from_str("INVALID"), None);
    }

    #[test]
    fn test_status_code() {
        assert!(StatusCode::OK.is_success());
        assert!(StatusCode::BAD_REQUEST.is_client_error());
        assert!(StatusCode::INTERNAL_SERVER_ERROR.is_server_error());
    }

    #[test]
    fn test_response_builder() {
        let response = Response::new(StatusCode::OK)
            .with_text("Hello, World!");

        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.body, b"Hello, World!");
    }
}
