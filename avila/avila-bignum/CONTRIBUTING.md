# Contributing to avila-bignum

Thank you for your interest in contributing to avila-bignum! We welcome contributions from the community.

## 🚀 Getting Started

1. **Fork the repository** and clone it locally
2. **Create a branch** for your feature or bugfix
3. **Make your changes** following our coding standards
4. **Write tests** for your changes
5. **Run the test suite** to ensure everything passes
6. **Submit a pull request**

## 📋 Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/avila-bignum.git
cd avila-bignum

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

## 🎯 Contribution Guidelines

### Code Style

- Follow Rust's official style guide
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Use meaningful variable and function names
- Add documentation comments for public APIs

### Testing

- Write unit tests for new functions
- Add integration tests for new features
- Include property-based tests when appropriate
- Ensure test coverage doesn't decrease
- Test edge cases and error conditions

### Documentation

- Document all public APIs with `///` comments
- Include code examples in documentation
- Update README.md if adding new features
- Add entries to CHANGELOG.md

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
feat: add modular multiplication
fix: correct overflow in addition
docs: update README with examples
test: add tests for subtraction
perf: optimize multiplication algorithm
```

## 🐛 Reporting Bugs

When reporting bugs, please include:

- Rust version (`rustc --version`)
- Operating system
- Minimal code example that reproduces the issue
- Expected behavior vs actual behavior
- Stack trace if applicable

## 💡 Feature Requests

Before requesting a feature:

1. Check if it's already on the [roadmap](docs/ROADMAP.md)
2. Search existing issues for similar requests
3. Explain the use case and benefits
4. Provide examples if possible

## 🔒 Security

For security vulnerabilities, please email security@avilaops.com instead of opening a public issue.

## 📜 Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Assume good intentions

## ✅ Pull Request Checklist

Before submitting a PR, ensure:

- [ ] Code builds without errors
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Commit messages follow conventions
- [ ] PR description explains the changes

## 🎓 Learning Resources

If you're new to big integer arithmetic or cryptography:

- [Introduction to Modern Cryptography](https://www.cs.umd.edu/~jkatz/imc.html)
- [Handbook of Applied Cryptography](http://cacr.uwaterloo.ca/hac/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 📞 Getting Help

- Open a [discussion](https://github.com/avilaops/avila-bignum/discussions) for questions
- Join our community chat (coming soon)
- Check the [documentation](https://docs.rs/avila-bignum)

---

Thank you for contributing to avila-bignum! 🎉
