# Contribuindo para Avila Analytics

Obrigado pelo seu interesse em contribuir! 🎉

## 📋 Código de Conduta

Este projeto segue o [Código de Conduta do Contributor Covenant](CODE_OF_CONDUCT.md). Ao participar, você concorda em seguir este código.

## 🚀 Como Contribuir

### Reportando Bugs

Antes de criar um issue de bug:
1. Verifique se já existe um issue semelhante
2. Use o template de bug report
3. Inclua informações detalhadas:
   - Versão do Rust
   - Sistema operacional
   - Passos para reproduzir
   - Comportamento esperado vs atual
   - Logs relevantes

### Sugerindo Features

Para sugerir uma nova feature:
1. Verifique o [BLUEPRINT.md](BLUEPRINT.md) e issues existentes
2. Use o template de feature request
3. Descreva claramente:
   - Problema que resolve
   - Solução proposta
   - Alternativas consideradas
   - Impacto esperado

### Pull Requests

#### Processo

1. **Fork** o repositório
2. **Clone** seu fork localmente
3. **Crie** uma branch para sua feature/fix
4. **Faça** suas alterações
5. **Adicione** testes
6. **Execute** os testes e benchmarks
7. **Commit** com mensagens descritivas
8. **Push** para seu fork
9. **Abra** um Pull Request

#### Guidelines de Código

**Estilo**
```bash
# Formate o código
cargo fmt

# Execute o linter
cargo clippy -- -D warnings
```

**Testes**
```bash
# Execute todos os testes
cargo test

# Testes com output
cargo test -- --nocapture

# Coverage
cargo tarpaulin --out Html
```

**Performance**
```bash
# Execute benchmarks
cargo bench

# Compare com baseline
cargo bench --bench event_ingestion -- --save-baseline main
```

#### Convenções de Commit

Usamos [Conventional Commits](https://www.conventionalcommits.org/):

```
<tipo>(<escopo>): <descrição>

[corpo opcional]

[rodapé opcional]
```

**Tipos:**
- `feat`: Nova feature
- `fix`: Correção de bug
- `docs`: Documentação
- `style`: Formatação, ponto e vírgula, etc
- `refactor`: Refatoração de código
- `perf`: Melhoria de performance
- `test`: Adição/correção de testes
- `chore`: Tarefas de build, configuração, etc

**Exemplos:**
```
feat(ml): adiciona modelo de classificação RandomForest

fix(tracker): corrige race condition no event buffer

docs(api): atualiza exemplos de REST endpoints

perf(storage): otimiza queries com índices
```

#### Checklist do PR

- [ ] Código segue as guidelines de estilo (`cargo fmt`, `cargo clippy`)
- [ ] Testes adicionados/atualizados e passando
- [ ] Documentação atualizada (README, docs/, comentários)
- [ ] CHANGELOG.md atualizado
- [ ] Benchmarks executados (se aplicável)
- [ ] Commit messages seguem convenções
- [ ] PR description clara e detalhada

## 🏗️ Estrutura do Projeto

### Módulos Principais

- `src/api/` - REST API e handlers
- `src/storage/` - Backends de storage
- `src/ml/` - Machine Learning
- `src/streaming/` - Stream processing
- `src/industry40/` - Módulos Industry 4.0
- `src/export/` - Exportação de dados
- `src/websocket/` - WebSocket handlers

### Adicionando Novos Módulos

1. Crie estrutura em `src/novo_modulo/`
2. Adicione `mod.rs` com exports públicos
3. Registre no `src/main.rs` ou lib.rs
4. Adicione testes em `tests/unit/` ou `tests/integration/`
5. Adicione benchmark em `benches/`
6. Adicione exemplo em `examples/`
7. Documente em `docs/api/`

## 📝 Documentação

### Comentários de Código

```rust
//! Documentação do módulo
//!
//! Descrição detalhada do propósito e uso do módulo.

/// Documentação de função/struct
///
/// # Exemplos
///
/// ```
/// use avila_analises::*;
/// let result = funcao();
/// ```
///
/// # Erros
///
/// Retorna erro se...
pub fn funcao() -> Result<()> {
    // Implementação
}
```

### Atualizando Docs

```bash
# Gerar documentação
cargo doc --open

# Com features privadas
cargo doc --document-private-items --open
```

## 🧪 Testes

### Estrutura de Testes

```
tests/
├── unit/          # Testes unitários
│   ├── models_tests.rs
│   └── tracker_tests.rs
└── integration/   # Testes de integração
    ├── api_tests.rs
    └── storage_tests.rs
```

### Escrevendo Testes

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funcionalidade() {
        // Arrange
        let input = setup();

        // Act
        let result = funcao(input);

        // Assert
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_async_funcionalidade() {
        // Teste async
    }
}
```

## 🎯 Prioridades Atuais

Veja [BLUEPRINT.md](BLUEPRINT.md) para roadmap completo.

**Fase 1 (Q1 2024) - Consolidação:**
- Suite de testes (80%+ coverage)
- Documentação abrangente
- CI/CD pipeline
- Error handling melhorado

**Áreas com maior necessidade:**
- Testes de integração
- Documentação de API
- Benchmarks de performance
- Exemplos práticos

## 💬 Comunicação

- **Issues**: Para bugs e feature requests
- **Discussions**: Para perguntas e ideias gerais
- **Discord**: [Link do servidor] (se houver)

## 🎓 Recursos

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## ❓ Dúvidas?

Não hesite em perguntar! Abra um issue com a tag `question` ou use Discussions.

---

**Obrigado por contribuir! 🚀**
