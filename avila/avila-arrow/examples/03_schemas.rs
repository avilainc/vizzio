//! Schema definition and usage

fn main() {
    println!("=== Schema Example ===\n");

    // Define schema fields
    let schema_fields = vec![
        ("id", "Int32", false),
        ("name", "String", false),
        ("email", "String", true),
        ("age", "Int32", true),
    ];

    println!("Schema Definition:");
    for (name, dtype, nullable) in schema_fields {
        let null_str = if nullable { "nullable" } else { "not null" };
        println!("  - {}: {} ({})", name, dtype, null_str);
    }

    println!("\n✓ Successfully defined schema");
}
