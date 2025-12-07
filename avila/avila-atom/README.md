# avila-atom

**Atomic Computational Structures - Fundamental Data Structures from First Principles**

[![Crates.io](https://img.shields.io/crates/v/avila-atom.svg)](https://crates.io/crates/avila-atom)
[![Documentation](https://docs.rs/avila-atom/badge.svg)](https://docs.rs/avila-atom)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/vizzio/avila-atom/workflows/CI/badge.svg)](https://github.com/vizzio/avila-atom/actions)

A high-performance, zero-cost abstraction data structures library for Rust, designed for systems programming with first-class `no_std` support.

## Features

- 🚀 **Zero-cost abstractions**: No runtime overhead vs manual implementations
- 🔒 **Lock-free structures**: Wait-free and lock-free concurrent data structures
- ⚡ **SIMD optimizations**: Vectorized operations on supported platforms
- 🎯 **no_std compatible**: Works in embedded and kernel environments
- 📦 **Modular features**: Pay only for what you use
- 🧪 **Battle-tested**: Comprehensive test suite with property-based testing

## Quick Start

```toml
[dependencies]
avila-atom = "0.1"
```

```rust
use avila_atom::{DynamicArray, AssociativeArray, map};

fn main() {
    // Dynamic array with O(1) amortized push
    let mut vec = DynamicArray::new();
    vec.push(1);
    vec.push(2);

    // HashMap/BTreeMap abstraction
    let mut map = AssociativeArray::new();
    map.insert("key", "value");

    // Convenient initialization macros
    let m = map! {
        "hello" => "world",
        "foo" => "bar",
    };
}
```

## Available Structures

### Core Types
- `Option<T>` - Optional values
- `Result<T, E>` - Error handling
- `DynamicArray<T>` - Growable array (Vec)
- `AssociativeArray<K, V>` - Key-value store (HashMap/BTreeMap)
- `StringBuffer` - UTF-8 string

### Advanced Structures
- `Arena` - Bump allocator for batch allocations
- `ObjectPool<T>` - Reusable object pool
- `FixedArray<T, N>` - Stack-allocated arrays
- `SmallString` - Small string optimization (SSO)

### Lock-Free Structures
- `LockFreeStack<T>` - Wait-free stack
- `AtomicCounter` - Cache-aligned atomic counter
- `RingBuffer<T, N>` - SPSC ring buffer

### Specialized Structures
- `BPlusTree<K, V>` - Cache-efficient B+Tree
- `RobinHoodMap<K, V>` - Robin Hood hash table
- `SparseSet` - Dense/sparse index set
- `BloomFilter` - Probabilistic set membership

## Performance

avila-atom is designed for maximum performance:

- **Cache-friendly**: Data structures optimized for modern CPU caches
- **SIMD support**: Vectorized operations when available (AVX2/AVX-512)
- **Memory efficient**: Careful memory layout and allocation strategies
- **Compile-time optimization**: Monomorphization and aggressive inlining

See [PERFORMANCE.md](PERFORMANCE.md) for detailed benchmarks.

## no_std Support

All structures work in `no_std` environments:

```toml
[dependencies]
avila-atom = { version = "0.1", default-features = false, features = ["alloc"] }
```

## Examples

See the [examples/](examples/) directory for comprehensive usage examples:

- [Basic collections](examples/basic_collections.rs)
- [Arena allocation](examples/arena_usage.rs)
- [Lock-free structures](examples/lockfree_demo.rs)
- [Custom allocators](examples/custom_allocators.rs)

## Documentation

Full API documentation is available at [docs.rs/avila-atom](https://docs.rs/avila-atom).

## Roadmap

See [BLUEPRINT.md](BLUEPRINT.md) for the complete development roadmap.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
