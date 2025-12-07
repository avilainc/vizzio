# 🗺️ Ávila CLI - Roadmap & TODOs

## 📋 Status Atual

### ✅ Concluído (v1.0.0)

#### Core Functionality
- [x] Estrutura modular completa
- [x] Sistema de parsing de argumentos
- [x] Suporte a subcomandos
- [x] Argumentos com short/long forms
- [x] Validação básica (required, conflicts, requires)
- [x] Sistema de cores ANSI
- [x] Macros declarativos (`cli!`, `arg!`)

#### Features Avançados
- [x] Grupos de argumentos (mutual exclusion)
- [x] Validadores customizados
- [x] Environment variable fallback
- [x] Config file parsing (KEY=VALUE, KEY: VALUE)
- [x] Shell completions (Bash, Zsh, Fish, PowerShell)
- [x] Value source tracking (CLI, ENV, Config, Default)
- [x] Sistema de erros estruturado

#### Testing & Quality
- [x] Testes unitários básicos por módulo
- [x] Zero dependências externas
- [x] Documentação inline (doc comments)

---

## 🚧 Em Desenvolvimento

### TODO: Melhorias Imediatas

#### 1. Testing & Coverage
```rust
// TODO: Expandir cobertura de testes
// Prioridade: ALTA
```
- [ ] Testes de integração end-to-end
- [ ] Testes de completion scripts
- [ ] Testes de validação de grupos complexos
- [ ] Testes de config file parsing com casos edge
- [ ] Property-based testing (QuickCheck style)
- [ ] Benchmarks de performance

#### 2. Error Handling
```rust
// TODO: Melhorar mensagens de erro
// Prioridade: MÉDIA
```
- [ ] Sugestões de correção ("did you mean?")
- [ ] Contexto adicional em erros de validação
- [ ] Stack trace para debug mode
- [ ] Error codes para parsing programático

#### 3. Documentation
```rust
// TODO: Documentação completa
// Prioridade: ALTA
```
- [ ] README.md principal com quickstart
- [ ] Exemplos práticos (pasta `examples/`)
- [ ] Guia de migração de outras CLIs (clap, structopt)
- [ ] API reference completo
- [ ] Tutorial passo a passo

#### 4. Validation Enhancements
```rust
// TODO: Validadores adicionais
// Prioridade: MÉDIA
```
- [ ] `validate_json()` - validar JSON válido
- [ ] `validate_regex()` - validar regex pattern
- [ ] `validate_semver()` - validar semantic version
- [ ] `validate_uuid()` - validar UUIDs
- [ ] `validate_hex_color()` - validar cores hex
- [ ] Validadores compostos (AND/OR logic)

---

## 🎯 Roadmap v1.1.0

### Features Planejados

#### 1. Derive Macros
```rust
// TODO: Implementar derive macros
// Prioridade: ALTA
```
**Objetivo:** Gerar CLI a partir de structs

```rust
#[derive(Cli)]
#[cli(name = "myapp", version = "1.0.0")]
struct Args {
    /// Enable verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Configuration file
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    /// Port number
    #[arg(short, long, validator = "validate_port")]
    port: u16,
}
```

**Tarefas:**
- [ ] Criar crate `avila-cli-derive`
- [ ] Implementar `#[derive(Cli)]`
- [ ] Atributos `#[arg(...)]` e `#[cli(...)]`
- [ ] Conversão automática de tipos
- [ ] Documentação derivada de doc comments

#### 2. Subcommand Improvements
```rust
// TODO: Melhorar sistema de subcomandos
// Prioridade: MÉDIA
```
- [ ] Subcomandos aninhados (multi-level)
- [ ] Aliases para comandos
- [ ] Subcomandos externos (plugin system)
- [ ] Argumentos globais compartilhados
- [ ] Help contextual por comando

