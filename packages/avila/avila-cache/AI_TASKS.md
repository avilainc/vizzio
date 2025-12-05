# 🤖 Tarefas para Desenvolvimento com IA

Este arquivo contém prompts detalhados para usar com GitHub Copilot, ChatGPT, ou outras ferramentas de IA para desenvolver as funcionalidades pendentes.

---

## 🔴 P0 - Alta Prioridade

### 1. Serialização com Serde (`src/serde.rs`)

**Prompt para IA:**
```
Você é um expert em Rust. Preciso implementar serialização completa para um cache.

CONTEXTO:
- Temos DistributedCache<K, V> em src/cache.rs
- Código deve ser no_std compatible
- Feature flag "serde" deve ser opcional

TAREFA:
1. Adicionar feature "serde" no Cargo.toml com dependências:
   - serde = { version = "1.0", default-features = false, optional = true }
   - serde_json = { version = "1.0", default-features = false, optional = true }
   - bincode = { version = "1.3", optional = true }

2. Em src/serde.rs, implementar:
   - #[cfg(feature = "serde")] impl Serialize para DistributedCache
   - #[cfg(feature = "serde")] impl Deserialize para DistributedCache
   - Métodos snapshot() e restore()
   - Suporte para JSON e Bincode

3. Adicionar testes de round-trip

CÓDIGO BASE:
```rust
// src/serde.rs atual (placeholder)
use crate::cache::DistributedCache;

impl<K, V> DistributedCache<K, V> {
    pub fn capacity_hint(&self) -> usize {
        self.data.len()
    }
}
```

IMPLEMENTE A SOLUÇÃO COMPLETA.
```

---

### 2. Sistema de Timestamp Real (`src/ttl.rs`)

**Prompt para IA:**
```
Você é um expert em Rust embedded e no_std.

CONTEXTO:
- src/ttl.rs tem Timestamp mockado que sempre retorna 0
- Precisa funcionar em no_std e std
- Múltiplos time sources

TAREFA:
1. Criar trait TimeSource:
```rust
pub trait TimeSource {
    fn now() -> Timestamp;
}
```

2. Implementar:
   - SystemTimeSource (feature "std")
   - MonotonicCounter (no_std, usa AtomicU64)
   - MockTimeSource (para testes)

3. Refatorar TtlCache para usar TimeSource genérico:
```rust
pub struct TtlCache<K, V, T: TimeSource = DefaultTimeSource>
```

4. Implementar cleanup automático

5. Adicionar testes com MockTimeSource

CÓDIGO ATUAL:
```rust
pub struct Timestamp(pub u64);
impl Timestamp {
    pub fn now() -> Self {
        Self(0) // TODO: implementar
    }
}
```

IMPLEMENTE A SOLUÇÃO COMPLETA COM FEATURES.
```

---

### 3. Thread-Safety Real (`src/concurrent.rs`)

**Prompt para IA:**
```
Você é um expert em Rust concurrency.

CONTEXTO:
- SharedCache atualmente usa RefCell (não thread-safe)
- Precisa de versões para std e no_std
- Usar RwLock para read-heavy workloads

TAREFA:
1. Adicionar features no Cargo.toml:
   - std (para std::sync::RwLock)
   - spin (para spin::RwLock no no_std)

2. Implementar conditional compilation:
```rust
#[cfg(feature = "std")]
use std::sync::RwLock;

#[cfg(all(not(feature = "std"), feature = "spin"))]
use spin::RwLock;
```

3. Refatorar SharedCache:
   - Usar Arc<RwLock<>> em vez de Arc<RefCell<>>
   - Métodos devem usar read()/write()
   - Otimizar para leituras

4. Adicionar testes de concorrência:
   - Multi-thread stress test
   - Deadlock detection test

CÓDIGO ATUAL:
```rust
pub struct SharedCache<K, V> {
    inner: Arc<RefCell<DistributedCache<K, V>>>,
}
```

IMPLEMENTE VERSÃO THREAD-SAFE COM FEATURES.
```

---

## 🟡 P1 - Média Prioridade

### 4. Implementar Metrics Trait (`src/traits.rs`)

