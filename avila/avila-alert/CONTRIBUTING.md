# Contributing to Avila Alert

Thank you for considering contributing to Avila Alert! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/avila-alert.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test --all-features`
6. Run clippy: `cargo clippy --all-features`
7. Format code: `cargo fmt`
8. Commit your changes: `git commit -am 'Add some feature'`
9. Push to the branch: `git push origin feature/your-feature-name`
10. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Build with default features
cargo build

# Build with all features
cargo build --all-features

# Build specific features
cargo build --features "timestamps,serialization"
```

### Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_alert_creation

# Run with verbose output
cargo test --all-features -- --nocapture
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench bench_alert_creation
```

## Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Follow Rust naming conventions
- Write meaningful commit messages
- Add tests for new features
- Update documentation for public APIs

## Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Include tests for new functionality
- Update CHANGELOG.md
- Update documentation if needed
- Ensure CI passes

## Feature Flags

When adding new features:

1. Consider if it should be optional
2. Add appropriate feature flag in Cargo.toml
3. Use `#[cfg(feature = "feature-name")]` appropriately
4. Update README.md with feature information

## Documentation

- Add doc comments for public APIs
- Include examples in doc comments
- Update README.md for user-facing changes
- Update BLUEPRINT.md for architectural changes

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Avila Alert version
- Minimal reproducible example
- Expected vs actual behavior
- Error messages or logs

## Questions?

Feel free to open an issue for questions or discussions!
