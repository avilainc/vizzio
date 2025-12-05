# Deriax Development Guide

## Architecture Overview

Deriax is organized into several core modules:

- **plugin/** - Plugin system for extensibility
- **cache/** - Caching layer for performance
- **analysis/** - Static and dynamic analysis engines
- **emulation/** - Code emulation using Unicorn
- **ml/** - Machine learning for malware detection
- **threat_intel/** - Threat intelligence integration
- **formats/** - Multi-format binary parsers
- **tui/** - Terminal user interface
- **web/** - Web API and dashboard
- **reporting/** - Report generation
- **malware/** - Malware detection
- **vuln/** - Vulnerability scanning
- **ctf/** - CTF tools

## Development Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies (Ubuntu/Debian)
sudo apt-get install libssl-dev pkg-config build-essential

# Install Capstone (disassembly)
sudo apt-get install libcapstone-dev
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code
cargo clippy

# Format code
cargo fmt
```

## Adding New Features

### Creating a New Module

1. Create module directory: `src/mymodule/`
2. Add `mod.rs` with public API
3. Implement functionality in separate files
4. Export in `src/lib.rs` or `src/main.rs`
5. Add tests in `tests/unit/mymodule_tests.rs`

### Creating a Plugin

See `plugins/README.md` for plugin development guide.

## Testing

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

## Documentation

```bash
# Generate docs
cargo doc --open

# Document private items
cargo doc --document-private-items
```

## Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open Pull Request

### Code Style

- Follow Rust conventions
- Run `cargo fmt` before committing
- Fix `cargo clippy` warnings
- Add tests for new features
- Update documentation

## Performance Profiling

```bash
# Profile with flamegraph
cargo flamegraph --bin deriax -- analyze sample.exe

# Benchmark
cargo bench --bench analysis_bench
```

## Debugging

```bash
# Run with debug logs
RUST_LOG=debug cargo run -- analyze sample.exe

# With GDB
rust-gdb target/debug/deriax

# With LLDB
rust-lldb target/debug/deriax
```

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v1.0.0`
4. Push tag: `git push origin v1.0.0`
5. GitHub Actions will build and publish release
