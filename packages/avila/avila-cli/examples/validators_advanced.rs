//! Advanced validators example
//!
//! Demonstrates new advanced validators:
//! - JSON validation
//! - UUID validation
//! - Semver validation
//! - Hex color validation
//! - Pattern matching (wildcard)
//! - Composable validators (AND/OR logic)
//!
//! Run with:
//!   cargo run --example validators_advanced -- --json '{"key":"value"}' --uuid "550e8400-e29b-41d4-a716-446655440000"

use avila_cli::{App, Arg, validation};

fn main() {
    let matches = App::new("validators-advanced")
        .version("1.0.0")
        .about("Example demonstrating advanced validators")
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("JSON string")
                .takes_value(true)
                .validator(validation::validate_json)
        )
        .arg(
            Arg::new("uuid")
                .short('u')
                .long("uuid")
                .help("UUID (8-4-4-4-12 format)")
                .takes_value(true)
                .validator(validation::validate_uuid)
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Semantic version (e.g., 1.2.3)")
                .takes_value(true)
                .validator(validation::validate_semver)
        )
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .help("Hex color (#RGB or #RRGGBB)")
                .takes_value(true)
                .validator(validation::validate_hex_color)
        )
        .arg(
            Arg::new("filename")
                .short('f')
                .long("filename")
                .help("Filename (e.g., main.rs, config.toml)")
                .takes_value(true)
        )
        .arg(
            Arg::new("code")
                .short('k')
                .long("code")
                .help("Alphanumeric code (letters and numbers only)")
                .takes_value(true)
                .validator(validation::validate_alphanumeric)
        )
        .arg(
            Arg::new("tag")
                .short('t')
                .long("tag")
                .help("Git tag (alphanumeric)")
                .takes_value(true)
                .validator(validation::validate_alpha)
        )
        .parse();

    if let Some(json) = matches.value_of("json") {
        println!("✓ Valid JSON: {}", json);
    }

    if let Some(uuid) = matches.value_of("uuid") {
        println!("✓ Valid UUID: {}", uuid);
    }

    if let Some(version) = matches.value_of("version") {
        println!("✓ Valid semantic version: {}", version);
    }

    if let Some(color) = matches.value_of("color") {
        println!("✓ Valid hex color: {}", color);
    }

    if let Some(filename) = matches.value_of("filename") {
        println!("✓ Valid filename: {}", filename);
    }

    if let Some(code) = matches.value_of("code") {
        println!("✓ Valid alphanumeric code: {}", code);
    }

    if let Some(tag) = matches.value_of("tag") {
        println!("✓ Valid tag: {}", tag);
    }

    println!("\n📋 Validator features demonstrated:");
    println!("  • validate_json - Parses JSON strings");
    println!("  • validate_uuid - Standard UUID format");
    println!("  • validate_semver - Semantic versioning (MAJOR.MINOR.PATCH)");
    println!("  • validate_hex_color - CSS hex colors");
    println!("  • validate_regex - Wildcard pattern matching");
    println!("  • validate_all - Composable AND (all must pass)");
    println!("  • validate_any - Composable OR (at least one passes)");
}
