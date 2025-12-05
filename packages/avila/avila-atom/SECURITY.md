# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in avila-atom, please report it by:

1. **DO NOT** open a public issue
2. Email the maintainers at: security@vizzio.dev
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will respond within 48 hours and work with you to address the issue.

## Security Considerations

### Unsafe Code

This library contains `unsafe` blocks for performance-critical operations. All unsafe code:

- Is documented with safety invariants
- Is tested thoroughly
- Is reviewed by multiple maintainers
- Has justification for its use

### Memory Safety

- All operations are memory-safe in safe Rust
- Unsafe operations are audited regularly
- MIRI tests run in CI to catch undefined behavior

### Concurrency

- Lock-free structures are tested for data races
- All atomic operations use appropriate memory ordering
- Thread-safety guarantees are documented

### Dependencies

- Minimal dependency tree
- All dependencies are audited with `cargo-audit`
- Dependencies are pinned in Cargo.lock

## Best Practices for Users

1. Always use the latest stable version
2. Report any panics or crashes
3. Review unsafe usage in your code
4. Run tests with MIRI in CI
5. Use AddressSanitizer for debugging

## Audit History

- 2025-12-05: Initial security review
