# 🔢 avila-bignum

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![No Dependencies](https://img.shields.io/badge/dependencies-0-brightgreen.svg)](Cargo.toml)

**High-performance arbitrary precision arithmetic library - 100% Pure Rust, Zero Dependencies**

## ✨ Features

- **🎯 Pure Rust**: Zero external dependencies, 100% safe Rust code
- **🚀 Fixed-size types**: U256, U512, U1024, U2048, U4096, I4096
- **🔐 Crypto-ready**: Modular arithmetic, GCD, primality testing
- **📦 No-std compatible**: Works in embedded and WASM environments
- **⚡ Efficient**: Optimized algorithms (Binary GCD, Square-and-multiply)
- **🔒 Type-safe**: Leverages Rust's type system for correctness

## 🚀 Quick Start

```rust
use avila_bignum::U1024;

// Basic arithmetic
let a = U1024::from(42u64);
let b = U1024::from(100u64);
let sum = a + b;  // 142

// Bitwise operations
let x = U1024::from(0b1100u64);
let y = U1024::from(0b1010u64);
let result = x & y;  // 0b1000

// Comparisons
assert!(a < b);
assert_eq!(a + a, U1024::from(84u64));

// Cryptographic operations
use avila_bignum::crypto::modular;
let base = [3u64];
let exp = [4u64];
let modulus = [10u64];
let mut result = [0u64];
modular::mod_pow(&base, &exp, &modulus, &mut result);
// 3^4 mod 10 = 81 mod 10 = 1
```

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
avila-bignum = { path = "../avila-bignum" }
```

**No external dependencies required!** 🎉

For `no_std` environments:

```toml
[dependencies]
avila-bignum = { path = "../avila-bignum", default-features = false }
```

## ✅ What's Implemented (v0.1.0)

### Core Types
- ✅ U256, U512 (base types)
- ✅ **U1024** - Full implementation with all operations
- ✅ **U2048** - RSA-2048 ready
- ✅ **U4096** - RSA-4096 ready with division
- ✅ I4096 (signed, base implementation)

### Arithmetic Operations
- ✅ Addition with carry (`+`, `add_assign`)
- ✅ Subtraction with borrow (`-`, `sub_assign`)
- ✅ Multiplication (`*`) - Schoolbook algorithm
- ✅ Division by u64 (`div_rem_u64` for U4096)
- ✅ Comparisons (`<`, `>`, `==`, `Ord`, `PartialOrd`)

### Bitwise Operations
- ✅ AND, OR, XOR, NOT (`&`, `|`, `^`, `!`)
- ✅ Left/Right shifts (`<<`, `>>`)
- ✅ Bit counting (`leading_zeros`, `trailing_zeros`)

### Cryptographic Primitives
- ✅ **Modular addition** - (a + b) mod m
- ✅ **Modular subtraction** - (a - b) mod m
- ✅ **Modular multiplication** - (a × b) mod m
- ✅ **Modular exponentiation** - a^b mod m (Square-and-multiply)
- ✅ **GCD** - Binary GCD (Stein's algorithm)
- ✅ **Primality testing** - Trial division + Miller-Rabin (basic)
- ✅ **Even/Odd checks**

## 📊 Zero Dependencies

```toml
[dependencies]
# Literally nothing! Pure Rust implementation.
```

This library has **ZERO runtime dependencies**. Everything is implemented from scratch in pure Rust.

## 🎯 Use Cases

- **RSA Cryptography**: Key generation, encryption, signing
- **Elliptic Curve Cryptography**: Field arithmetic for curves
- **Zero-Knowledge Proofs**: Large integer computations
- **Blockchain**: Transaction signing, consensus algorithms
- **Number Theory**: Prime testing, factorization

## 🏗️ Project Structure

```
avila-bignum/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── types/              # Type definitions (U256, U1024, etc.)
│   ├── arithmetic/         # Basic arithmetic operations
│   ├── crypto/             # Cryptographic primitives
│   ├── traits/             # Trait implementations
│   └── utils/              # Utility functions
├── tests/                  # Integration tests
├── benches/                # Performance benchmarks
├── examples/               # Usage examples
└── docs/                   # Additional documentation
```

## 🔧 Development Status

**Current Version**: 0.1.0 (Early Development)

See [ROADMAP.md](docs/ROADMAP.md) for planned features and timeline.

### Implemented
- ✅ Type definitions for U1024, U2048, U4096, I4096
- ✅ Basic addition with carry
- ✅ Constant values (ZERO, ONE, MAX)
- ✅ Type conversions

### In Progress
- 🚧 Subtraction, multiplication, division
- 🚧 Comparison operators
- 🚧 Bitwise operations
- 🚧 Modular arithmetic

### Planned
- 📋 RSA key generation
- 📋 Prime number testing
- 📋 Constant-time operations
- 📋 SIMD optimizations

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run benchmarks
cargo bench

# Run property tests
cargo test --test proptest
```

## 📊 Performance

Benchmarks coming soon. Target: 90%+ performance of established libraries like GMP.

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🔗 Resources

- [Documentation](https://docs.rs/avila-bignum)
- [Issue Tracker](https://github.com/avilaops/avila-bignum/issues)
- [Changelog](CHANGELOG.md)

---

**Note**: This library is under active development. API may change before 1.0 release.
