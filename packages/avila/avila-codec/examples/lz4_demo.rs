//! LZ4 compression demonstration
//!
//! Run with: cargo run --example lz4_demo

use avila_codec::compression::lz4;

fn main() {
    println!("=== LZ4 Compression Demo ===\n");

    // Example 1: Simple text
    let text = b"Hello World! Hello World! Hello World!";
    println!("Original: {:?}", core::str::from_utf8(text).unwrap());
    println!("Original size: {} bytes", text.len());

    let compressed = lz4::compress(text).expect("Compression failed");
    println!("Compressed size: {} bytes", compressed.len());
    println!(
        "Compression ratio: {:.2}%",
        (1.0 - compressed.len() as f64 / text.len() as f64) * 100.0
    );

    let decompressed = lz4::decompress(&compressed).expect("Decompression failed");
    println!("Decompressed: {:?}\n", core::str::from_utf8(&decompressed).unwrap());
    assert_eq!(text, &decompressed[..]);

    // Example 2: Repetitive data
    let mut repetitive = Vec::new();
    for _ in 0..50 {
        repetitive.extend_from_slice(b"pattern");
    }
    println!("Repetitive data size: {} bytes", repetitive.len());

    let compressed = lz4::compress(&repetitive).expect("Compression failed");
    println!("Compressed size: {} bytes", compressed.len());
    println!(
        "Compression ratio: {:.2}%\n",
        (1.0 - compressed.len() as f64 / repetitive.len() as f64) * 100.0
    );

    let decompressed = lz4::decompress(&compressed).expect("Decompression failed");
    assert_eq!(repetitive, decompressed);

    // Example 3: Lorem ipsum
    let lorem = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
                  tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit \
                  amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut \
                  labore et dolore magna aliqua.";
    println!("Lorem ipsum size: {} bytes", lorem.len());

    let compressed = lz4::compress(lorem).expect("Compression failed");
    println!("Compressed size: {} bytes", compressed.len());
    println!(
        "Compression ratio: {:.2}%\n",
        (1.0 - compressed.len() as f64 / lorem.len() as f64) * 100.0
    );

    let decompressed = lz4::decompress(&compressed).expect("Decompression failed");
    assert_eq!(lorem, &decompressed[..]);

    // Example 4: Binary data
    let binary: Vec<u8> = (0..=255).cycle().take(1000).collect();
    println!("Binary data size: {} bytes", binary.len());

    let compressed = lz4::compress(&binary).expect("Compression failed");
    println!("Compressed size: {} bytes", compressed.len());
    println!(
        "Compression ratio: {:.2}%\n",
        (1.0 - compressed.len() as f64 / binary.len() as f64) * 100.0
    );

    let decompressed = lz4::decompress(&compressed).expect("Decompression failed");
    assert_eq!(binary, decompressed);

    println!("All tests passed! ✓");
}
