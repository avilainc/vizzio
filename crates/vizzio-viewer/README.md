# 🏗️ Vizzio Viewer MVP

Visualizador de projetos IFC em 3D/VR/AR escala 1:1.

## 🚀 Características

- ✅ **Parser IFC STEP** nativo em Rust (zero deps externas)
- ✅ **Renderer 3D WebGL** via WebAssembly
- 🔄 **Suporte VR/AR** com WebXR (em desenvolvimento)
- 🎨 **Interface web moderna** e responsiva
- 📱 **Multiplataforma** (Desktop, Web, Mobile)

## 📦 Estrutura

```
vizzio-viewer/
├── src/
│   ├── main.rs          # CLI binário
│   └── lib.rs           # Biblioteca WASM
├── static/
│   └── index.html       # Interface web
└── Cargo.toml
```

## 🛠️ Compilação

### Desktop (CLI)
```bash
cargo build --release
cargo run
```

### WebAssembly
```bash
# Instala wasm-pack se necessário
cargo install wasm-pack

# Compila para WASM
cd crates/vizzio-viewer
wasm-pack build --target web --release

# Serve o app
python -m http.server 8080 -d static
```

Acesse: `http://localhost:8080`

## 📖 Uso

### CLI
```bash
# Coloque arquivos .ifc na pasta do projeto
./vizzio-viewer
```

### Web
1. Abra `http://localhost:8080`
2. Clique em "📁 Carregar IFC"
3. Selecione um arquivo .ifc
4. Use o mouse para orbitar a câmera
5. Use scroll para zoom

### VR/AR
- **VR**: Clique em "🥽 Modo VR" (requer headset compatível)
- **AR**: Clique em "📲 Modo AR" (requer dispositivo compatível)

## 🎮 Controles

| Ação | Desktop | VR | AR |
|------|---------|----|----|
| Orbitar | Mouse drag | Head movement | Device movement |
| Zoom | Scroll | Controller | Pinch |
| Selecionar | Click | Controller trigger | Tap |

## 🏗️ Arquitetura

### Crates Utilizadas

- `avila-bim`: Parser IFC STEP format
- `avila-vision`: Engine de renderização 3D
- `avila-error`: Sistema de erros unificado
- **Servidor HTTP**: Padrão `avila-ai-proxy` (std::net::TcpListener)
  - Zero dependências externas (sem tokio/hyper/warp)
  - 100% Avila Stack

### Servidor HTTP

O vizzio-viewer utiliza o **padrão avila-ai-proxy** para servidor HTTP:

- Implementação nativa com `std::net::TcpListener`
- Sem dependências externas (tokio, hyper, warp, etc)
- Thread pool para conexões concorrentes
- MIME types corretos para WASM/JS/HTML
- Multi-path fallback para arquivos estáticos

Referência: `crates/avila-ai-proxy/src/main.rs`

### Pipeline de Renderização

```
IFC File → Parser → Geometria → Triangulação → WebGL → Display
```

## 🔬 Testes

```bash
# Testa parser IFC
cargo test -p avila-bim

# Testa renderer
cargo test -p avila-vision

# Testa aplicação
cargo test -p vizzio-viewer
```

## 📝 Roadmap MVP

- [x] Parser IFC básico (STEP format)
- [x] Extração de geometria (Wall, Slab, Beam, Column)
- [x] Renderer WebGL com shaders
- [x] Câmera perspectiva (orbit, zoom)
- [x] Interface web responsiva
- [ ] Carregar modelos IFC reais
- [ ] Renderizar geometria completa
- [ ] Suporte WebXR VR
- [ ] Suporte WebXR AR
- [ ] Medidas em tempo real
- [ ] Escala 1:1 precisa

## 🤝 Contribuindo

Este é um projeto da **Avila Inc** seguindo a filosofia:
> "Não dependemos de ninguém. Construímos tudo do zero, do bit ao pixel."

Todas as crates são implementações próprias sem dependências externas.

## 📄 Licença

Propriedade da Avila Inc © 2025

## 📞 Suporte

- Docs: https://docs.vizzio.ai
- Email: avilaops@vizzio.ai