#### 3. Interactive Mode
```rust
// TODO: Modo interativo
// Prioridade: BAIXA
```
- [ ] Prompts interativos para args faltando
- [ ] Autocomplete interativo
- [ ] Confirmação para ações destrutivas
- [ ] Menu de seleção para `possible_values`

#### 4. Config File Enhancements
```rust
// TODO: Suporte a mais formatos
// Prioridade: MÉDIA
```
- [ ] TOML completo (via feature flag)
- [ ] JSON support
- [ ] YAML support
- [ ] INI format
- [ ] Merge de múltiplos config files
- [ ] Config file discovery (XDG, home, etc)

---

## 🚀 Roadmap v1.2.0

### Advanced Features

#### 1. Async Support
```rust
// TODO: Suporte async/await
// Prioridade: MÉDIA
```
- [ ] Async validators (I/O bound)
- [ ] Async config loading
- [ ] Non-blocking parsing

#### 2. Internationalization (i18n)
```rust
// TODO: Suporte multi-idioma
// Prioridade: BAIXA
```
- [ ] Mensagens de erro traduzíveis
- [ ] Help text multi-idioma
- [ ] Detecção automática de locale
- [ ] Fallback para inglês

#### 3. Advanced Validation
```rust
// TODO: Validação avançada
// Prioridade: MÉDIA
```
- [ ] Validação de múltiplos argumentos combinados
- [ ] Conditional requirements
- [ ] Custom error messages por validação
- [ ] Validation pipelines

#### 4. Shell Integration
```rust
// TODO: Integração shell avançada
// Prioridade: BAIXA
```
- [ ] Completion dinâmico (valores de API)
- [ ] Man page generation
- [ ] Shell script helpers
- [ ] Environment setup scripts

---

## 🔮 Roadmap v2.0.0 (Futuro)

### Breaking Changes & Major Features

#### 1. No-std Support
```rust
// TODO: Suporte no_std
// Prioridade: BAIXA
```
- [ ] Core sem dependência de std
- [ ] Allocator customizável
- [ ] Embedded systems support

#### 2. Plugin System
```rust
// TODO: Sistema de plugins
// Prioridade: BAIXA
```
- [ ] Dynamic plugin loading
- [ ] Plugin API estável
- [ ] Plugin discovery
- [ ] Sandboxing para plugins

#### 3. Advanced UI
```rust
// TODO: UI components
// Prioridade: BAIXA
```
- [ ] Progress bars
- [ ] Spinners
- [ ] Tables formatadas
- [ ] Syntax highlighting para output

#### 4. Web Integration
```rust
// TODO: CLI to Web
// Prioridade: MUITO BAIXA
```
- [ ] WASM compilation
- [ ] Web-based CLI emulator
- [ ] REST API generation from CLI

---

## 📝 TODOs por Módulo

### `src/app.rs`
- [ ] TODO: Adicionar suporte a aliases para comandos
- [ ] TODO: Implementar help customizado por comando
- [ ] TODO: Melhorar formatação de help (wrap, padding)
- [ ] TODO: Adicionar `--help-all` para mostrar todos os subcomandos

### `src/arg.rs`
- [ ] TODO: Adicionar suporte a `multiple_values(true)`
- [ ] TODO: Implementar `value_delimiter(',')`
- [ ] TODO: Suportar ranges de valores numéricos
- [ ] TODO: Adicionar `hidden_possible_values`

### `src/matches.rs`
- [ ] TODO: Adicionar `values_of()` para múltiplos valores
- [ ] TODO: Implementar `occurrences_of()` para contar flags
- [ ] TODO: Método `to_json()` para serialização
- [ ] TODO: Método `from_json()` para deserialização

### `src/validation/validators.rs`
- [ ] TODO: Adicionar mais validadores (JSON, UUID, etc)
- [ ] TODO: Implementar validador composto `all_of()` / `any_of()`
- [ ] TODO: Cache de resultados de validação
- [ ] TODO: Validação assíncrona

