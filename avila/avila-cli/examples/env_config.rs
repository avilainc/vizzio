//! Environment and config example
//!
//! Demonstrates value fallback priority:
//! - Command-line arguments (highest priority)
//! - Environment variables
//! - Config file
//! - Default values (lowest priority)
//!
//! Run with:
//!   export MYAPP_PORT=3000
//!   cargo run --example env_config
//!   cargo run --example env_config -- --port 8080

use avila_cli::{App, Arg};

fn main() {
    let matches = App::new("env-config")
        .version("1.0.0")
        .about("Example demonstrating environment and config file support")
        .env_prefix("MYAPP") // Allows MYAPP_PORT, MYAPP_HOST, etc.
        .config_file("app.conf") // Optional config file
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Server port")
                .takes_value(true)
                .default_value("8080")
                .env("PORT") // Also reads from PORT env var
        )
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .help("Server host")
                .takes_value(true)
                .default_value("localhost")
        )
        .arg(
            Arg::new("database")
                .short('d')
                .long("database")
                .help("Database connection string")
                .takes_value(true)
        )
        .parse();

    // Show value and its source
    if let Some(port) = matches.value_of("port") {
        let source = matches.value_source("port").unwrap();
        println!("Port: {} (from: {:?})", port, source);
    }

    if let Some(host) = matches.value_of("host") {
        let source = matches.value_source("host").unwrap();
        println!("Host: {} (from: {:?})", host, source);
    }

    if let Some(db) = matches.value_of("database") {
        let source = matches.value_source("database").unwrap();
        println!("Database: {} (from: {:?})", db, source);
    }

    println!("\n Priority order:");
    println!("  1. Command-line arguments");
    println!("  2. Specific environment variable (PORT)");
    println!("  3. Prefix environment variable (MYAPP_PORT)");
    println!("  4. Config file (app.conf)");
    println!("  5. Default value");
}
