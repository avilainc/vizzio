//! Servidor HTTP simples para servir arquivos estáticos
//! Padrão avila-ai-proxy: usa apenas std::net::TcpListener (100% Avila Stack)

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::path::PathBuf;

/// Inicia servidor HTTP
pub fn start(port: u16) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    println!("🌐 Servidor rodando em http://localhost:{}", port);
    println!("🥽 Modo VR: http://localhost:{}?mode=vr", port);
    println!("📲 Modo AR: http://localhost:{}?mode=ar", port);
    println!("\n🛑 Pressione Ctrl+C para parar\n");

    let mut request_count = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                request_count += 1;
                if let Err(e) = handle_client(stream, request_count) {
                    eprintln!("❌ Erro ao processar requisição #{}: {}", request_count, e);
                }
            }
            Err(e) => {
                eprintln!("❌ Erro de conexão: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, req_num: usize) -> std::io::Result<()> {
    let mut buffer = [0u8; 2048];
    let bytes_read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Parseia linha de requisição: GET /path HTTP/1.1
    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 2 {
        println!("⚠️  [{}] Bad Request", req_num);
        send_response(&mut stream, 400, "text/plain", b"Bad Request")?;
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    // Remove query string se existir
    let path_clean = path.split('?').next().unwrap_or(path);

    println!("📥 [{}] {} {}", req_num, method, path);

    // Mapeia path para arquivo
    let file_path = match path_clean {
        "/" | "/index.html" => "static/index.html",
        "/vizzio_viewer.js" => "static/vizzio_viewer.js",
        "/vizzio_viewer_bg.wasm" => "static/vizzio_viewer_bg.wasm",
        _ => {
            println!("🚫 [{}] 404 Not Found: {}", req_num, path_clean);
            send_response(&mut stream, 404, "text/html",
                b"<html><body><h1>404 Not Found</h1><p>Arquivo nao encontrado</p></body></html>")?;
            return Ok(());
        }
    };

    // Tenta múltiplos caminhos possíveis
    let possible_paths = vec![
        PathBuf::from("crates/vizzio-viewer").join(file_path),
        PathBuf::from(file_path),
    ];

    let mut found = false;
    for try_path in &possible_paths {
        if let Ok(contents) = fs::read(try_path) {
            let content_type = match path_clean {
                p if p.ends_with(".html") => "text/html; charset=utf-8",
                p if p.ends_with(".js") => "application/javascript; charset=utf-8",
                p if p.ends_with(".wasm") => "application/wasm",
                _ => "application/octet-stream",
            };

            println!("✅ [{}] 200 OK - {} ({} bytes)", req_num, try_path.display(), contents.len());
            send_response(&mut stream, 200, content_type, &contents)?;
            found = true;
            break;
        }
    }

    if !found {
        println!("🚫 [{}] 404 File Not Found: {}", req_num, file_path);
        send_response(&mut stream, 404, "text/html",
            b"<html><body><h1>404 Not Found</h1><p>Arquivo nao encontrado no servidor</p></body></html>")?;
    }

    Ok(())
}fn send_response(
    stream: &mut TcpStream,
    status_code: u16,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let status_text = match status_code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Unknown",
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Cache-Control: no-cache\r\n\
         \r\n",
        status_code,
        status_text,
        content_type,
        body.len()
    );

    stream.write_all(response.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;

    Ok(())
}
