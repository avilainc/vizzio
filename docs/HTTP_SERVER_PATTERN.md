# 🌐 Padrão HTTP Server - Avila Stack

## Visão Geral

O **padrão avila-ai-proxy** é o padrão oficial para servidores HTTP no Avila Stack. Utiliza apenas `std::net::TcpListener` da biblioteca padrão Rust, sem dependências externas.

## Características

✅ **Zero Dependências**
- Apenas `std::net` e `std::io`
- Sem tokio, hyper, warp, axum
- Binários menores e compilação mais rápida

✅ **Thread Pool Nativo**
- `std::thread::spawn` para cada conexão
- Concorrência sem runtime async
- Simples e efetivo

✅ **Completo**
- Parsing HTTP request
- Routing (match method + path)
- MIME types
- CORS
- Status codes

## Implementação Base

### Servidor Básico

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub fn start(port: u16) -> std::io::Result<()> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr)?;

    println!("🌐 Servidor rodando em http://localhost:{}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("❌ Erro: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("❌ Erro de conexão: {}", e),
        }
    }

    Ok(())
}
```

### Handler de Conexão

```rust
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // Read request
    let mut buffer = [0; 8192];
    let n = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..n]);

    // Parse first line
    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 2 {
        send_response(&mut stream, 400, "Bad Request", b"", "text/plain")?;
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    // Routing
    let (status, body, mime) = match (method, path) {
        ("GET", "/") => (200, b"Hello, World!" as &[u8], "text/plain"),
        ("GET", "/health") => (200, b"{\"status\":\"ok\"}", "application/json"),
        _ => (404, b"Not Found", "text/plain"),
    };

    send_response(&mut stream, status, status_text(status), body, mime)?;

    Ok(())
}
```

### Envio de Resposta

```rust
fn send_response(
    stream: &mut TcpStream,
    status: u16,
    status_text: &str,
    body: &[u8],
    content_type: &str,
) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         \r\n",
        status, status_text, content_type, body.len()
    );

    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;

    Ok(())
}

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}
```

## Exemplos de Uso

### 1. API JSON (avila-ai-proxy)

```rust
// crates/avila-ai-proxy/src/main.rs
match (method, path) {
    ("GET", "/health") => {
        let json = r#"{"status":"ok"}"#;
        send_response(&mut stream, 200, "OK", json.as_bytes(), "application/json")?;
    }
    ("POST", "/v1/chat/completions") => {
        let body = extract_body(&request);
        let response = handle_chat(body)?;
        send_response(&mut stream, 200, "OK", response.as_bytes(), "application/json")?;
    }
    _ => send_response(&mut stream, 404, "Not Found", b"{\"error\":\"Not Found\"}", "application/json")?,
}
```

### 2. Arquivos Estáticos (vizzio-viewer)

```rust
// crates/vizzio-viewer/src/server.rs
fn serve_static_file(stream: &mut TcpStream, path: &str) -> std::io::Result<()> {
    let file_path = if path == "/" { "static/index.html" } else { &path[1..] };

    let locations = [
        PathBuf::from(format!("crates/vizzio-viewer/{}", file_path)),
        PathBuf::from(file_path),
    ];

    for location in &locations {
        if location.exists() {
            let content = fs::read(location)?;
            let mime = get_mime_type(location);
            send_response(stream, 200, "OK", &content, mime)?;
            return Ok(());
        }
    }

    send_response(stream, 404, "Not Found", b"File not found", "text/plain")?;
    Ok(())
}

fn get_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("js") => "application/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}
```

## Projetos que Usam o Padrão

| Projeto | Localização | Propósito |
|---------|-------------|-----------|
| **avila-ai-proxy** | `crates/avila-ai-proxy/src/main.rs` | API JSON para Ollama/OpenAI |
| **vizzio-viewer** | `crates/vizzio-viewer/src/server.rs` | Servidor de arquivos estáticos (WASM) |

## Performance

### Benchmarks (comparação informal)

| Métrica | avila-ai-proxy | hyper + tokio |
|---------|----------------|---------------|
| Binário | ~5MB | ~15MB |
| Compilação | ~3s | ~30s |
| Latência | ~1-2ms | ~1-2ms |
| Throughput | ~10k req/s | ~20k req/s |
| Dependências | 0 | 50+ |

**Conclusão**: Para casos de uso do Avila Stack (APIs internas, servidores de desenvolvimento), o padrão avila-ai-proxy oferece excelente relação simplicidade/performance.

## Quando Usar

✅ **Use o padrão avila-ai-proxy quando:**
- Servidor HTTP para desenvolvimento
- APIs internas (não público)
- Servidores de arquivos estáticos
- Provas de conceito
- Performance não é crítica (<10k req/s)
- Quer evitar dependências pesadas

❌ **Considere alternativas quando:**
- Servidor público com alto tráfego (>100k req/s)
- Necessita HTTP/2 ou HTTP/3
- Precisa de features avançadas (websockets, streaming, etc)
- Já tem infraestrutura tokio

## Boas Práticas

### 1. Timeout em Leituras

```rust
stream.set_read_timeout(Some(Duration::from_secs(5)))?;
```

### 2. Limite de Buffer

```rust
const MAX_REQUEST_SIZE: usize = 8192;
let mut buffer = [0; MAX_REQUEST_SIZE];
```

### 3. Logging Estruturado

```rust
println!("📥 [{}] {} {}", request_num, method, path);
println!("✅ [{}] 200 OK - {} bytes", request_num, content.len());
```

### 4. Tratamento de Erros

```rust
match stream.read(&mut buffer) {
    Ok(0) => return Ok(()), // Cliente fechou conexão
    Ok(n) => { /* processar */ }
    Err(e) => {
        eprintln!("❌ Erro de leitura: {}", e);
        return Err(e);
    }
}
```

## Extensões Futuras

### Possíveis Melhorias (mantendo zero deps)

- [ ] Thread pool fixo (vs thread por conexão)
- [ ] Keep-alive (conexões persistentes)
- [ ] Chunked transfer encoding
- [ ] Basic auth
- [ ] Rate limiting simples

## Referências

- **RFC 2616**: HTTP/1.1 Specification
- **std::net docs**: https://doc.rust-lang.org/std/net/
- **avila-ai-proxy**: Implementação de referência

---

**Status**: ✅ Padrão Oficial do Avila Stack
**Primeira Implementação**: avila-ai-proxy v1.0
**Mantido por**: Equipe Avila Stack
**Última atualização**: 2025-12-09
