# Contributing to avila-cache

Obrigado por considerar contribuir para o avila-cache! 🎉

## 📋 Checklist Rápido

Antes de submeter PR:
- [ ] Código compila sem warnings
- [ ] Testes passando (`cargo test`)
- [ ] Formatação aplicada (`cargo fmt`)
- [ ] Clippy sem alertas (`cargo clippy`)
- [ ] Documentação atualizada
- [ ] CHANGELOG.md atualizado
- [ ] TODOs marcados se aplicável

## 🚀 Como Começar

### 1. Setup do Ambiente

```bash
# Clone o repositório
git clone <repo-url>
cd avila-cache

# Verificar compilação
cargo build

# Rodar testes
cargo test

# Verificar formatação
cargo fmt --check

# Rodar clippy
cargo clippy -- -D warnings
```

### 2. Estrutura do Código

Leia `TODO.md` para entender o roadmap e prioridades.

**Convenções**:
- Código `no_std` compatível (usar `alloc` quando necessário)
- Testes em cada módulo (`#[cfg(test)] mod tests`)
- Documentação inline para funções públicas
- Exemplos em doc comments quando útil

### 3. Workflow

```bash
# Criar branch feature
git checkout -b feature/minha-feature

# Fazer commits pequenos e focados
git commit -m "feat: adiciona suporte para X"

# Antes do PR
cargo test
cargo fmt
cargo clippy

# Push e abrir PR
git push origin feature/minha-feature
```

## 📝 Estilo de Commit

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` nova funcionalidade
- `fix:` correção de bug
- `docs:` apenas documentação
- `test:` adiciona/modifica testes
- `refactor:` refatoração de código
- `perf:` melhoria de performance
- `chore:` tarefas de manutenção

**Exemplos**:
```
feat: adiciona política de eviction LFU
fix: corrige memory leak no ShardedCache
docs: atualiza README com exemplos de TTL
test: adiciona property tests para eviction
```

## 🎯 Áreas para Contribuir

### Alta Prioridade
1. **Serde Integration** (`src/serde.rs`)
   - Implementar serialização real
   - Adicionar feature flag opcional

2. **Concurrent Cache** (`src/concurrent.rs`)
   - Substituir RefCell por Mutex/RwLock
   - Garantir thread-safety

3. **Benchmarks** (`benches/`)
   - Setup criterion.rs
   - Comparações entre políticas

### Média Prioridade
4. **Cache Patterns** (novo módulo)
   - Cache Aside
   - Write Through
   - Read Through

5. **Melhorias TTL**
   - Timestamp real
   - Cleanup automático

### Fácil para Começar (Good First Issue)
- Adicionar mais exemplos
- Melhorar documentação
- Adicionar testes
- Corrigir TODOs inline no código

## 🧪 Testes

### Rodar Testes
```bash
# Todos os testes
cargo test

# Específico
cargo test --test integration_test

# Com output
cargo test -- --nocapture

# Coverage (se tiver tarpaulin)
cargo tarpaulin --out Html
```

### Escrever Testes
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funcionalidade() {
        // Arrange
        let mut cache = Cache::new();

        // Act
        cache.insert(1, "value");

        // Assert
        assert_eq!(cache.get(&1), Some(&"value"));
    }
}
```

## 📚 Documentação

### Doc Comments
```rust
/// Brief one-line description
///
/// More detailed explanation with examples:
///
/// # Examples
///
/// ```
/// use avila_cache::Cache;
/// let cache = Cache::new();
/// ```
///
/// # Panics
///
/// Panics if capacity is zero.
///
/// # Errors
///
/// Returns `CacheError::CapacityExceeded` if full.
pub fn my_function() {}
```

### Gerar Docs
```bash
cargo doc --open
```

## 🐛 Reportar Bugs

Ao abrir issue, inclua:
- Versão do Rust (`rustc --version`)
- Código mínimo que reproduz o bug
- Comportamento esperado vs atual
- Stack trace se houver panic

## 💡 Sugerir Features

Antes de implementar feature grande:
1. Abra issue descrevendo a proposta
2. Discuta design e trade-offs
3. Aguarde feedback dos mantenedores
4. Implemente após aprovação

## ⚡ Performance

Se contribuir com otimizações:
- Inclua benchmarks provando melhoria
- Documente trade-offs (ex: memória vs velocidade)
- Considere impacto em no_std

## 🔒 Segurança

Se encontrar vulnerabilidade de segurança:
- **NÃO** abra issue pública
- Entre em contato diretamente com mantenedores
- Aguarde fix antes de disclosure

## 📜 Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob a mesma licença do projeto.

## 🤝 Code of Conduct

- Seja respeitoso e inclusivo
- Aceite críticas construtivas
- Foque no que é melhor para a comunidade
- Ajude novos contribuidores

---

**Dúvidas?** Abra uma issue com a tag `question` ou entre em contato com os mantenedores.

Obrigado por contribuir! 🚀
