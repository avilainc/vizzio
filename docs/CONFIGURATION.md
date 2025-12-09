# VIZZIO Workspace Configuration Guide

## 🔧 Configuração Unificada

Este guia explica como VIZZIO integra Rust + Node.js em um único monorepo.

---

## 📂 Sistema de Arquivos

### Cargo.toml (Raiz)
```toml
[workspace]
members = [
    "avila/...",      # Todos os 130+ crates
    "avx/...",        # Todos os 23 crates
]
```

**O que faz:**
- Define todos os crates Rust como um workspace
- Permite `cargo build --workspace`
- Compartilha dependências (ver `[workspace.dependencies]`)

### package.json (Raiz)
```json
{
  "workspaces": [
    "packages/core",
    "packages/workflows",
    "packages/backend",
    "..."
  ]
}
```

**O que faz:**
- Define todos os packages Node.js como um workspace
- Permite `npm install` uma única vez
- Instala dependências em `node_modules` raiz (hoisted)

---

## 🔄 Como Funciona o Build

### Cenário 1: Build Apenas Rust

```bash
cargo build --workspace --release
```

**O que acontece:**
1. Rust compila todas as crates em `Cargo.toml`
2. Gera binários em `target/release/`
3. Cria artifacts para uso

**Tempo esperado:** 10-30 min (primeira vez)

### Cenário 2: Build Apenas Node

```bash
npm install
npm run build --workspaces
```

**O que acontece:**
1. npm instala todas as dependências
2. Compila TypeScript → JavaScript
3. Gera bundles em `packages/*/dist/`

**Tempo esperado:** 5-10 min

### Cenário 3: Build Completo

```bash
npm run build:all
```

**O que faz:**
1. `npm run build:rust` → Rust compilation
2. `npm run build:node` → Node.js compilation
3. Ambos rodam sequencialmente

**Tempo esperado:** 15-40 min

---

## 🚀 Estrutura de Desenvolvimento

### Workspace Rust (Cargo.toml)

```
vizzio/
├── Cargo.toml
├── avila/
│   ├── avila-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── avila-db/
│   │   ├── Cargo.toml
│   │   └── src/
│   └── ... (128+ mais)
└── avx/
    ├── avx-gpu/
    │   ├── Cargo.toml
    │   └── src/
    └── ... (22+ mais)
```

**Regra:** Cada crate tem seu próprio `Cargo.toml`

### Workspace Node.js (package.json)

```
vizzio/
├── package.json
└── packages/
    ├── core/
    │   ├── package.json
    │   ├── src/
    │   └── dist/
    ├── workflows/
    │   ├── package.json
    │   ├── src/
    │   └── dist/
    └── ... (11+ mais)
```

**Regra:** Cada package tem seu próprio `package.json`

---

## 🔗 Integrando Rust + Node.js

### Opção A: Subprocess

Node chama Rust como programa externo:

```typescript
// Node.js code
import { spawn } from 'child_process';

const rust_process = spawn('avila-processor', ['--input', data]);

rust_process.stdout.on('data', (output) => {
  console.log('Rust output:', output.toString());
});
```

```rust
// Rust code (avila-processor/src/main.rs)
fn main() {
    let input = std::env::args().nth(1);
    println!("{:?}", process(input));
}
```

**Prós:** Simples, isolado, escalável
**Contras:** Overhead de processo

---

### Opção B: WASM

Compilar Rust para WebAssembly:

```bash
# Instalar
cargo install wasm-pack

# Compilar crate para WASM
cd avila/avila-core-workspace
wasm-pack build --target nodejs
```

Node.js pode usar:

```typescript
// Node.js code
import * as avila from './avila_core_wasm.js';

const result = avila.process_data(data);
```

**Prós:** Nativa no Node, fast, sem IPC overhead
**Contras:** Precisa estrutura especial, FFI limitations

---

### Opção C: HTTP/gRPC

Rust roda como serviço separado:

```rust
// avila-service/src/main.rs
use axum::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/process", post(process_handler));

    Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

Node.js chama via HTTP:

```typescript
// Node.js code
const response = await fetch('http://localhost:5000/process', {
  method: 'POST',
  body: JSON.stringify(data)
});
```

**Prós:** Escalável, versioning fácil, multi-language
**Contras:** Latência de rede, overhead serialização

---

### Opção D: FFI (Foreign Function Interface)

Usar `node-ffi` ou similar:

```typescript
// Node.js
import ffi from 'ffi-napi';
import ref from 'ref-napi';

