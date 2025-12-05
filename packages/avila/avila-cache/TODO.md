# TODO - avila-cache Roadmap

## 🎯 Status Atual
- ✅ Estrutura base implementada
- ✅ Políticas de eviction (LRU, LFU, FIFO)
- ✅ Sistema de estatísticas
- ✅ Operações batch
- ✅ TTL support
- ✅ Sharding básico

---

## 📋 Roadmap por Prioridade

### 🔴 Alta Prioridade (P0)

#### TODO: Integração Real de Serde
**Arquivo**: `src/serde.rs`
**Status**: 🟡 Placeholder
**Descrição**: Implementar serialização/deserialização real com serde
```rust
// Precisa:
// - Feature flag para serde opcional
// - Impl Serialize/Deserialize para DistributedCache
// - Snapshot/restore completo
// - Testes de round-trip
```

#### TODO: Timestamp Real no TTL
**Arquivo**: `src/ttl.rs`
**Status**: 🟡 Mock implementado
**Descrição**: Sistema de timestamp funcional (compatível no_std)
```rust
// Atualmente usa Timestamp(u64) mockado
// Precisa: integração com tempo real ou contador monotônico
```

#### TODO: Thread Safety no SharedCache
**Arquivo**: `src/concurrent.rs`
**Status**: 🟡 Usa RefCell (não thread-safe)
**Descrição**: Implementar versão truly concurrent
```rust
// Opções:
// - Feature flag para std::sync::Mutex
// - Usar spin::Mutex para no_std
// - RwLock para read-heavy workloads
```

---

### 🟡 Média Prioridade (P1)

#### TODO: Metrics Trait Implementação
**Arquivo**: `src/traits.rs`
**Status**: 🔴 Trait definido mas não usado
**Descrição**: Conectar Metrics trait com CacheStats
```rust
// Implementar Metrics para CacheStats
// Adicionar custom metrics tracking
```

#### TODO: Eviction Policy - TTL Híbrido
**Arquivo**: `src/eviction.rs`
**Status**: 🔴 Não implementado
**Descrição**: Combinar TTL com LRU/LFU
```rust
// TtlLruPolicy: evita por TTL primeiro, depois LRU
// Útil para cache de sessões
```

#### TODO: ShardedCache Melhorias
**Arquivo**: `src/sharding.rs`
**Status**: 🟡 Básico implementado
**Descrição**: Adicionar funcionalidades
```rust
// - Rebalanceamento de shards
// - Estatísticas por shard
// - Configuração de hash function
// - Suporte a resize
```

#### TODO: Batch Operations - Transações
**Arquivo**: `src/batch.rs`
**Status**: 🟡 Básico implementado
**Descrição**: Operações atômicas em batch
```rust
// - Rollback em caso de erro
// - All-or-nothing semantics
// - Transaction log
```

---

### 🟢 Baixa Prioridade (P2)

#### TODO: Cache Warming
**Arquivo**: `src/warming.rs` (novo)
**Status**: 🔴 Não iniciado
**Descrição**: Pre-populate cache strategies
```rust
// - Load from snapshot
// - Lazy loading
// - Predictive warming
```

#### TODO: Compression Support
**Arquivo**: `src/compression.rs` (novo)
**Status**: 🔴 Não iniciado
**Descrição**: Comprimir valores grandes
```rust
// - Feature flag opcional
// - Transparent compression/decompression
// - Threshold configurável
```

#### TODO: Persistence Layer
**Arquivo**: `src/persistence.rs` (novo)
**Status**: 🔴 Não iniciado
**Descrição**: Salvar/carregar cache em disco
```rust
// - Snapshot periódico
// - WAL (Write-Ahead Log)
// - Recovery automático
```

#### TODO: Cache Patterns
**Arquivo**: `src/patterns.rs` (novo)
**Status**: 🔴 Não iniciado
**Descrição**: Implementar padrões comuns
```rust
// - Cache Aside
// - Read Through
// - Write Through
// - Write Behind
// - Refresh Ahead
```

---

