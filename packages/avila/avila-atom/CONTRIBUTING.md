# Contributing to avila-atom

Thank you for your interest in contributing! We welcome contributions of all kinds.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/avila-atom`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run tests: `cargo test --all-features`
6. Run benchmarks: `cargo bench`
7. Format code: `cargo fmt`
8. Check lints: `cargo clippy --all-features`
9. Commit and push
10. Open a Pull Request

## Development Guidelines

### Code Style

- Follow Rust API guidelines: https://rust-lang.github.io/api-guidelines/
- Use `cargo fmt` for formatting
- Fix all `cargo clippy` warnings
- Maintain consistent naming conventions

### Testing

- Add tests for all new features
- Maintain >90% code coverage
- Include property-based tests for complex logic
- Test edge cases thoroughly

### Performance

- Add benchmarks for performance-critical code
- Ensure no performance regressions
- Document Big-O complexity
- Profile before optimizing

### Documentation

- Document all public APIs with `///` comments
- Include examples in documentation
- Update README.md for significant changes
- Keep CHANGELOG.md up to date

## Areas for Contribution

### High Priority
- [ ] Complete documentation for existing structures
- [ ] Add more test cases and property tests
- [ ] Implement missing structures from blueprint
- [ ] Performance optimizations

### Medium Priority
- [ ] SIMD implementations
- [ ] Additional examples
- [ ] Embedded platform testing
- [ ] FFI bindings

### Low Priority
- [ ] Visualization tools
- [ ] Educational content
- [ ] Alternative implementations

## Pull Request Process

1. Ensure all tests pass
2. Update documentation
3. Add entry to CHANGELOG.md
4. Request review from maintainers
5. Address review feedback
6. Squash commits if requested

## Code Review

All submissions require review. We review:

- Correctness
- Performance
- Safety (unsafe code justification)
- Documentation
- Test coverage
- API design

## Unsafe Code

Unsafe code requires:

- Clear justification
- Safety invariants documented
- Comprehensive testing
- Review by multiple maintainers

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