const lib = ffi.Library('./target/release/libavila.so', {
  process: ['int', ['string']]
});

const result = lib.process('data');
```

```rust
// Rust (avila/lib.rs)
#[no_mangle]
pub extern "C" fn process(input: *const c_char) -> i32 {
    // Processing
    42
}
```

**Prós:** Muito rápido, direto
**Contras:** Precisa unsafe code, complexo

---

## 📦 Dependências Compartilhadas

### Rust
Definir em `Cargo.toml` workspace:

```toml
[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
# ... todas as crates usam essas versões
```

Usar em qualquer crate:

```toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
```

### Node.js
Instalar no root:

```bash
npm install --workspace=packages express
```

Todas as packages herdam via workspace hoisting.

---

## 🏗️ Padrão de Organização

### Por Camada (Recomendado)

```
vizzio/
├── avila/
│   ├── core/          ← Infrastructure
│   ├── network/       ← Communication
│   ├── crypto/        ← Security
│   ├── ml/            ← Intelligence
│   └── ...
│
├── avx/
│   ├── gpu/           ← High performance
│   ├── quantum/       ← Advanced
│   └── ...
│
└── packages/
    ├── core/          ← Types
    ├── backend/       ← API
    ├── frontend/      ← UI
    └── ...
```

### Por Domínio (Alternativa)

```
vizzio/
├── domains/
│   ├── payments/
│   │   ├── backend/          (Express handlers)
│   │   ├── types/            (TypeScript types)
│   │   ├── processor/        (Rust crate)
│   │   └── ...
│   ├── marketing/
│   │   ├── backend/
│   │   ├── ml/               (Rust ML)
│   │   ├── worker/           (Bull queue)
│   │   └── ...
│   └── ...
```

---

## 🔄 Workflow de Dependências

### Exemplo: Feature "Advanced Analytics"

```
packages/frontend
  └─ precisa de tipos
     └─ @vizzio/core
        └─ precisa de tipos
           └─ avila/avila-dataframe
              └─ precisa de Rust compilation
                 └─ cargo build avila-dataframe
```

**Build order:**
1. Compilar `avila-dataframe` (Rust)
2. Gerar TypeScript bindings (via WASM/FFI)
3. Compilar `@vizzio/core` (TypeScript)
4. Compilar `packages/frontend` (React)

---

## ⚡ Performance Tips

### Rust
```bash
# Build mais rápido
cargo build --jobs 4          # Menos paralelismo
cargo build -j 8              # Mais paralelismo

# Incremental builds
cargo check                    # Sem linking
cargo build --incremental

# Profile otimizado
cargo build -Z timings        # Ver tempos
```

### Node.js
```bash
# npm workspace install rápido
npm ci                        # CI mode (reproducible)
npm install --legacy-peer-deps

# npm workspaces
npm install --workspace=packages/core
npm run build --workspace=packages/core
```

---

## 🔍 Debugging

### Rust
```bash
# WASM debugging
wasm-pack build --dev --target nodejs

# Rust backtrace
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run

# Clippy warnings
cargo clippy -- -D warnings
```

### Node.js
```bash
# Debug mode
node --inspect packages/backend/dist/index.js

# Verbose npm
npm run build -- --verbose

# Check dependencies
npm ls @vizzio/core
```

---

## 📝 Best Practices

### ✅ DO

- ✅ Use shared `Cargo.toml` dependencies
- ✅ Use npm workspaces for Node packages
- ✅ Organize by feature/domain
- ✅ Keep Rust separate from Node (different tech stacks)
- ✅ Use Docker for isolation
- ✅ Version releases together (monorepo versioning)

### ❌ DON'T

- ❌ Don't have circular dependencies
- ❌ Don't ignore workspace.dependencies
- ❌ Don't hardcode paths (use relative paths)
- ❌ Don't mix Rust/Node in same crate
- ❌ Don't forget to update Cargo.toml when adding crates
- ❌ Don't use `npm install` in individual packages

---

## 🎯 Checklist para Nova Feature

- [ ] Criar Rust crate em `avila/` ou `avx/`
- [ ] Adicionar ao `Cargo.toml` workspace
- [ ] Criar Node.js package em `packages/`
- [ ] Adicionar ao `package.json` workspace
- [ ] Definir exports/bindings
- [ ] Escrever testes (Rust + Node)
- [ ] Documentar em README.md
- [ ] Adicionar ao CHANGELOG.md
- [ ] Commit & Push

---

**VIZZIO Configuration Guide** ✨
