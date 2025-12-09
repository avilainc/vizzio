//! # Avila AI Proxy
//!
//! Servidor proxy 100% Rust para Ollama, OpenAI e DeepSeek
//! usando APENAS bibliotecas Avila (sem dependências externas).

#![warn(missing_docs)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod config;
mod handlers;
mod models;
mod ollama;

use config::Config;
use handlers::AppContext;

/// Ponto de entrada principal
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Carregar configuração
    let config = Config::from_env()?;

    println!("╔══════════════════════════════════════════╗");
    println!("║     Avila AI Proxy Server v1.0           ║");
    println!("╚══════════════════════════════════════════╝");
    println!();
    println!("📡 Servidor: http://{}:{}", config.host, config.port);
    println!("🤖 Ollama:   {}", config.ollama_url);

    println!("✅ Modo:     100% Local (Ollama apenas)");
    println!("🔒 Zero endpoints externos");    println!();
    println!("Endpoints:");
    println!("  GET  /health");
    println!("  GET  /v1/models");
    println!("  POST /v1/chat/completions");
    println!("  POST /v1/completions");
    println!();

    // Criar contexto compartilhado
    let ctx = AppContext::new(config.clone());

    // Bind TCP listener
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr)?;

    println!("🚀 Servidor rodando em {}", addr);
    println!("   Press Ctrl+C to stop");
    println!();

    // Accept connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Clone context for thread (requires Clone impl)
                let ctx = ctx.clone();

                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream, &ctx) {
                        eprintln!("❌ Erro ao processar request: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("❌ Erro ao aceitar conexão: {}", e);
            }
        }
    }

    Ok(())
}

/// Handle HTTP connection
fn handle_connection(
    mut stream: TcpStream,
    ctx: &AppContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read request
    let mut buffer = [0; 8192];
    let n = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..n]);

    // Parse first line (method + path)
    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 2 {
        send_response(&mut stream, 400, "Bad Request")?;
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    // Route request
    let (status, body) = match (method, path) {
        ("GET", "/health") => {
            match handlers::handle_health() {
                Ok(json) => (200, json),
                Err(e) => (500, format!(r#"{{"error":"{}"}}"#, e)),
            }
        }
        ("GET", "/v1/models") => {
            match handlers::handle_models(ctx) {
                Ok(json) => (200, json),
                Err(e) => (500, format!(r#"{{"error":"{}"}}"#, e)),
            }
        }
        ("POST", "/v1/chat/completions") => {
            // Extract body from request
            let body_start = request.find("\r\n\r\n").unwrap_or(request.len());
            let req_body = &request[body_start..];

            match handlers::handle_chat_completion(ctx, req_body) {
                Ok(json) => (200, json),
                Err(e) => (500, format!(r#"{{"error":"{}"}}"#, e)),
            }
        }
        ("POST", "/v1/completions") => {
            let body_start = request.find("\r\n\r\n").unwrap_or(request.len());
            let req_body = &request[body_start..];

            match handlers::handle_completion(ctx, req_body) {
                Ok(json) => (200, json),
                Err(e) => (500, format!(r#"{{"error":"{}"}}"#, e)),
            }
        }
        _ => (404, r#"{"error":"Not Found"}"#.to_string()),
    };

    send_response(&mut stream, status, &body)?;

    Ok(())
}

/// Send HTTP response
fn send_response(
    stream: &mut TcpStream,
    status: u16,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         \r\n\
         {}",
        status,
        status_text,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

