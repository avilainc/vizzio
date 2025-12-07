//! Macros example
//!
//! Demonstrates using declarative macros for rapid CLI definition:
//! - cli! macro for quick app setup
//! - arg! macro for concise argument definition
//!
//! Run with:
//!   cargo run --example macros -- --config app.toml -v --output result.json

use avila_cli::{cli, arg};

fn main() {
    let matches = cli!("macros-demo" => {
        version: "1.0.0",
        about: "Declarative CLI using macros",
        args: [
            arg!("config",
                short: 'c',
                help: "Configuration file",
                takes_value: true,
                default: "config.toml"
            ),
            arg!("verbose", short: 'v'),
            arg!("output",
                short: 'o',
                help: "Output file",
                takes_value: true,
                required: true
            ),
            arg!("format",
                short: 'f',
                help: "Output format",
                takes_value: true,
                default: "json"
            )
        ]
    }).parse();

    let config = matches.value_of("config").unwrap();
    let output = matches.value_of("output").unwrap();
    let format = matches.value_of("format").unwrap();

    println!("Configuration file: {}", config);
    println!("Output file: {}", output);
    println!("Output format: {}", format);

    if matches.is_present("verbose") {
        println!("\n[VERBOSE] Processing with full logging");
        println!("[VERBOSE] Reading config from: {}", config);
        println!("[VERBOSE] Will write to: {}", output);
    }

    println!("\n✓ Processing complete!");
}
