# avila-cache

Cache distribuído avançado para Rust com suporte a `no_std`.

## 🚀 Funcionalidades

- ✅ **Múltiplas políticas de eviction**
  - LRU (Least Recently Used)
  - LFU (Least Frequently Used)
  - FIFO (First In First Out)
  - Sem eviction

- ✅ **Operações avançadas**
  - Operações batch (múltiplas chaves)
  - TTL (Time-to-Live)
  - Iteradores completos
  - Estatísticas detalhadas

- ✅ **Arquitetura flexível**
  - Sharding para melhor concorrência
  - Cache compartilhado com Arc
  - Builder pattern para fácil configuração
  - Suporte a `no_std`

## 📦 Estrutura de Módulos

```
src/
├── lib.rs           # API pública e documentação
├── cache.rs         # DistributedCache e ManagedCache
├── operations.rs    # Operações estendidas (remove, clear, etc)
├── error.rs         # Sistema de erros tipados
├── config.rs        # CacheConfig com builder
├── eviction.rs      # Políticas de eviction (LRU, LFU, FIFO)
├── iter.rs          # Iteradores (keys, values, iter)
├── stats.rs         # Estatísticas (hit/miss rate)
├── sharding.rs      # ShardedCache para concorrência
├── serde.rs         # Suporte a serialização
├── builder.rs       # CacheBuilder pattern
├── ttl.rs           # Time-to-Live support
├── concurrent.rs    # SharedCache com Arc
├── batch.rs         # Operações batch
└── examples.rs      # Exemplos de uso
```

## 🔧 Uso Básico

### Cache Simples

```rust
use avila_cache::DistributedCache;

let mut cache = DistributedCache::new();
cache.insert("key", "value");
assert_eq!(cache.get(&"key"), Some(&"value"));
```

### Cache com LRU

```rust
use avila_cache::builder::CacheBuilder;

let mut cache = CacheBuilder::new()
    .max_capacity(100)
    .with_lru()
    .unwrap();

cache.insert(1, "one");
cache.insert(2, "two");

// Quando cheio, evita o menos recentemente usado
```

### Cache com LFU

```rust
use avila_cache::builder::CacheBuilder;

let mut cache = CacheBuilder::new()
    .max_capacity(50)
    .with_lfu()
    .unwrap();

// Evita entradas menos frequentemente acessadas
```

### Cache com FIFO

```rust
use avila_cache::builder::CacheBuilder;

let mut cache = CacheBuilder::new()
    .max_capacity(10)
    .with_fifo()
    .unwrap();

// First in, first out
```

## 📊 Estatísticas

```rust
use avila_cache::builder::CacheBuilder;

let mut cache = CacheBuilder::new()
    .max_capacity(100)
    .with_lru()
    .unwrap();

cache.insert(1, "a");
cache.get(&1); // hit
cache.get(&2); // miss

let stats = cache.stats();
println!("Hit rate: {:.2}%", stats.hit_rate() * 100.0);
println!("Hits: {}, Misses: {}", stats.hits, stats.misses);
println!("Evictions: {}", stats.evictions);
```

## ⚡ Operações Batch

```rust
use avila_cache::DistributedCache;

let mut cache = DistributedCache::new();

// Insert batch
cache.insert_batch(vec![
    (1, "a"),
    (2, "b"),
    (3, "c"),
]);

// Get batch
let result = cache.get_batch(&[1, 2, 4]);
println!("Found: {:?}", result.found);
println!("Missing: {:?}", result.missing);

// Remove batch
let removed = cache.remove_batch(&[1, 3]);
```

## 🔀 Sharding

```rust
use avila_cache::ShardedCache;

let mut cache = ShardedCache::new(4); // 4 shards

for i in 0..1000 {
    cache.insert(i, i * 2);
}

println!("Total entries: {}", cache.total_len());
```

## ⏱️ TTL (Time-to-Live)

```rust
use avila_cache::ttl::TtlCache;
use core::time::Duration;

let mut cache = TtlCache::with_default_ttl(Duration::from_secs(300));

cache.insert(1, "expires in 5 min");
cache.insert_with_ttl(2, "custom ttl", Duration::from_secs(60));

// Cleanup expired entries
let expired_count = cache.cleanup_expired();
```

## 🔄 Cache Compartilhado

```rust
use avila_cache::SharedCache;

let cache = SharedCache::new();
cache.insert(1, "value");

// Clone handle para compartilhar
let cache2 = cache.clone_handle();
assert_eq!(cache2.get(&1), Some("value"));
```

## 🎯 Iteradores

```rust
use avila_cache::DistributedCache;

let mut cache = DistributedCache::new();
cache.insert(1, "a");
cache.insert(2, "b");

// Iterar sobre chaves
for key in cache.keys() {
    println!("Key: {}", key);
}

// Iterar sobre valores
for value in cache.values() {
    println!("Value: {}", value);
}

// Iterar sobre pares
for (key, value) in cache.iter() {
    println!("{}: {}", key, value);
}
```

## 🏗️ Configuração Avançada

```rust
use avila_cache::{CacheConfig, DistributedCache};

let config = CacheConfig::new()
    .with_capacity(1000)
    .with_stats(true);

let cache = DistributedCache::<String, Vec<u8>>::with_config(config).unwrap();
```

## 🧪 Testes

Cada módulo possui testes unitários completos:

```bash
cargo test
```

## 📝 Notas de Implementação

- **no_std**: Usa apenas `alloc`, sem dependência de `std`
- **Zero dependencies externas**: Tudo implementado internamente
- **BTreeMap**: Usado como estrutura base (ordenado, determinístico)
- **Performance**: Sharding para melhorar concorrência
- **Flexibilidade**: Trait `EvictionPolicy` permite políticas customizadas

## 🎓 Exemplos Completos

Veja `src/examples.rs` para exemplos detalhados de todos os recursos.

## 🔮 Possíveis Expansões Futuras

- [ ] Persistência em disco
- [ ] Compressão de valores
- [ ] Cache distribuído real (networking)
- [ ] Integração com serde para serialização real
- [ ] Políticas de eviction híbridas
- [ ] Warmup de cache
- [ ] Cache aside / write-through patterns
