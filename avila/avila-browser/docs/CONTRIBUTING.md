# Contributing to Avila Browser

Thank you for your interest in contributing to Avila Browser! This document provides guidelines and instructions for contributing.

## 🤝 Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Basic understanding of cryptography and networking (helpful but not required)

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/vizzio/avila-browser.git
cd avila-browser

# Build the project
cargo build

# Run tests
cargo test

# Run with examples
cargo run --example seven_layers
```

## 📝 How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, Rust version)
- Relevant logs or error messages

### Suggesting Enhancements

Feature requests are welcome! Please include:
- Clear description of the feature
- Use case and motivation
- Potential implementation approach (if you have ideas)

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Write clear, documented code
   - Follow Rust conventions
   - Add tests for new functionality
   - Update documentation as needed

4. **Ensure tests pass**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Commit your changes**
   ```bash
   git commit -m "feat: add your feature description"
   ```

   We follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` New feature
   - `fix:` Bug fix
   - `docs:` Documentation changes
   - `test:` Test additions/changes
   - `refactor:` Code refactoring
   - `perf:` Performance improvements
   - `chore:` Maintenance tasks

6. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Open a Pull Request**
   - Provide clear description
   - Reference related issues
   - Wait for review

## 🔒 Security Considerations

This is a security-focused project. Please:
- Never commit secrets, keys, or credentials
- Consider threat models for new features
- Document security implications
- Use constant-time operations where applicable
- Follow cryptographic best practices

### Reporting Security Vulnerabilities

**DO NOT** open public issues for security vulnerabilities. Instead:
- Email security@vizzio.dev
- Include detailed description
- Provide proof of concept (if applicable)
- Wait for confirmation before public disclosure

## 🎨 Code Style

### Rust Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Maximum line length: 100 characters
- Document public APIs with examples

### Documentation

- Use `///` for public item documentation
- Use `//!` for module-level documentation
- Include examples in documentation
- Explain complex algorithms
- Document safety requirements for `unsafe` code

### Testing

- Unit tests in the same file as implementation
- Integration tests in `tests/` directory
- Security tests in `tests/security/`
- Benchmark tests in `benches/`
- Aim for >80% code coverage

Example:
```rust
/// Encrypts data using AES-256-GCM
///
/// # Examples
///
/// ```
/// use avila_browser::crypto::Aes256Gcm;
///
/// let cipher = Aes256Gcm::new([0u8; 32]);
/// let ciphertext = cipher.encrypt(b"secret", &[]).unwrap();
/// ```
pub fn encrypt(&self, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, Error> {
    // Implementation
}
```

## 🏗️ Project Structure

```
avila-browser/
├── src/
│   ├── core/          # Core browser functionality
│   ├── crypto/        # Cryptographic primitives
│   ├── layers/        # Security layers (Tor, VPN, etc.)
│   ├── network/       # Networking low-level
│   ├── privacy/       # Privacy features
│   ├── protocols/     # Network protocols
│   ├── rendering/     # HTML/CSS/JS rendering
│   ├── storage/       # Data persistence
│   ├── api/           # Public APIs
│   └── cli/           # Command-line tools
├── tests/             # Integration tests
├── benches/           # Benchmarks
├── examples/          # Example programs
└── docs/              # Documentation
```

## 🧪 Testing Requirements

All contributions must include tests:

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests
```rust
// tests/integration/feature_test.rs
use avila_browser::*;

#[test]
fn test_integration() {
    // Test implementation
}
```

### Security Tests
```rust
// tests/security/timing_test.rs
#[test]
fn test_constant_time_operation() {
    // Verify no timing leaks
}
```

## 📚 Documentation Requirements

- All public APIs must be documented
- Include code examples
- Explain parameters and return values
- Document error conditions
- Add usage examples

## 🎯 Areas Needing Help

We especially welcome contributions in:
- [ ] Cryptographic implementations
- [ ] Network protocol support
- [ ] Privacy features
- [ ] Performance optimizations
- [ ] Documentation
- [ ] Testing
- [ ] Language bindings (Python, JS, Go)

## 💬 Communication

- **GitHub Issues**: Bug reports, feature requests
- **Pull Requests**: Code contributions
- **Discussions**: Questions, ideas, general chat
- **Discord**: Real-time community chat (coming soon)

## 📄 License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT or Apache 2.0).

## 🙏 Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Given credit in documentation

Thank you for helping make Avila Browser better! 🚀
