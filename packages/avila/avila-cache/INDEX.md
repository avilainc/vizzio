# 📋 Índice de Documentação - avila-cache

Guia completo de navegação pela documentação do projeto.

## 🎯 Por Onde Começar?

### Novo no Projeto?
1. 📖 **[README.md](README.md)** - Visão geral e exemplos
2. 🚀 **[WORKFLOW.md](WORKFLOW.md)** - Setup e workflow diário
3. 🎯 **[QUICK_WINS.md](QUICK_WINS.md)** - Tarefas rápidas para começar

### Quer Contribuir?
1. 🤝 **[CONTRIBUTING.md](CONTRIBUTING.md)** - Guia de contribuição
2. 📋 **[TODO.md](TODO.md)** - Roadmap completo
3. 📝 **[CHANGELOG.md](CHANGELOG.md)** - Histórico de mudanças

## 📚 Documentação Completa

### Visão Geral
| Documento | Descrição | Quando Usar |
|-----------|-----------|-------------|
| **[README.md](README.md)** | Visão geral, features, exemplos de uso | Entender o que o projeto faz |
| **[WORKFLOW.md](WORKFLOW.md)** | Setup inicial, workflow diário, estrutura | Configurar ambiente de dev |
| **[INDEX.md](INDEX.md)** | Este arquivo - navegação | Encontrar documentos |

### Desenvolvimento
| Documento | Descrição | Quando Usar |
|-----------|-----------|-------------|
| **[TODO.md](TODO.md)** | Roadmap completo, prioridades, features futuras | Planejar trabalho, ver progresso |
| **[QUICK_WINS.md](QUICK_WINS.md)** | Tarefas pequenas e rápidas (5min-2h) | Começar a contribuir rapidamente |
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | Guidelines, estilo, processo de PR | Antes de abrir PR |
| **[CHANGELOG.md](CHANGELOG.md)** | Histórico de versões e mudanças | Ver o que mudou, escrever release notes |

### Ferramentas
| Arquivo | Descrição | Como Usar |
|---------|-----------|-----------|
| **[tasks.ps1](tasks.ps1)** | Helper scripts PowerShell | `. .\tasks.ps1` e depois `Show-Help` |

## 🗂️ Estrutura de Código

### Módulos Core
```
src/
├── lib.rs          → Entry point, API pública, re-exports
├── cache.rs        → DistributedCache, ManagedCache
├── operations.rs   → Operações estendidas (remove, clear, etc)
├── error.rs        → CacheError, CacheResult
└── config.rs       → CacheConfig, builder pattern
```

### Features Principais
```
src/
├── eviction.rs     → LRU, LFU, FIFO policies + trait
├── stats.rs        → CacheStats, hit/miss tracking
├── iter.rs         → keys(), values(), iter()
└── builder.rs      → CacheBuilder pattern
```

### Features Avançadas
```
src/
├── sharding.rs     → ShardedCache para concorrência
├── ttl.rs          → Time-to-Live support
├── concurrent.rs   → SharedCache com Arc
├── batch.rs        → Operações batch
├── serde.rs        → Serialização (placeholder)
└── traits.rs       → Utility traits
```

### Exemplos e Testes
```
src/
└── examples.rs     → Exemplos de uso (testes)
```

## 📊 TODOs por Prioridade

### 🔴 Alta (P0) - Fazer Primeiro
- [ ] **Serde Integration** → `src/serde.rs` + `TODO.md`
- [ ] **Real Timestamp** → `src/ttl.rs` + `TODO.md`
- [ ] **Thread Safety** → `src/concurrent.rs` + `TODO.md`

### 🟡 Média (P1) - Fazer Depois
- [ ] **Metrics Implementation** → `src/traits.rs` + `TODO.md`
- [ ] **Hybrid Eviction** → `src/eviction.rs` + `TODO.md`
- [ ] **Sharding Improvements** → `src/sharding.rs` + `TODO.md`

### 🟢 Baixa (P2) - Fazer Eventualmente
- [ ] **Cache Warming** → `TODO.md`
- [ ] **Compression** → `TODO.md`
- [ ] **Persistence** → `TODO.md`

## 🎯 TODOs por Tempo Disponível

