# Contributing to avila-buffer

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and inclusive. We're all here to build something great together.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/avila-buffer.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run formatting: `cargo fmt`
7. Run clippy: `cargo clippy -- -D warnings`
8. Commit your changes: `git commit -m "Add feature: description"`
9. Push to your fork: `git push origin feature/your-feature-name`
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Build with all features
cargo build --all-features

# Build for no_std
cargo build --no-default-features

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Guidelines

### Code Style

- Follow Rust standard style guidelines
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Maximum line length: 100 characters

### Testing

- Write tests for all new functionality
- Maintain test coverage above 90%
- Include both unit tests and integration tests
- Add property-based tests for complex logic

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Your test here
    }
}
```

### Documentation

- Document all public APIs with `///` comments
- Include examples in documentation
- Update README.md for user-facing changes
- Add entries to CHANGELOG.md

```rust
/// Creates a new buffer with the specified capacity.
///
/// # Examples
///
/// ```
/// use avila_buffer::ByteBuffer;
///
/// let buffer = ByteBuffer::with_capacity(1024);
/// assert_eq!(buffer.capacity(), 1024);
/// ```
pub fn with_capacity(capacity: usize) -> Self {
    // Implementation
}
```

### Commit Messages

Follow conventional commits:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Adding or updating tests
- `chore:` Maintenance tasks

Examples:
```
feat: add FixedBuffer implementation
fix: resolve buffer overflow in write operation
docs: update API documentation for RingBuffer
```

## Pull Request Process

1. Update documentation for any API changes
2. Add tests for new functionality
3. Ensure all tests pass
4. Update CHANGELOG.md
5. Request review from maintainers

### PR Checklist

- [ ] Tests pass locally (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Examples are provided (if applicable)

## Areas for Contribution

Looking for where to start? Check these areas:

### High Priority
- [ ] Implement FixedBuffer for stack allocation
- [ ] Add buffer pooling system
- [ ] Improve documentation with more examples
- [ ] Add fuzzing tests
- [ ] Performance benchmarks and optimizations

### Medium Priority
- [ ] Async I/O support with Tokio
- [ ] Compression integration
- [ ] Cryptography features
- [ ] SIMD optimizations

### Good First Issues
- [ ] Add more unit tests
- [ ] Improve error messages
- [ ] Add code examples
- [ ] Fix typos in documentation

## Performance Considerations

When contributing performance-sensitive code:

1. Add benchmarks for new features
2. Compare with baseline performance
3. Document any trade-offs
4. Avoid premature optimization
5. Profile before optimizing

## Questions?

- Open an issue for bug reports or feature requests
- Join discussions in existing issues
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under both MIT and Apache-2.0 licenses.
