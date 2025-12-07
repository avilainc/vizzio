# Avila Alert

[![CI](https://github.com/avila/avila-alert/workflows/CI/badge.svg)](https://github.com/avila/avila-alert/actions)
[![Crates.io](https://img.shields.io/crates/v/avila-alert.svg)](https://crates.io/crates/avila-alert)
[![Documentation](https://docs.rs/avila-alert/badge.svg)](https://docs.rs/avila-alert)
[![License](https://img.shields.io/crates/l/avila-alert.svg)](LICENSE)

A flexible and ergonomic alert system for Rust applications.

## Features

- 🎯 **Multiple Alert Levels**: Trace, Debug, Info, Warning, Error, Critical
- 🎨 **Flexible Formatting**: Simple, detailed, JSON formats
- 🏗️ **Builder Pattern**: Ergonomic API for constructing complex alerts
- 🔌 **Handler System**: Extensible system for custom alert processing
- ⚡ **Optional Features**:
  - Timestamps (via `chrono`)
  - Serialization (via `serde`)
  - Async support (via `tokio`)
  - Tracing integration

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
avila-alert = "0.1"
```

### Basic Usage

```rust
use avila_alert::{Alert, AlertLevel, AlertManager, ConsoleHandler, SimpleFormatter};

fn main() {
    // Create a manager and add a console handler
    let manager = AlertManager::new();
    manager.add_handler(Box::new(ConsoleHandler::new(SimpleFormatter)));

    // Dispatch alerts
    manager.dispatch(Alert::info("Application started"));
    manager.dispatch(Alert::warning("Low memory detected"));
    manager.dispatch(Alert::error("Connection failed"));
}
```

### Using Builder Pattern

```rust
use avila_alert::{AlertBuilder, AlertLevel};

let alert = AlertBuilder::new()
    .level(AlertLevel::Warning)
    .message("Authentication failed")
    .tag("security")
    .tag("auth")
    .context("user", "john.doe")
    .context("ip", "192.168.1.1")
    .build()
    .unwrap();
```

### Using Macros

```rust
use avila_alert::{info, warning, error};

let alert1 = info!("Server listening on port 8080");
let alert2 = warning!("High CPU usage: {}%", 95);
let alert3 = error!("Failed to connect to database");
```

### Custom Handlers

```rust
use avila_alert::{Alert, AlertHandler, CallbackHandler};

let handler = CallbackHandler::new(|alert| {
    // Send to external monitoring service
    println!("Sending alert to monitoring: {:?}", alert);
});

manager.add_handler(Box::new(handler));
```

## Features

Enable optional features in your `Cargo.toml`:

```toml
[dependencies]
avila-alert = { version = "0.1", features = ["full"] }
```

Available features:
- `timestamps` - Add timestamp support via `chrono`
- `serialization` - Enable JSON serialization via `serde`
- `async` - Async alert manager with `tokio`
- `tracing-integration` - Integration with `tracing` crate
- `full` - Enable all features

## Documentation

For detailed documentation, examples, and API reference, visit [docs.rs/avila-alert](https://docs.rs/avila-alert).

## Examples

See the [examples](examples/) directory for more usage examples:

- `basic.rs` - Basic alert usage
- `builder.rs` - Using the builder pattern
- `custom_handler.rs` - Creating custom handlers
- `formatters.rs` - Different formatting options
- `async_example.rs` - Async usage (requires `async` feature)

Run examples with:

```bash
cargo run --example basic
cargo run --example builder --features full
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
