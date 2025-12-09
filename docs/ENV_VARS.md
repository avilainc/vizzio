# Avila Stack Environment Variables

Este documento descreve todas as variáveis de ambiente utilizadas pelas bibliotecas Avila.

## 🔧 AVILA CORE

### Logging e Observabilidade
| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_LOG_LEVEL` | `info` | Nível de log: trace, debug, info, warn, error |
| `AVILA_LOG_FORMAT` | `json` | Formato: json, text, pretty |
| `AVILA_METRICS_ENABLED` | `true` | Habilita coleta de métricas |
| `AVILA_METRICS_PORT` | `9090` | Porta para exportar métricas Prometheus |
| `AVILA_TRACE_ENABLED` | `true` | Habilita distributed tracing |

## 💾 AVILA DATABASE (avila-db)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_DB_PATH` | `./data/avila.db` | Caminho do arquivo do banco |
| `AVILA_DB_CACHE_SIZE` | `1024` | Tamanho do cache em MB |
| `AVILA_DB_WAL_ENABLED` | `true` | Write-Ahead Logging (durabilidade) |
| `AVILA_DB_MAX_CONNECTIONS` | `100` | Máximo de conexões simultâneas |
| `AVILA_DB_POOL_SIZE` | `10` | Tamanho do pool de conexões |

## 🔒 AVILA CRYPTO (avila-crypto, avila-aead, avila-jwt)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_CRYPTO_KEY` | - | **Chave de 256 bits para AEAD (ChaCha20-Poly1305)** |
| `AVILA_JWT_SECRET` | - | **Secret para assinar JWT tokens** |
| `AVILA_JWT_EXPIRY` | `3600` | Tempo de expiração do token (segundos) |
| `AVILA_KDF_ITERATIONS` | `100000` | Iterações PBKDF2 para derivação de chaves |
| `AVILA_TLS_CERT` | `./certs/server.crt` | Certificado TLS X.509 |
| `AVILA_TLS_KEY` | `./certs/server.key` | Chave privada TLS |

**⚠️ IMPORTANTE**: Troque `AVILA_CRYPTO_KEY` e `AVILA_JWT_SECRET` em produção!

## 🌐 AVILA HTTP (avila-http)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_HTTP_HOST` | `0.0.0.0` | Interface de rede para bind |
| `AVILA_HTTP_PORT` | `3000` | Porta HTTP/HTTPS |
| `AVILA_HTTP_WORKERS` | `4` | Número de worker threads |
| `AVILA_HTTP_TIMEOUT` | `30` | Timeout de requests (segundos) |
| `AVILA_HTTP_MAX_BODY_SIZE` | `10485760` | Tamanho máximo do body (10MB) |

## 📡 AVILA GRPC (avila-grpc)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_GRPC_HOST` | `0.0.0.0` | Interface de rede |
| `AVILA_GRPC_PORT` | `50051` | Porta gRPC |
| `AVILA_GRPC_MAX_MESSAGE_SIZE` | `4194304` | Tamanho máximo de mensagem (4MB) |
| `AVILA_GRPC_KEEPALIVE_INTERVAL` | `60` | Keepalive ping (segundos) |

## 🏗️ AVILA BIM (avila-bim)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_BIM_IFC_PATH` | `./*.ifc` | Padrão glob para arquivos IFC |
| `AVILA_BIM_OUTPUT_DIR` | `./output/models` | Diretório para glTF exportados |
| `AVILA_BIM_GLTF_COMPRESSION` | `true` | Compressão Draco para meshes |
| `AVILA_BIM_4D_ENABLED` | `true` | Habilita 4D scheduling (timeline) |

## 🗺️ AVILA GEO (avila-geo)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_GEO_PROJ_DB` | `./data/proj.db` | Database de projeções cartográficas |
| `AVILA_GEO_TILE_CACHE` | `./cache/tiles` | Cache de tiles vetoriais/raster |
| `AVILA_GEO_MAX_ZOOM` | `18` | Zoom máximo para tiles |
| `AVILA_GEO_TILE_SIZE` | `256` | Tamanho do tile (pixels) |

