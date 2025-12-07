# Examples

This directory contains practical examples demonstrating various features of the Ávila CLI library.

## Basic Examples

### 1. `basic.rs` - Getting Started
**Features**: App creation, arguments with short/long forms, value access

```powershell
cargo run --example basic -- --name John --age 30 -v
```

Demonstrates:
- Creating a basic CLI app
- Required and optional arguments
- Boolean flags
- Basic validation

---

### 2. `subcommands.rs` - Command Hierarchy
**Features**: Multiple commands, command-specific arguments

```powershell
cargo run --example subcommands -- install package-name
cargo run --example subcommands -- remove package-name --force
cargo run --example subcommands -- list --all
```

Demonstrates:
- Defining subcommands (install, remove, list)
- Command-specific arguments
- Detecting which command was invoked

---

### 3. `multiple_values.rs` - Multiple Values
**Features**: Multiple occurrences, delimiters, occurrence counting

```powershell
cargo run --example multiple_values -- --file input1.txt --file input2.txt
cargo run --example multiple_values -- --tags rust,cli,parser
cargo run --example multiple_values -- -vvv
```

Demonstrates:
- Multiple values for single argument
- Comma-separated values with delimiters
- Counting flag occurrences (verbosity levels)
- Fixed number of values

---

## Validation Examples

### 4. `validators.rs` - Built-in Validators
**Features**: Port, email, URL, path validation

```powershell
cargo run --example validators -- --port 8080 --email test@example.com --url https://example.com
```

Demonstrates:
- `validate_port` - Port numbers (1-65535)
- `validate_email` - Email addresses
- `validate_url` - Web URLs
- `validate_path_exists` - File path validation
- `validate_hex_color` - Hex color codes
- `validate_semver` - Semantic versioning

---

### 5. `validators_advanced.rs` - Advanced Validators
**Features**: JSON, UUID, semver, patterns, composable validators

```powershell
cargo run --example validators_advanced -- --json '{"key":"value"}' --uuid "550e8400-e29b-41d4-a716-446655440000"
```

Demonstrates:
- `validate_json` - JSON string parsing
- `validate_uuid` - UUID format validation
- `validate_semver` - Semantic version (1.2.3)
- `validate_hex_color` - CSS hex colors
- `validate_regex` - Wildcard pattern matching (*.rs)
- `validate_all` - Composable AND logic
- `validate_any` - Composable OR logic
- `validate_alphanumeric` - Letters and numbers only

---

## Advanced Examples

### 6. `macros.rs` - Declarative Syntax
**Features**: `cli!` and `arg!` macros for rapid development

```powershell
cargo run --example macros -- --config app.toml -v --output result.json
```

Demonstrates:
- `cli!` macro for quick app definition
- `arg!` macro for concise arguments
- Reduced boilerplate code

---

### 7. `env_config.rs` - Configuration Priority
**Features**: Environment variables, config files, defaults

```powershell
$env:MYAPP_PORT = "3000"
cargo run --example env_config
cargo run --example env_config -- --port 8080
```

Demonstrates:
- Value priority: CLI args > env vars > config file > defaults
- Environment variable prefix
- Config file parsing
- Value source tracking

---

### 8. `completion.rs` - Shell Completions
**Features**: Generate completion scripts for multiple shells

```powershell
cargo run --example completion -- bash > completion.bash
cargo run --example completion -- zsh > _completion
cargo run --example completion -- fish > completion.fish
cargo run --example completion -- powershell > completion.ps1
```

Demonstrates:
- Bash completion generation
- Zsh completion generation
- Fish completion generation
- PowerShell completion generation
- Installation instructions for each shell

---

### 9. `error_suggestions.rs` - "Did You Mean?"
**Features**: Smart error suggestions for typos

```powershell
cargo run --example error_suggestions -- --naem John
cargo run --example error_suggestions -- isntall
cargo run --example error_suggestions -- --format josn
```

Demonstrates:
- Levenshtein distance algorithm
- Argument name suggestions
- Command name suggestions
- Possible value suggestions
- Threshold-based similarity matching

---

### 10. `help_demo.rs` - Professional Help Formatting
**Features**: Beautiful, colorized help text

```powershell
cargo run --example help_demo -- --help
cargo run --example help_demo -- install --help
```

Demonstrates:
- Colorized output with ANSI codes
- Proper alignment and spacing
- Section headers (USAGE, OPTIONS, COMMANDS)
- Required/optional indicators
- Default value display
- Possible values listing
- Context-sensitive help for subcommands

---

## Running All Examples

```powershell
# Run all examples in sequence
$examples = @("basic", "subcommands", "multiple_values", "validators", "validators_advanced", "macros", "env_config", "completion", "error_suggestions", "help_demo")
foreach ($ex in $examples) {
    Write-Host "`n=== Running example: $ex ===`n" -ForegroundColor Cyan
    cargo run --example $ex
}
```

---

## Example Features Map

| Feature                   | Basic | Sub | Multi | Valid | VAdv | Macro | Env | Comp | Err | Help |
|---------------------------|-------|-----|-------|-------|------|-------|-----|------|-----|------|
| Simple arguments          | ✓     | ✓   | ✓     | ✓     | ✓    | ✓     | ✓   | ✓    | ✓   | ✓    |
| Boolean flags             | ✓     | ✓   | ✓     |       |      | ✓     |     |      | ✓   | ✓    |
| Subcommands              |       | ✓   |       |       |      |       |     |      | ✓   | ✓    |
| Multiple values          |       |     | ✓     |       |      |       |     |      |     |      |
| Value delimiters         |       |     | ✓     |       |      |       |     |      |     |      |
| Occurrence counting      |       |     | ✓     |       |      |       |     |      |     |      |
| Basic validators         |       |     |       | ✓     |      |       |     |      |     |      |
| Advanced validators      |       |     |       |       | ✓    |       |     |      |     |      |
| Composable validators    |       |     |       |       | ✓    |       |     |      |     |      |
| Declarative macros       |       |     |       |       |      | ✓     |     |      |     |      |
| Environment variables    |       |     |       |       |      |       | ✓   |      |     |      |
| Config files             |       |     |       |       |      |       | ✓   |      |     |      |
| Value source tracking    |       |     |       |       |      |       | ✓   |      |     |      |
| Shell completions        |       |     |       |       |      |       |     | ✓    |     |      |
| Error suggestions        |       |     |       |       |      |       |     |      | ✓   |      |
| Professional help        |       |     |       |       |      |       |     |      |     | ✓    |

---

## Next Steps

After exploring these examples:

1. **Read the documentation**: Check `MODULAR_STRUCTURE.md` for architecture details
2. **Review the roadmap**: See `ROADMAP.md` for planned features
3. **Write tests**: Add integration tests for your CLI
4. **Build your CLI**: Use these patterns in your own project

For questions or contributions, see the main README.
