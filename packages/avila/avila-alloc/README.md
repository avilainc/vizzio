# 🚀 avila-alloc

[![Crates.io](https://img.shields.io/crates/v/avila-alloc.svg)](https://crates.io/crates/avila-alloc)
[![Documentation](https://docs.rs/avila-alloc/badge.svg)](https://docs.rs/avila-alloc)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/avila/avila-alloc/workflows/CI/badge.svg)](https://github.com/avila/avila-alloc/actions)

> **Zero-dependency, stack-first memory allocation library for Rust**

`avila-alloc` provides high-performance, predictable memory allocators designed for systems where heap allocation is expensive, unavailable, or undesirable. Built on the Avila philosophy of zero dependencies and `no_std` first.

## ✨ Features

- 🔒 **`no_std` by default** - works in embedded, kernel, and WASM environments
- 🚀 **Zero dependencies** - no external crates in core functionality
- ⚡ **Stack-first** - minimize heap allocations with stack-based containers
- 🎯 **Predictable** - constant-time operations, no hidden allocations
- 🛡️ **Type-safe** - leverages Rust's type system for memory safety
- 🧪 **Battle-tested** - comprehensive test suite and fuzzing

## 📦 Installation

```toml
[dependencies]
avila-alloc = "0.1"
```

## 🎯 Quick Start

```rust
use avila_alloc::{StackVec, StackString, Arena, Pool};

// Stack-based vector - no heap allocation
let mut vec = StackVec::<i32, 10>::new();
vec.push(42)?;
vec.push(84)?;

// Stack-based string - UTF-8 validated
let mut s = StackString::<32>::new();
s.push_str("Hello, ")?;
s.push_str("World!")?;

// Bump allocator for temporary allocations
#[cfg(feature = "std")]
let arena = Arena::new(4096);

// Object pool for recycling instances
let mut pool = Pool::<MyStruct, 16>::new();
let obj = pool.alloc()?;
```

## 📚 Core Types

### Stack Allocators

| Type | Description | Use Case |
|------|-------------|----------|
| `StackVec<T, N>` | Fixed-capacity vector | Collections with known max size |
| `StackString<N>` | UTF-8 validated string | String building without heap |
| `StackBox<T, N>` | Box on stack | Single value storage |
| `StackQueue<T, N>` | Circular queue | FIFO without allocations |

### Arena Allocators

| Type | Description | Use Case |
|------|-------------|----------|
| `Arena` | Bump allocator (requires `std`) | Temporary allocations |
| `StaticArena<N>` | `no_std` bump allocator | Embedded systems |
| `TypedArena<T>` | Type-specialized arena | Same-type allocations |
| `ScopeArena` | Lifetime-scoped arena | Scope-based deallocation |

### Pool Allocators

| Type | Description | Use Case |
|------|-------------|----------|
| `Pool<T, N>` | Fixed-size object pool | Object recycling |
| `SizedPool<SIZE, N>` | Generic size pool | Variable-sized objects |
| `SlabAllocator<T, N>` | Slab allocation | Kernel-style allocation |

## 🔧 Feature Flags

```toml
[dependencies]
avila-alloc = { version = "0.1", features = ["std", "serde"] }
```

- **`std`** - enables standard library features (Arena, etc.)
- **`serde`** - serialization support for all types
- **`experimental`** - unstable features under development

## 📊 Performance

Benchmarks comparing `avila-alloc` with standard allocators:

```
StackVec::push     10.2 ns  (std::Vec: 45.3 ns)  ⚡ 4.4x faster
StackString::push  8.7 ns   (String: 38.1 ns)    ⚡ 4.4x faster
Arena::alloc       3.1 ns   (Box::new: 52.7 ns)  ⚡ 17x faster
Pool::alloc        2.8 ns   (Box::new: 52.7 ns)  ⚡ 18.8x faster
```

See [benches/](benches/) for detailed benchmarks.

## 🎓 Examples

### Embedded System (no_std)

```rust
#![no_std]
use avila_alloc::{StackVec, StackString};

fn process_sensor_data() {
    let mut readings = StackVec::<f32, 100>::new();
    // Process up to 100 readings without heap allocation
}
```

### Web Assembly

```rust
use avila_alloc::StaticArena;

// WASM environment with limited memory
static ARENA: StaticArena<65536> = StaticArena::new();
```

### Game Engine

```rust
use avila_alloc::{ScopeArena, Pool};

// Per-frame allocations
let frame_arena = ScopeArena::new(1_048_576);

// Reusable entity pool
let mut entities = Pool::<Entity, 1024>::new();
```

More examples in [examples/](examples/) directory.

## 🧪 Testing

```bash
# Run all tests
cargo test --all-features

# Run with Miri (undefined behavior detection)
cargo +nightly miri test

# Run benchmarks
cargo bench

# Generate coverage
cargo tarpaulin --all-features
```

## 📖 Documentation

- [API Documentation](https://docs.rs/avila-alloc)
- [The avila-alloc Book](docs/book/) (coming soon)
- [Migration Guide](docs/MIGRATION.md)
- [Architecture Overview](docs/ARCHITECTURE.md)

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

Part of the **Avila Project** - a collection of zero-dependency Rust libraries.

## 🗺️ Roadmap

See [ROADMAP.md](docs/ROADMAP.md) for the complete development plan.

**Current Status**: Phase 1 - Foundation and Stability

---

Made with ❤️ by the Avila Team
