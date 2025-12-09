# 🏛️ Arquitetura do Vizzio Viewer

## Filosofia: 100% Avila Stack

O Vizzio Viewer segue rigorosamente a filosofia do Avila Stack: **zero dependências externas**, usando apenas componentes nativos da pilha Avila.

## Servidor HTTP: Padrão avila-ai-proxy

### Implementação

O servidor HTTP do vizzio-viewer utiliza o **padrão avila-ai-proxy**, implementado originalmente em `crates/avila-ai-proxy/src/main.rs`.

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub fn start(port: u16) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    for stream in listener.incoming() {
        thread::spawn(|| handle_client(stream));
    }

    Ok(())
}
```

### Características

✅ **Zero Dependências Externas**
- Usa apenas `std::net::TcpListener` (stdlib Rust)
- Sem tokio, hyper, warp, axum ou qualquer framework HTTP
- Thread pool nativo para concorrência

✅ **MIME Types Corretos**
- `application/wasm` para arquivos `.wasm`
- `application/javascript` para `.js`
- `text/html` para `.html`

✅ **Multi-Path Fallback**
```rust
let locations = [
    PathBuf::from(format!("crates/vizzio-viewer/{}", file_path)),
    PathBuf::from(file_path),
];
```

✅ **CORS Habilitado**
```rust
"Access-Control-Allow-Origin: *\r\n"
```

## Stack Completo

### Backend (Rust Native)

```
┌─────────────────────────────────────┐
│     vizzio-viewer (binário)         │
├─────────────────────────────────────┤
│  • HTTP Server (avila-ai-proxy)     │
│  • IFC Parser (avila-bim)           │
│  • Cache (HashMap)                  │
│  • Métricas (Performance)           │
└─────────────────────────────────────┘
```

### Frontend (WASM + WebGL)

```
┌─────────────────────────────────────┐
│    static/index.html                │
├─────────────────────────────────────┤
│  • avila-vision (WASM)              │
│  • WebGL Renderer                   │
│  • WebXR (VR/AR)                    │
│  • UI Controls                      │
└─────────────────────────────────────┘
```

## Pipeline de Processamento

```
┌───────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│ Arquivo   │────▶│  avila-  │────▶│  Extrai  │────▶│  Cache   │
│ .ifc      │     │   bim    │     │ Geometria│     │ (HashMap)│
└───────────┘     └──────────┘     └──────────┘     └──────────┘
                                          │
                                          ▼
┌───────────┐     ┌──────────┐     ┌──────────┐
│  Browser  │◀────│   HTTP   │◀────│  Serve   │
│  (WebGL)  │     │  Server  │     │  WASM    │
└───────────┘     └──────────┘     └──────────┘
```

## Dependências Oficiais

### Cargo.toml
```toml
[dependencies]
# Avila Stack - tudo que precisamos já está aqui!
# Servidor HTTP: Padrão avila-ai-proxy (std::net::TcpListener)
avila-bim = { path = "../avila-bim" }
avila-vision = { path = "../avila-vision" }
avila-error = { path = "../avila-error" }
```

## Comparação com Alternativas

| Feature | Vizzio (Avila) | Alternativas (tokio/hyper) |
|---------|----------------|---------------------------|
| Dependências | 3 (Avila) | 50+ (tokio + hyper + deps) |
| Binário | ~5MB | ~15-30MB |
| Compilação | ~3s | ~30-60s |
| Complexidade | Baixa (134 linhas) | Alta (framework completo) |
| Async/Await | Thread pool | Runtime tokio |

## Referências

### Código Fonte
- **Servidor HTTP**: `crates/vizzio-viewer/src/server.rs` (padrão avila-ai-proxy)
- **Main**: `crates/vizzio-viewer/src/main.rs`
- **Cache**: `crates/vizzio-viewer/src/cache.rs`
- **Parser IFC**: `crates/avila-bim/src/lib.rs`
- **Renderer 3D**: `crates/avila-vision/src/lib.rs`

### Padrão Avila-AI-Proxy
- **Implementação Original**: `crates/avila-ai-proxy/src/main.rs`
- **Padrão**: `std::net::TcpListener` + thread pool
- **Usado por**: vizzio-viewer, avila-ai-proxy, futuros servidores HTTP no Avila Stack

## Performance

### Métricas (IFC 31MB, 522.920 entidades)

```
⏱️  Parse IFC:              218ms
⏱️  Extração geometria:     434ms
⏱️  Total startup:          ~700ms
💾 Cache:                   31.09 MB
📊 Geometrias:              103.718 objetos
```

### Servidor HTTP

```
🌐 Threads:                1 por conexão
⚡ Latência:               <5ms (arquivos estáticos)
📦 WASM load:              ~2-3s (primeira vez)
🔄 Requests:               Multi-thread (std::thread::spawn)
```

## Filosofia de Design

1. **Simplicidade**: 134 linhas de servidor HTTP vs frameworks complexos
2. **Zero Deps**: Apenas stdlib + Avila Stack
3. **Performance**: LTO + opt-level=3 + strip
4. **Manutenibilidade**: Código legível, sem macros complexas
5. **Reutilização**: Padrão avila-ai-proxy usado em múltiplos projetos

---

**Mantido por**: Equipe Avila Stack
**Última atualização**: 2025-12-09
**Versão**: 0.1.0
