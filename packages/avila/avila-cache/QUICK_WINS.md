# Quick Wins - Tarefas Rápidas para Começar

Tarefas pequenas e bem definidas para contribuidores iniciantes ou para fazer progresso rápido.

## 🎯 5-15 minutos

### ✅ Documentação

- [ ] **Adicionar exemplo de uso em `cache.rs`**
  - Localização: `src/cache.rs` - função `new()`
  - Adicionar doc example mostrando uso básico

- [ ] **Documentar `EvictionPolicy` trait**
  - Localização: `src/eviction.rs`
  - Adicionar exemplo de implementação customizada

- [ ] **Adicionar link cruzado entre módulos**
  - Usar `[`CacheConfig`]` nos docs para criar links

- [ ] **Corrigir typos no README**
  - Revisar ortografia e gramática

### ✅ Testes

- [ ] **Adicionar teste de edge case em `operations.rs`**
  - Testar `remove` em cache vazio
  - Testar `clear` múltiplas vezes

- [ ] **Teste de capacidade zero em config**
  - Já existe mas pode adicionar mais casos

- [ ] **Teste de sharding com shard_count=1**
  - Verificar que funciona com apenas 1 shard

### ✅ Code Quality

- [ ] **Adicionar `#[must_use]` em funções que retornam valores importantes**
  - Ex: `get()`, `remove()`, `select_victim()`

- [ ] **Marcar structs como `#[derive(Debug)]` onde falta**

- [ ] **Adicionar `#[inline]` em funções hot path**
  - `get()`, `insert()`, `contains_key()`

## 🎯 15-30 minutos

### ✅ Features Pequenas

- [ ] **Adicionar `get_or_insert` em DistributedCache**
  ```rust
  pub fn get_or_insert(&mut self, key: K, default: V) -> &V {
      self.data.entry(key).or_insert(default)
  }
  ```

- [ ] **Adicionar `retain()` method**
  ```rust
  pub fn retain<F>(&mut self, f: F)
  where F: FnMut(&K, &mut V) -> bool
  ```

- [ ] **Implementar `From<Vec<(K,V)>>` para DistributedCache**
  ```rust
  impl<K: Ord, V> From<Vec<(K, V)>> for DistributedCache<K, V>
  ```

- [ ] **Adicionar `keys_vec()` e `values_vec()` helpers**
  ```rust
  pub fn keys_vec(&self) -> Vec<K> where K: Clone
  pub fn values_vec(&self) -> Vec<V> where V: Clone
  ```

### ✅ Melhorias de API

- [ ] **Adicionar `peek()` que não atualiza LRU**
  - Em `ManagedCache`, get() sem chamar `on_access()`

- [ ] **Adicionar `capacity()` getter em ManagedCache**
  ```rust
  pub fn capacity(&self) -> usize {
      self.max_capacity
  }
  ```

- [ ] **Adicionar `is_full()` helper**
  ```rust
  pub fn is_full(&self) -> bool {
      self.len() >= self.max_capacity
  }
  ```

## 🎯 30-60 minutos

### ✅ Exemplos Executáveis

- [ ] **Criar `examples/basic_usage.rs`**
  ```rust
  // cargo run --example basic_usage
  // Mostrar insert, get, remove
  ```

- [ ] **Criar `examples/lru_cache.rs`**
  ```rust
  // Demonstrar LRU em ação com prints
  ```

- [ ] **Criar `examples/statistics.rs`**
  ```rust
  // Mostrar coleta de métricas
  ```

### ✅ Testes de Integração

- [ ] **Criar `tests/integration_basic.rs`**
  ```rust
  // Teste end-to-end de cache simples
  ```

- [ ] **Criar `tests/integration_eviction.rs`**
  ```rust
  // Testar todas as políticas de eviction
  ```

### ✅ Utilities

- [ ] **Adicionar método `debug_info()` em CacheStats**
  ```rust
  pub fn debug_info(&self) -> String {
      format!(
          "Hits: {}, Misses: {}, Hit Rate: {:.2}%",
          self.hits, self.misses, self.hit_rate() * 100.0
      )
  }
  ```

- [ ] **Adicionar `reset_stats()` em ManagedCache**
  ```rust
  pub fn reset_stats(&mut self) {
      self.stats.reset();
  }
  ```

## 🎯 1-2 horas

### ✅ Features Médias

- [ ] **Implementar `Entry` API similar ao HashMap**
  ```rust
  pub enum Entry<'a, K, V> {
      Occupied(OccupiedEntry<'a, K, V>),
      Vacant(VacantEntry<'a, K, V>),
  }
  ```

- [ ] **Adicionar `get_or_insert_with` para lazy init**
  ```rust
  pub fn get_or_insert_with<F>(&mut self, key: K, f: F) -> &V
  where F: FnOnce() -> V
  ```

- [ ] **Cache Statistics Avançado**
  - Adicionar p50, p95, p99 latencies
  - Adicionar size distribution

- [ ] **Melhorar BatchResult**
  ```rust
  impl<K, V> BatchResult<K, V> {
      pub fn success_rate(&self) -> f64 { /* ... */ }
      pub fn to_map(self) -> BTreeMap<K, V> { /* ... */ }
  }
  ```

## 📝 Como Usar Este Arquivo

1. **Escolha uma tarefa** que se alinha com seu interesse/experiência
2. **Marque com `[x]` quando começar** (para evitar duplicação)
3. **Faça commit** da tarefa completa
4. **Abra PR** referenciando esta lista
5. **Atualize CHANGELOG.md**

## 🏆 Progresso

- [ ] Nível 1: 5 quick wins completados
- [ ] Nível 2: 10 quick wins completados
- [ ] Nível 3: 20 quick wins completados
- [ ] 🎖️ Mestre: Todas as quick wins completadas

---

**Dica**: Comece com tarefas de 5-15min para se familiarizar com o codebase!
