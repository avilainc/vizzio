# Ávila CLI - Estrutura Modular

## 📁 Estrutura do Projeto

```
src/
├── lib.rs                    # Ponto de entrada e re-exports públicos
├── app.rs                    # App e Command - definição da aplicação
├── arg.rs                    # Arg, ArgGroup, Validator, ValueSource
├── matches.rs                # Matches - resultados do parsing
├── colors.rs                 # Sistema de cores ANSI
├── macros.rs                 # Macros cli! e arg!
├── error.rs                  # Tipos de erro customizados
├── validation/
│   ├── mod.rs               # Módulo de validação
│   ├── validators.rs        # Validadores predefinidos
│   └── groups.rs            # Validação de grupos de argumentos
├── completion/
│   ├── mod.rs               # Módulo de completions
│   ├── bash.rs              # Completion para Bash
│   ├── zsh.rs               # Completion para Zsh
│   ├── fish.rs              # Completion para Fish
│   └── powershell.rs        # Completion para PowerShell
└── config/
    ├── mod.rs               # Módulo de configuração
    └── parser.rs            # Parser de arquivos de config
```

## 🎯 Responsabilidades dos Módulos

### Core Modules

#### `lib.rs`
- Ponto de entrada da biblioteca
- Re-exports públicos de tipos principais
- Declaração de módulos

#### `app.rs`
- Struct `App` - aplicação principal
- Struct `Command` - subcomandos
- Lógica de parsing de argumentos
- Sistema de help e validação

#### `arg.rs`
- Struct `Arg` - definição de argumentos
- Struct `ArgGroup` - grupos de argumentos
- Type `Validator` - função validadora
- Enum `ValueSource` - origem dos valores

#### `matches.rs`
- Struct `Matches` - resultados do parsing
- Métodos de consulta de valores
- Conversão de tipos
- Parsing interno de argumentos

#### `error.rs`
- Enum `CliError` - tipos de erro
- Mensagens formatadas
- Implementação de `std::error::Error`
- Suporte a mensagens coloridas

### Utility Modules

#### `colors.rs`
- Constantes ANSI para cores
- Função `colorize()` para colorir texto
- Detecção de suporte a cores no terminal

#### `macros.rs`
- Macro `cli!` - definição declarativa de CLI
- Macro `arg!` - definição declarativa de argumentos
- Helpers para construção rápida

### Feature Modules

#### `validation/`
**validators.rs** - Validadores predefinidos:
- `validate_port()` - validação de portas
- `validate_ip()` - validação de IPs
- `validate_url()` - validação de URLs
- `validate_email()` - validação de emails
- `validate_path_exists()` - verificação de paths
- `validate_is_file()` / `validate_is_dir()`
- `validate_range()` - valores numéricos em range
- `validate_min_length()` / `validate_max_length()`

**groups.rs** - Validação de grupos:
- Grupos obrigatórios (ao menos um presente)
- Grupos mutuamente exclusivos
- Validação com mensagens coloridas

#### `completion/`
Geração de scripts de completion para:
- **Bash** - completion tradicional
- **Zsh** - completion com descrições
- **Fish** - completion interativo
- **PowerShell** - completion com tooltips

#### `config/`
**parser.rs** - Parser de configuração:
- Suporte a formato KEY=VALUE
- Suporte a formato KEY: VALUE
- Comentários (# e //)
- Valores entre aspas
- Conversão para HashMap

## 🔧 Uso

### Exemplo Básico

```rust
use avila_cli::{App, Arg, Command};

fn main() {
    let matches = App::new("myapp")
        .version("1.0.0")
        .about("My CLI application")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .help("Config file path")
            .takes_value(true))
        .command(Command::new("run")
            .about("Run the application")
            .arg(Arg::new("port")
                .short('p')
                .takes_value(true)))
        .parse();

    if let Some(config) = matches.value_of("config") {
        println!("Using config: {}", config);
    }
}
```

### Usando Macros

```rust
use avila_cli::{cli, arg};

let app = cli!("myapp" => {
    version: "1.0.0",
    about: "My CLI app",
    args: [
        arg!("verbose", short: 'v'),
        arg!("output", takes_value: true, default: "out.txt")
    ]
});
```

### Validadores Customizados

```rust
use avila_cli::{Arg, validation};

let arg = Arg::new("port")
    .takes_value(true)
    .validator(validation::validate_port);

// Ou validador inline
let arg2 = Arg::new("email")
    .takes_value(true)
    .validator(|v| {
        if v.contains('@') {
            Ok(())
        } else {
            Err("must be valid email".to_string())
        }
    });
```

### Shell Completions

```rust
use avila_cli::{App, Shell};

let app = App::new("myapp")
    .arg(Arg::new("verbose").short('v'));

// Gerar completion para bash
let bash_script = app.generate_completion(Shell::Bash);
println!("{}", bash_script);
```

## ✨ Benefícios da Modularização

### Manutenibilidade
- Código organizado por responsabilidade única
- Fácil localização de funcionalidades
- Testes isolados e focados

### Extensibilidade
- Adicionar novos shells de completion sem modificar código existente
- Criar validadores customizados em módulo separado
- Suportar novos formatos de config facilmente

### Performance
- Compilação incremental mais eficiente
- Possibilidade de features opcionais (futuro)
- Lazy loading de módulos pesados

### Testabilidade
- Testes unitários por módulo
- Mocks e fixtures simplificados
- Cobertura granular de código

## 🚀 Próximos Passos (Sugestões)

1. **Features opcionais** via Cargo features
   - `features = ["completion"]` para incluir apenas completion
   - `features = ["validation"]` para validadores extras

2. **Async support** para validadores I/O-bound

3. **Custom derives** para gerar CLI de structs
   ```rust
   #[derive(Cli)]
   struct Args {
       #[arg(short, long)]
       verbose: bool,
   }
   ```

4. **Plugin system** para extensões customizadas

5. **Internacionalização (i18n)** para mensagens de erro

## 📝 Notas

- Zero dependências externas
- 100% Rust puro
- Compatível com `no_std` (futuro)
- Performance otimizada (O(1) lookups)
