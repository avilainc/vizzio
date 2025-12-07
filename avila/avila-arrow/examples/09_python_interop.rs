//! Python interoperability example

fn main() {
    println!("=== Python Interop Example ===\n");

    println!("This example demonstrates how to use avila-arrow from Python:\n");

    println!("```python");
    println!("import avila_arrow");
    println!("");
    println!("# Create an array");
    println!("array = avila_arrow.Array([1, 2, 3, 4, 5])");
    println!("print(f\"Array length: {{array.len()}}\")");
    println!("");
    println!("# Create a RecordBatch");
    println!("batch = avila_arrow.RecordBatch()");
    println!("print(f\"Rows: {{batch.num_rows()}}\")");
    println!("print(f\"Columns: {{batch.num_columns()}}\")");
    println!("```\n");

    println!("To use the Python bindings:");
    println!("1. cd python/");
    println!("2. pip install maturin");
    println!("3. maturin develop");
    println!("4. python -c 'import avila_arrow'");

    println!("\n✓ Python bindings available via PyO3");
}
