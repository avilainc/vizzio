//! Checksum calculation examples

use avila_codec::prelude::*;

fn main() {
    println!("=== Checksum Examples ===\n");

    let test_cases = [
        b"" as &[u8],
        b"a",
        b"Hello, World!",
        b"The quick brown fox jumps over the lazy dog",
        b"123456789",
    ];

    for data in test_cases.iter() {
        let text = core::str::from_utf8(data).unwrap_or("<binary>");
        println!("Data: {:?}", text);
        println!("  CRC32:    0x{:08X}", crc::crc32(data));
        println!("  CRC16:    0x{:04X}", crc::crc16(data));
        println!("  XXHash32: 0x{:08X}", xxhash::xxhash32(data));
        println!();
    }

    println!("=== Incremental CRC32 ===");
    let part1 = b"Hello, ";
    let part2 = b"World!";

    // Calculate incrementally
    let crc1 = crc::crc32(part1);
    let crc_final = crc::crc32_with_initial(part2, crc1);

    // Calculate in one go
    let mut full = Vec::new();
    full.extend_from_slice(part1);
    full.extend_from_slice(part2);
    let crc_full = crc::crc32(&full);

    println!("Part 1 CRC: 0x{:08X}", crc1);
    println!("Incremental: 0x{:08X}", crc_final);
    println!("Full: 0x{:08X}", crc_full);
    assert_eq!(crc_final, crc_full, "Incremental CRC mismatch!");

    println!("\n=== XXHash with Seeds ===");
    let data = b"test data";
    for seed in [0, 42, 12345, 0xDEADBEEF] {
        let hash = xxhash::xxhash32_with_seed(data, seed);
        println!("Seed 0x{:08X} -> Hash 0x{:08X}", seed, hash);
    }
}