## 🔧 Melhorias Técnicas

### TODO: Benchmarks
**Arquivo**: `benches/` (novo)
**Status**: 🔴 Não iniciado
```rust
// - Criterion.rs benchmarks
// - Comparação entre políticas
// - Memory usage profiling
// - Throughput tests
```

### TODO: Examples Executáveis
**Arquivo**: `examples/` (novo)
**Status**: 🟡 Apenas em tests
```rust
// - Mover de src/examples.rs para examples/
// - Exemplos standalone executáveis
// - cargo run --example lru_cache
```

### TODO: Documentação API
**Status**: 🟡 Básica presente
```rust
// - Rustdoc para todos os públicos
// - Exemplos em cada função
// - Links cruzados entre módulos
// - Performance notes
```

### TODO: Error Handling Melhorado
**Arquivo**: `src/error.rs`
**Status**: 🟡 Básico implementado
```rust
// - Error context (backtrace no std)
// - Error codes
// - Recovery suggestions
// - Custom error types por módulo
```

---

## 🧪 Testes

### TODO: Integration Tests
**Arquivo**: `tests/` (novo)
**Status**: 🔴 Apenas unit tests
```rust
// - Testes end-to-end
// - Multi-threading stress tests
// - Memory leak detection
// - Fuzzing
```

### TODO: Property-Based Testing
**Arquivo**: `tests/property.rs` (novo)
**Status**: 🔴 Não iniciado
```rust
// - QuickCheck/proptest
// - Cache invariants
// - Eviction correctness
```

### TODO: Coverage Report
**Status**: 🔴 Não medido
```bash
# Setup tarpaulin ou llvm-cov
# Target: >80% coverage
```

---

## 📦 Packaging

### TODO: Cargo.toml Features
**Status**: 🟡 Sem features opcionais
```toml
[features]
default = []
std = []
serde = ["dep:serde"]
compression = ["dep:lz4"]
metrics = []
```

### TODO: CI/CD Pipeline
**Arquivo**: `.github/workflows/` (novo)
**Status**: 🔴 Não iniciado
```yaml
# - GitHub Actions
# - Lint (clippy)
# - Format check
# - Test em multiple Rust versions
# - Benchmark regression
```

### TODO: Publicar no crates.io
**Status**: 🔴 Não publicado
```bash
# - Verificar nome disponível
# - Adicionar metadata Cargo.toml
# - License file
# - Changelog
```

---

## 🎓 Educacional

### TODO: Architecture Decision Records
**Arquivo**: `docs/adr/` (novo)
**Status**: 🔴 Não documentado
```markdown
# Decisões importantes:
# - Por que BTreeMap vs HashMap
# - Por que no_std
# - Trade-offs de cada eviction policy
```

### TODO: Performance Guide
**Arquivo**: `docs/PERFORMANCE.md` (novo)
**Status**: 🔴 Não iniciado
```markdown
# Guia de otimização:
# - Escolher política correta
# - Tuning de shards
# - Memory vs speed trade-offs
```

---

## 🔮 Features Avançadas (Futuro)

- [ ] **Distributed Cache Real**: Networking, consensus, replicação
- [ ] **Async Support**: Tokio/async-std integration
- [ ] **WebAssembly**: WASM-friendly builds
- [ ] **Observability**: OpenTelemetry integration
- [ ] **Hot Reload**: Configuração dinâmica
- [ ] **Machine Learning**: Predictive eviction
- [ ] **Multi-tier Cache**: RAM + SSD + Network
- [ ] **CDC (Change Data Capture)**: Stream de mudanças

---

## 📊 Métricas de Sucesso

- [ ] >80% code coverage
- [ ] <100ms p99 latency (inserts/gets)
- [ ] Zero unsafe code
- [ ] Documentação completa
- [ ] Publicado no crates.io
- [ ] >100 stars no GitHub
- [ ] Usado em produção

---

**Última atualização**: 5 de dezembro de 2025
**Versão**: 0.1.0-alpha
**Mantenedores**: Vizzio Team