### `src/completion/`
- [ ] TODO: Completion dinâmico com sugestões em tempo real
- [ ] TODO: Suporte a Elvish shell
- [ ] TODO: Suporte a Nushell
- [ ] TODO: Testar completions em diferentes shells

### `src/config/parser.rs`
- [ ] TODO: Suporte a TOML completo
- [ ] TODO: Suporte a JSON
- [ ] TODO: Validação de schema de config
- [ ] TODO: Config file watching (reload on change)

### `src/error.rs`
- [ ] TODO: Error recovery suggestions
- [ ] TODO: Contextual help em erros
- [ ] TODO: Error reporting melhorado (formato JSON)
- [ ] TODO: Integration com `anyhow` / `eyre` (feature flag)

### `src/colors.rs`
- [ ] TODO: Suporte a temas customizáveis
- [ ] TODO: True color (24-bit) support
- [ ] TODO: Detecção melhorada de terminal capabilities
- [ ] TODO: Fallback gracioso para terminais limitados

---

## 🎨 Melhorias de UX

### Help System
- [ ] Colorização inteligente de help text
- [ ] Exemplos inline no help
- [ ] Links para documentação online
- [ ] Screenshots/ASCII art no help

### Error Messages
- [ ] Sugestões "did you mean?" para comandos/args
- [ ] Highlighting de parte problemática
- [ ] Links para troubleshooting
- [ ] Códigos de erro documentados

### Progress Feedback
- [ ] Spinner para operações longas
- [ ] Progress bar com ETA
- [ ] Logs estruturados (JSON mode)
- [ ] Quiet mode para scripting

---

## 🔧 Infraestrutura

### CI/CD
- [ ] GitHub Actions para testes
- [ ] Coverage reporting (codecov)
- [ ] Benchmarks automatizados
- [ ] Release automation
- [ ] Changelog gerado automaticamente

### Tooling
- [ ] Clippy lints customizados
- [ ] Rustfmt config otimizado
- [ ] Pre-commit hooks
- [ ] Dependabot configuration

### Documentation
- [ ] docs.rs configuration
- [ ] Examples testados automaticamente
- [ ] API changelog tracking
- [ ] Migration guides

---

## 📊 Métricas & Objetivos

### Performance
- [ ] Parsing < 1ms para CLIs típicos
- [ ] Memory footprint < 1MB
- [ ] Startup time < 10ms
- [ ] Zero allocations no hot path

### Quality
- [ ] Test coverage > 90%
- [ ] Zero unsafe code (sem #![forbid(unsafe_code)])
- [ ] Clippy warnings = 0
- [ ] Documentation coverage 100%

### Adoption
- [ ] 100+ stars no GitHub
- [ ] 10+ contribuidores
- [ ] Usado em projetos de produção
- [ ] Featured no awesome-rust

---

## 💡 Ideias para Explorar

### Research
- [ ] Integration com `clap` ecosystem (migration path)
- [ ] Benchmarks vs outras libs (clap, structopt, argh)
- [ ] Survey de features mais pedidas
- [ ] Case studies de uso

### Experimental
- [ ] DSL para definir CLIs
- [ ] Visual CLI builder
- [ ] CLI to GUI converter
- [ ] Natural language parsing

---

## 📞 Como Contribuir

### Pegando TODOs
1. Escolha um TODO marcado com prioridade ALTA ou MÉDIA
2. Comente na issue correspondente (ou crie uma)
3. Faça um fork e PR com testes
4. Atualize este ROADMAP quando concluir

### Sugerindo Novos TODOs
1. Abra uma issue com label `enhancement`
2. Descreva o caso de uso
3. Proponha API se aplicável
4. Discuta trade-offs

---

**Última Atualização:** 5 de dezembro de 2025
**Versão Atual:** 1.0.0
**Próxima Release:** 1.1.0 (Q1 2026)