**Prompt para IA:**
```
Você é um expert em observability e métricas.

TAREFA:
1. Implementar Metrics trait para CacheStats
2. Criar CustomMetrics com histogramas
3. Adicionar export para Prometheus format
4. Integrar com ManagedCache

REQUISITOS:
- Low overhead
- Configurável
- Múltiplos formats de export
- Labels/tags support

IMPLEMENTE SISTEMA COMPLETO DE MÉTRICAS.
```

---

### 5. Políticas de Eviction Híbridas (`src/eviction.rs`)

**Prompt para IA:**
```
Você é um expert em algoritmos de cache.

TAREFA:
Implementar novas políticas de eviction:

1. TtlLruPolicy - combina TTL + LRU
2. TtlLfuPolicy - combina TTL + LFU
3. AdaptivePolicy - alterna entre LRU/LFU baseado em hit rate
4. SizeBasedPolicy - evita por tamanho de entrada
5. RandomPolicy - baseline para testes
6. ArcPolicy - Adaptive Replacement Cache

CADA POLÍTICA DEVE:
- Implementar trait EvictionPolicy
- Ter testes unitários
- Ter documentação com use cases
- Ser benchmarkável

IMPLEMENTE TODAS AS POLÍTICAS.
```

---

### 6. Melhorias no Sharding (`src/sharding.rs`)

**Prompt para IA:**
```
Você é um expert em sistemas distribuídos.

TAREFA:
Melhorar ShardedCache com:

1. Estatísticas por shard
2. Rebalanceamento dinâmico
3. Consistent hashing
4. Configuração de hash function
5. Iteração paralela
6. Shard affinity

REQUISITOS:
- Backward compatible
- Performance improvements
- Configurável

IMPLEMENTE AS MELHORIAS.
```

---

## 🟢 P2 - Baixa Prioridade

### 7. Cache Warming

**Prompt para IA:**
```
Implemente estratégias de cache warming:
- Load from snapshot
- Lazy loading
- Predictive warming
- Batch preloading

Criar novo módulo src/warming.rs
```

---

### 8. Compression Support

**Prompt para IA:**
```
Implemente suporte a compressão transparente:
- LZ4 compression
- Threshold configurável
- Transparent compress/decompress
- Feature flag opcional

Criar módulo src/compression.rs
```

---

### 9. Persistence Layer

**Prompt para IA:**
```
Implemente camada de persistência:
- Snapshot periódico
- Write-Ahead Log (WAL)
- Recovery automático
- Async I/O support

Criar módulo src/persistence.rs
```

---

### 10. Benchmarks Completos

**Prompt para IA:**
```
Criar suite de benchmarks com criterion.rs:
- Benchmark de cada política de eviction
- Comparação LRU vs LFU vs FIFO vs ARC
- Memory overhead
- Throughput tests
- Latency percentiles

Criar diretório benches/ com benchmarks.
```

---

## 📝 Como Usar

### Com GitHub Copilot Chat:
1. Abra o arquivo alvo (ex: `src/serde.rs`)
2. Selecione o código
3. Abra Copilot Chat
4. Cole o prompt correspondente
5. Revise e aplique as sugestões

### Com ChatGPT/Claude:
1. Copie o prompt completo
2. Cole no chat
3. Copie o código gerado
4. Cole no arquivo correspondente
5. Teste e ajuste

### Com Copilot Inline:
1. Adicione comentários no código:
```rust
// TODO: Implement serde serialization for DistributedCache
// Requirements: no_std compatible, feature flag, round-trip tests
```
2. Deixe Copilot sugerir
3. Use Tab para aceitar

---

## 🎯 Ordem Recomendada

1. ✅ **Serde** - Base para persistence
2. ✅ **Timestamp** - Base para TTL funcional
3. ✅ **Thread-Safety** - Crítico para produção
4. ✅ **Metrics** - Observability
5. ✅ **Eviction Híbrido** - Performance
6. ⏭️ Resto conforme necessidade

---

## 🧪 Checklist Pós-Implementação

Após cada implementação:
- [ ] Código compila sem warnings
- [ ] Testes passam (`cargo test`)
- [ ] Clippy limpo (`cargo clippy`)
- [ ] Formatado (`cargo fmt`)
- [ ] Documentação adicionada
- [ ] Exemplo de uso criado
- [ ] CHANGELOG.md atualizado
- [ ] TODO.md atualizado (marcar como completo)

---

**Dica**: Comece com uma tarefa pequena para entender o padrão, depois escale para as maiores!
