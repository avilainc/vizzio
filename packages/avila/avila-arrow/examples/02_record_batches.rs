//! RecordBatch operations

fn main() {
    println!("=== RecordBatch Example ===\n");

    // Simulate a RecordBatch with multiple columns
    let ids: Vec<i32> = vec![1, 2, 3, 4, 5];
    let names: Vec<String> = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
        "David".to_string(),
        "Eve".to_string(),
    ];
    let scores: Vec<f64> = vec![95.5, 87.3, 91.2, 88.8, 93.7];

    println!("RecordBatch with {} rows", ids.len());
    println!("Columns: id (Int32), name (String), score (Float64)\n");

    for i in 0..ids.len() {
        println!("Row {}: id={}, name={}, score={}",
                 i, ids[i], names[i], scores[i]);
    }

    println!("\n✓ Successfully created and displayed RecordBatch");
}