### ⚡ 5-15 minutos
→ Ver **[QUICK_WINS.md](QUICK_WINS.md)** seção "5-15 minutos"
- Adicionar docs
- Testes simples
- Code quality fixes

### ⏱️ 15-30 minutos
→ Ver **[QUICK_WINS.md](QUICK_WINS.md)** seção "15-30 minutos"
- Features pequenas
- API improvements
- Helper methods

### 🕐 30-60 minutos
→ Ver **[QUICK_WINS.md](QUICK_WINS.md)** seção "30-60 minutos"
- Exemplos executáveis
- Testes de integração
- Utilities

### 🕓 1-2 horas
→ Ver **[QUICK_WINS.md](QUICK_WINS.md)** seção "1-2 horas"
- Entry API
- Advanced stats
- Batch improvements

## 🔍 Buscar TODOs no Código

### Manual
```powershell
# Buscar no código
Get-ChildItem -Path src -Recurse -Filter "*.rs" | Select-String "TODO"
```

### Com Helper
```powershell
# Carregar helper
. .\tasks.ps1

# Ver todos os TODOs
Find-AllTodos

# Contar por arquivo
Count-Todos

# Ver próximas tarefas
Show-NextTasks
```

## 🧪 Testes

### Localização
- **Unit tests**: Dentro de cada arquivo em `src/*.rs`
- **Integration tests**: (Futuro) `tests/`
- **Examples**: `src/examples.rs`

### Como Rodar
```powershell
# Todos os testes
cargo test

# Com helper
. .\tasks.ps1
Run-Tests
```

## 📈 Progresso do Projeto

### Implementado ✅
- ✅ Core cache (DistributedCache)
- ✅ Managed cache com eviction
- ✅ 4 políticas de eviction (LRU, LFU, FIFO, None)
- ✅ Estatísticas completas
- ✅ Sharding básico
- ✅ TTL support
- ✅ Batch operations
- ✅ SharedCache
- ✅ Builder pattern
- ✅ Iteradores
- ✅ Documentação extensiva

### Em Progresso 🟡
- 🟡 Serde integration (placeholder)
- 🟡 Real timestamp (mockado)
- 🟡 Thread-safety (RefCell)

### Planejado 🔴
- 🔴 Benchmarks
- 🔴 CI/CD
- 🔴 Examples executáveis
- 🔴 Integration tests
- 🔴 Persistence
- 🔴 Compression

## 🚀 Quick Commands

```powershell
# Setup inicial
cargo build
cargo test

# Durante desenvolvimento
cargo watch -x test
cargo fmt
cargo clippy

# Antes de PR
. .\tasks.ps1
Pre-PR-Check

# Ver docs
cargo doc --open

# Estatísticas
. .\tasks.ps1
Show-Stats
```

## 📞 Links Úteis

### Documentação Externa
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [no_std Support](https://docs.rust-embedded.org/book/intro/no-std.html)

### Referências de Cache
- [Cache Replacement Policies](https://en.wikipedia.org/wiki/Cache_replacement_policies)
- [LRU Cache](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU)
- [LFU Cache](https://en.wikipedia.org/wiki/Least_frequently_used)

## 🎓 Aprendizado

### Novos Contribuidores
1. Leia [README.md](README.md) completamente
2. Configure ambiente com [WORKFLOW.md](WORKFLOW.md)
3. Escolha uma tarefa em [QUICK_WINS.md](QUICK_WINS.md)
4. Leia [CONTRIBUTING.md](CONTRIBUTING.md) antes do PR

### Entender Arquitetura
1. Comece por `src/lib.rs` - veja os re-exports
2. Leia `src/cache.rs` - core implementations
3. Explore `src/eviction.rs` - políticas
4. Veja `src/examples.rs` - casos de uso

### Fazer Mudanças Significativas
1. Abra issue discutindo a proposta
2. Leia `TODO.md` para ver se já está planejado
3. Discuta design com mantenedores
4. Implemente após aprovação
5. Atualize `CHANGELOG.md`

---

**Última atualização**: 5 de dezembro de 2025

**Dúvidas?** Abra uma issue com tag `question` ou veja [CONTRIBUTING.md](CONTRIBUTING.md)
