# Contributing to avila-alloc

Thank you for your interest in contributing to `avila-alloc`! 🎉

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## How to Contribute

### Reporting Bugs

- Use GitHub Issues to report bugs
- Include minimal reproducible examples
- Specify Rust version, OS, and configuration

### Suggesting Features

- Open an issue with `[Feature Request]` prefix
- Explain the use case and benefits
- Consider performance and `no_std` compatibility

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes following our style guide
4. Add tests for new functionality
5. Run all checks: `cargo test && cargo clippy && cargo fmt`
6. Commit with descriptive messages
7. Push and create a PR

## Development Setup

```bash
# Clone repository
git clone https://github.com/avila/avila-alloc.git
cd avila-alloc

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy --all-features -- -D warnings
```

## Style Guide

- Follow Rust naming conventions
- Maximum line length: 100 characters
- Use `rustfmt` for formatting
- Write documentation for all public items
- Include examples in documentation

## Testing Requirements

- Unit tests for all new functionality
- Integration tests for complex features
- Property tests using `proptest` where applicable
- Benchmarks for performance-critical code

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add StackBox allocator
fix: resolve arena alignment issue
docs: improve StackVec examples
test: add property tests for Pool
perf: optimize StackString::push
```

## Review Process

- All PRs require at least one review
- CI must pass (tests, clippy, formatting)
- Coverage should not decrease
- Documentation must be updated

## Questions?

Feel free to open an issue for questions or discussion!
