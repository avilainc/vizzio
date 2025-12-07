# avila-buffer

[![Crates.io](https://img.shields.io/crates/v/avila-buffer.svg)](https://crates.io/crates/avila-buffer)
[![Documentation](https://docs.rs/avila-buffer/badge.svg)](https://docs.rs/avila-buffer)
[![License](https://img.shields.io/crates/l/avila-buffer.svg)](https://github.com/vizzio/avila-buffer)
[![Build Status](https://github.com/vizzio/avila-buffer/workflows/CI/badge.svg)](https://github.com/vizzio/avila-buffer/actions)

High-performance buffer management library with zero-copy operations for Rust.

## Features

- 🚀 **Zero-copy operations** - Efficient memory management
- 🔧 **Multiple buffer types** - ByteBuffer, RingBuffer, and more
- 🎯 **no_std support** - Works in embedded environments
- ⚡ **High performance** - Optimized for speed and memory efficiency
- 🔒 **Thread-safe options** - Concurrent buffer access
- 📦 **Modular features** - Enable only what you need

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
avila-buffer = "0.1"
```

### Basic Usage

```rust
use avila_buffer::ByteBuffer;

// Create a buffer
let mut buffer = ByteBuffer::with_capacity(1024);

// Write data
buffer.write(b"Hello, World!")?;

// Read data
let mut output = vec![0u8; 13];
buffer.read(&mut output)?;

assert_eq!(&output, b"Hello, World!");
```

### Ring Buffer

```rust
use avila_buffer::RingBuffer;

// Create a ring buffer with capacity for 10 items
let mut ring = RingBuffer::new(10);

// Push items
ring.push(42);
ring.push(100);

// Pop items
assert_eq!(ring.pop(), Some(42));
assert_eq!(ring.pop(), Some(100));
```

## Buffer Types

| Type | Description | Use Case |
|------|-------------|----------|
| `ByteBuffer` | Dynamic byte buffer with read/write cursors | General I/O operations |
| `RingBuffer<T>` | Circular buffer with fixed capacity | Event queues, streaming |
| `FixedBuffer<N>` | Stack-allocated buffer (planned) | Embedded systems |
| `PooledBuffer` | Reusable buffer pool (planned) | High-throughput scenarios |
| `SharedBuffer` | Thread-safe buffer (planned) | Concurrent access |

## Feature Flags

Enable optional features in your `Cargo.toml`:

```toml
[dependencies]
avila-buffer = { version = "0.1", features = ["async", "serde"] }
```

Available features:

- `std` (default) - Standard library support
- `async` - Asynchronous I/O with Tokio
- `compression` - Built-in compression support
- `crypto` - Encryption capabilities
- `serde` - Serialization support

## Performance

Benchmarks on a modern x86_64 CPU (lower is better):

| Operation | avila-buffer | std::vec::Vec | Overhead |
|-----------|--------------|---------------|----------|
| Write 1KB | 245 ns | 240 ns | +2% |
| Read 1KB | 180 ns | 175 ns | +3% |
| Ring push/pop | 15 ns | - | - |

Run benchmarks yourself:

```bash
cargo bench
```

## no_std Support

This library works in `no_std` environments:

```toml
[dependencies]
avila-buffer = { version = "0.1", default-features = false }
```

## Examples

Check the [examples](./examples) directory:

- [`basic.rs`](./examples/basic.rs) - Basic buffer operations
- [`ring_buffer.rs`](./examples/ring_buffer.rs) - Ring buffer usage
- [`async_io.rs`](./examples/async_io.rs) - Async I/O (requires `async` feature)

Run an example:

```bash
cargo run --example basic
```

## Documentation

Full API documentation is available at [docs.rs/avila-buffer](https://docs.rs/avila-buffer).

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## Roadmap

- [x] Core ByteBuffer and RingBuffer
- [ ] FixedBuffer for stack allocation
- [ ] Buffer pooling system
- [ ] Async I/O support
- [ ] SIMD optimizations
- [ ] Compression integration
- [ ] v1.0.0 stable release

See our full [Development Blueprint](./docs/BLUEPRINT.md) for details.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

Part of the [Avila](https://github.com/vizzio/avila) ecosystem.
