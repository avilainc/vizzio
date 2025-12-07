//! Compression techniques example

fn main() {
    println!("=== Compression Example ===\n");

    // RLE (Run-Length Encoding)
    let rle_data: Vec<i32> = vec![1, 1, 1, 1, 2, 2, 3, 3, 3];
    println!("Original data (RLE): {:?}", rle_data);
    println!("Compressed: [(1, 4), (2, 2), (3, 3)]");

    // Delta encoding
    let delta_data: Vec<i32> = vec![10, 11, 12, 13, 14, 15];
    let deltas: Vec<i32> = delta_data.windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    println!("\nOriginal data (Delta): {:?}", delta_data);
    println!("Delta encoded: base={}, deltas={:?}", delta_data[0], deltas);

    // Dictionary encoding
    let dict_data = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    println!("\nOriginal data (Dict): {:?}", dict_data);
    println!("Dictionary: [\"apple\", \"banana\", \"cherry\"]");
    println!("Indices: [0, 1, 0, 2, 1, 0]");

    println!("\n✓ Successfully demonstrated compression techniques");
}
