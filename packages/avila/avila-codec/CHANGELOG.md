# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of core encoding modules
- Hex encoding/decoding with constant-time operations
- Base64 standard encoding (RFC 4648)
- Base58 Bitcoin-style encoding
- Base32 encoding with standard and hex variants
- Base85 encoding (ASCII85 and Z85 variants)
- URL encoding (percent encoding, RFC 3986)
- Multibase self-describing encodings (IPFS-style)
- CRC32/16 checksums with lookup table optimization
- XXHash32 fast non-cryptographic hashing
- Variable-length integer encoding (LEB128, ZigZag)
- Quoted-Printable encoding (RFC 2045)
- Comprehensive test suite (45+ tests)
- Multiple examples (basic, checksums, varint, multibase)
- SIMD infrastructure (placeholders for AVX2/NEON)
- Traits module for extensibility
- no_std support
- Full documentation

## [0.1.0] - 2025-12-05

### Added
- Initial release