## ⚡ AVILA ASYNC (avila-async)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_ASYNC_WORKERS` | `num_cpus` | Threads do runtime (usa CPU cores) |
| `AVILA_ASYNC_STACK_SIZE` | `2097152` | Tamanho da stack por task (2MB) |
| `AVILA_ASYNC_EVENT_INTERVAL` | `61` | Intervalo de polling epoll (ms) |

## 📊 AVL CONSOLE (avl-console)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVL_CONSOLE_HOST` | `0.0.0.0` | Interface web |
| `AVL_CONSOLE_PORT` | `8888` | Porta do dashboard |
| `AVL_METRICS_RETENTION` | `30d` | Retenção de métricas |
| `AVL_LOG_RETENTION` | `7d` | Retenção de logs |
| `AVL_TRACE_SAMPLE_RATE` | `0.1` | Taxa de amostragem de traces (10%) |

## 🔄 AVILA COORDINATOR (avila-coordinator)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_COORDINATOR_RAFT_PORT` | `7000` | Porta Raft consensus |
| `AVILA_COORDINATOR_HEARTBEAT` | `500` | Heartbeat interval (ms) |
| `AVILA_COORDINATOR_ELECTION_TIMEOUT` | `1500` | Timeout para eleição de leader (ms) |
| `AVILA_COORDINATOR_NODE_ID` | `node-1` | ID único do nó no cluster |

## 💾 AVILA CACHE (avila-cache)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_CACHE_MAX_SIZE` | `1073741824` | Tamanho máximo (1GB) |
| `AVILA_CACHE_TTL` | `3600` | TTL padrão (segundos) |
| `AVILA_CACHE_EVICTION_POLICY` | `lru` | Política: lru, lfu, fifo |

## 🤖 AVILA ML (avila-ml)

| Variável | Padrão | Descrição |
|----------|--------|-----------|
| `AVILA_ML_DEVICE` | `cpu` | Device: cpu, cuda, rocm |
| `AVILA_ML_BATCH_SIZE` | `32` | Tamanho do batch para inferência |
| `AVILA_ML_NUM_THREADS` | `4` | Threads para operações BLAS |
| `AVILA_ML_MODEL_PATH` | `./models` | Diretório de modelos treinados |

---

## 🔌 Serviços Externos (Opcional)

Estas variáveis são **opcionais** e só necessárias se você quiser integrar com serviços de terceiros:

### DNS & Domínios
- `PORKBUN_API_KEY`, `PORKBUN_SECRET_KEY`
- `CLOUDFLARE_API_KEY`

### Bancos de Dados Externos
- `MONGO_ATLAS_URI`

### Pagamentos
- `PAYPAL_ID`, `PAYPAL_TOKEN_API`
- `STRIPE_API`

### IA Externa
- `OPENAI_API_KEY`, `LANGSMITH_API_KEY`
- `HF_TOKEN` (Hugging Face)
- `OLLAMA_URL`, `OLLAMA_API_KEY`
- `DEEPSEEK_API_KEY`

### Developer Tools
- `GITHUB_USERNAME`, `GITHUB_TOKEN`
- `CARGO_REGISTRY_TOKEN`
- `SENTRY_TOKEN_API`
- `NGROK`

---

## 🚀 Uso

```bash
# Copie o arquivo de exemplo
cp .env.example .env

# Edite com seus valores
nano .env

# As bibliotecas Avila carregam automaticamente via:
use avila_config::load_env;
load_env(); // Lê .env do workspace root
```

## 🔒 Segurança

- ✅ **Nunca** commite `.env` no Git (já está no `.gitignore`)
- ✅ Use `.env.example` como template sem secrets
- ✅ Gere chaves fortes: `openssl rand -hex 32`
- ✅ Rotacione secrets periodicamente em produção
- ✅ Use gestores de secrets (Vault, AWS Secrets Manager)

## 📚 Referências

- [12-Factor App - Config](https://12factor.net/config)
- [OWASP - Secret Management](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html)
- [Avila Docs](https://docs.avila.inc/env-vars)
