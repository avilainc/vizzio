# Development Workflow

Guia rápido para desenvolvimento diário no avila-cache.

## 🚀 Setup Inicial

```powershell
# Clone e entre no diretório
cd avila-cache

# Carregar helper de tarefas
. .\tasks.ps1

# Verificar ambiente
cargo --version
cargo build
cargo test
```

## 📝 Workflow Diário

### 1. Antes de Começar
```powershell
# Ver próximas tarefas
Show-NextTasks

# Ver TODOs no código
Find-AllTodos

# Ou ver quick wins
cat QUICK_WINS.md
```

### 2. Durante o Desenvolvimento
```powershell
# Rodar testes continuamente
cargo watch -x test

# Ou manualmente
Run-Tests

# Verificar formatação
Check-Format

# Aplicar formatação
cargo fmt
```

### 3. Antes de Commit
```powershell
# Checklist completo
Pre-PR-Check

# Se tudo OK:
git add .
git commit -m "feat: sua mensagem"
```

### 4. Antes de PR
```bash
# Atualizar com main
git fetch origin
git rebase origin/main

# Push
git push origin feature/sua-feature

# Abrir PR no GitHub
```

## 📂 Estrutura de Arquivos

```
avila-cache/
├── src/              # Código fonte
│   ├── lib.rs        # Entry point, API pública
│   ├── cache.rs      # Core cache implementations
│   ├── eviction.rs   # Políticas de eviction
│   └── ...           # Outros módulos
├── tests/            # (Futuro) Integration tests
├── examples/         # (Futuro) Exemplos executáveis
├── benches/          # (Futuro) Benchmarks
├── docs/             # (Futuro) Documentação extra
├── README.md         # Documentação principal
├── TODO.md           # Roadmap completo
├── QUICK_WINS.md     # Tarefas rápidas
├── CHANGELOG.md      # Histórico de mudanças
├── CONTRIBUTING.md   # Guia de contribuição
└── tasks.ps1         # Helper scripts
```

## 🔍 Como Encontrar Tarefas

### Por Prioridade
1. **P0 (Alta)**: Veja seção "Alta Prioridade" no `TODO.md`
2. **P1 (Média)**: Veja seção "Média Prioridade" no `TODO.md`
3. **P2 (Baixa)**: Veja seção "Baixa Prioridade" no `TODO.md`

### Por Tempo Disponível
1. **5-15min**: Veja `QUICK_WINS.md` seção "5-15 minutos"
2. **15-30min**: Veja `QUICK_WINS.md` seção "15-30 minutos"
3. **30-60min**: Veja `QUICK_WINS.md` seção "30-60 minutos"
4. **1-2h**: Veja `QUICK_WINS.md` seção "1-2 horas"

### Por Interesse
- **Documentação**: Veja TODOs em arquivos `.rs` e `README.md`
- **Testes**: Veja `QUICK_WINS.md` seção "Testes"
- **Features**: Veja `TODO.md` seções de funcionalidades
- **Performance**: Veja benchmarks e optimizations no `TODO.md`

## 🧪 Testes

### Rodar Testes
```powershell
# Todos os testes
cargo test

# Teste específico
cargo test test_lru_cache

# Com output
cargo test -- --nocapture

# Ignorar tests lentos
cargo test --release
```

### Escrever Testes
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minha_feature() {
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

### Gerar Docs Localmente
```powershell
cargo doc --open
```

### Escrever Docs
```rust
/// Brief description
///
/// More detailed explanation.
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
pub fn my_function() {}
```

## 🎯 Próximos Passos

Após setup inicial:
1. ✅ Leia `README.md` completamente
2. ✅ Revise `TODO.md` para entender roadmap
3. ✅ Escolha uma tarefa em `QUICK_WINS.md`
4. ✅ Leia `CONTRIBUTING.md` antes do primeiro PR
5. ✅ Configure editor com rust-analyzer

## 🛠️ Ferramentas Úteis

### Recomendadas
- **rust-analyzer**: LSP para IDE
- **cargo-watch**: Auto-recompile on save
- **cargo-expand**: Expand macros
- **cargo-outdated**: Check dependencies

### Instalação
```powershell
cargo install cargo-watch
cargo install cargo-expand
cargo install cargo-outdated
```

## 💡 Dicas

### Performance
- Use `--release` para benchmarks
- Profile com `cargo flamegraph`
- Check com `cargo bloat`

### Debug
- Use `dbg!()` macro para quick debug
- Use `RUST_BACKTRACE=1` para stack traces
- Use `cargo tree` para ver dependências

### Code Quality
- Sempre rode `cargo clippy`
- Use `cargo fmt` antes de commit
- Adicione tests para new features
- Update CHANGELOG.md

## 📞 Ajuda

- **Dúvidas sobre código**: Abra issue com tag `question`
- **Bug encontrado**: Abra issue com reprodução mínima
- **Feature request**: Discuta em issue antes de implementar
- **Problemas no setup**: Verifique versions: `cargo --version`, `rustc --version`

---

Happy coding! 🦀
