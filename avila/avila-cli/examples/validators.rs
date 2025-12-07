//! Validators example
//!
//! Demonstrates using built-in validators:
//! - Port validation
//! - Email validation
//! - URL validation
//! - File path validation
//! - Custom validators
//!
//! Run with:
//!   cargo run --example validators -- --port 8080 --email test@example.com --url https://example.com

use avila_cli::{App, Arg, validation};

fn main() {
    let matches = App::new("validators")
        .version("1.0.0")
        .about("Example demonstrating input validation")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Server port (1-65535)")
                .takes_value(true)
                .validator(validation::validate_port)
                .default_value("8080")
        )
        .arg(
            Arg::new("email")
                .short('e')
                .long("email")
                .help("Email address")
                .takes_value(true)
                .validator(validation::validate_email)
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("Website URL")
                .takes_value(true)
                .validator(validation::validate_url)
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Input file (must exist)")
                .takes_value(true)
                .validator(validation::validate_path_exists)
        )
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .help("Hex color code")
                .takes_value(true)
                .validator(validation::validate_hex_color)
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Semantic version")
                .takes_value(true)
                .validator(validation::validate_semver)
        )
        .parse();

    if let Some(port) = matches.value_of("port") {
        println!("Server will run on port: {}", port);
    }

    if let Some(email) = matches.value_of("email") {
        println!("Email: {}", email);
    }

    if let Some(url) = matches.value_of("url") {
        println!("URL: {}", url);
    }

    if let Some(file) = matches.value_of("file") {
        println!("Processing file: {}", file);
    }

    if let Some(color) = matches.value_of("color") {
        println!("Color: {}", color);
    }

    if let Some(version) = matches.value_of("version") {
        println!("Version: {}", version);
    }
}
