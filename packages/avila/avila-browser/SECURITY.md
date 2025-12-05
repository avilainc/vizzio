# Security Policy

## Supported Versions

Currently, as this is a pre-1.0 project in active development, we provide security updates for:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@vizzio.dev**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information:

- Type of issue (e.g. buffer overflow, timing attack, etc.)
- Full paths of source file(s) related to the issue
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Security Disclosure Process

1. **Report received**: Security team acknowledges receipt within 48 hours
2. **Initial assessment**: Team evaluates severity and impact (1-3 days)
3. **Fix development**: Patch is developed and tested (varies by severity)
4. **Disclosure**:
   - Critical: Immediate patch release
   - High: Patch within 7 days
   - Medium: Patch within 30 days
   - Low: Scheduled for next release
5. **Public disclosure**: After patch is available, advisory is published
6. **Credit**: Reporter credited in SECURITY.md (if desired)

## Security Considerations

This project deals with security and privacy. Some important notes:

### Cryptographic Implementation
- We use well-audited libraries (ring, RustCrypto, etc.)
- Custom crypto implementations are avoided
- All crypto code should be reviewed by experts

### Constant-Time Operations
- Timing attacks are a serious concern
- Secret comparisons must be constant-time
- Side-channel resistance is required

### Memory Safety
- We leverage Rust's memory safety guarantees
- `unsafe` code requires extra scrutiny and documentation
- Buffer operations must be bounds-checked

### Network Security
- All connections use TLS 1.3 minimum
- Certificate validation is mandatory
- No weak cipher suites

### Privacy Guarantees
- No telemetry or tracking
- No logging of user data
- Minimal metadata collection

## Security Audits

We plan to conduct professional security audits before v1.0 release:

- [x] Cryptographic implementation review - **In Progress**: Using audited crates (aes-gcm, chacha20poly1305, ed25519-dalek, x25519-dalek)
- [ ] Side-channel analysis - **Pending**: Libraries used are designed to be constant-time, formal verification needed
- [ ] Penetration testing - **Not Started**
- [ ] Code audit by security researchers - **Not Started**

### Cryptographic Implementation Status

✅ **Completed:**
- AES-256-GCM authenticated encryption
- ChaCha20-Poly1305 authenticated encryption
- Ed25519 digital signatures
- X25519 key exchange (Diffie-Hellman)
- SHA-256, SHA3-256, BLAKE3 hashing
- 26+ unit tests covering security properties

All implementations use production-grade, audited cryptographic libraries from the Rust ecosystem.

## Known Issues

Currently, this is in early development. Known security limitations:

⚠️ **DO NOT USE IN PRODUCTION**

- ✅ Cryptographic primitives are now implemented using audited libraries (AES-GCM, ChaCha20-Poly1305, Ed25519, X25519, BLAKE3)
- ⚠️ Security layers (Tor, VPN, I2P) are not fully functional yet
- ⚠️ No formal security audits completed
- ⚠️ Side-channel resistance not formally verified (libraries used are constant-time)
- ⚠️ Traffic analysis resistance not tested in real-world scenarios
- ⚠️ Rendering engine not implemented
- ⚠️ Network layer needs real-world testing

## Security Best Practices for Contributors

1. **Never commit secrets**: No API keys, passwords, or private keys
2. **Use constant-time operations**: For cryptographic comparisons
3. **Validate all inputs**: Never trust external data
4. **Fail securely**: Errors should not leak information
5. **Document security implications**: Explain threat models
6. **Test security properties**: Include security tests
7. **Follow secure coding guidelines**: See CONTRIBUTING.md

## Security-Related Dependencies

We regularly audit dependencies for vulnerabilities using:
- `cargo audit` (automated in CI)
- Dependabot security alerts
- Manual review of critical dependencies

## Threat Model

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed threat model.

**In scope:**
- Network-level adversaries (ISP, hosting providers)
- Passive traffic analysis
- Website tracking and fingerprinting
- Cookie tracking
- Malicious exit nodes

**Out of scope:**
- Endpoint compromise (malware on user's machine)
- Physical attacks
- Nation-state quantum computing (until PQC deployed)
- Browser exploitation vulnerabilities
- Operating system vulnerabilities

## Contact

- Security issues: security@vizzio.dev
- General inquiries: contact@vizzio.dev
- PGP key: [To be added]

---

**Last Updated:** December 5, 2025
