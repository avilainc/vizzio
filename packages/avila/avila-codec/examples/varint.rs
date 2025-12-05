//! Variable-length integer encoding examples

use avila_codec::binary::varint;

fn main() {
    println!("=== VarInt Encoding Examples ===\n");

    println!("=== LEB128 Unsigned ===");
    let test_values_u64 = [0u64, 1, 127, 128, 255, 300, 16384, 2097151, u32::MAX as u64];

    for value in test_values_u64.iter() {
        let encoded = varint::encode_leb128_u64(*value);
        let (decoded, bytes_read) = varint::decode_leb128_u64(&encoded).unwrap();

        println!("Value: {:10} | Bytes: {} | Encoded: {:?}",
                 value, bytes_read, encoded);
        assert_eq!(decoded, *value, "Roundtrip failed!");
    }

    println!("\n=== LEB128 Signed ===");
    let test_values_i64 = [-1i64, 0, 1, -64, 64, -128, 128, -1000, 1000];

    for value in test_values_i64.iter() {
        let encoded = varint::encode_leb128_i64(*value);
        let (decoded, bytes_read) = varint::decode_leb128_i64(&encoded).unwrap();

        println!("Value: {:10} | Bytes: {} | Encoded: {:?}",
                 value, bytes_read, encoded);
        assert_eq!(decoded, *value, "Roundtrip failed!");
    }

    println!("\n=== ZigZag Encoding ===");
    println!("ZigZag maps signed integers to unsigned for efficient encoding:");

    let zigzag_values = [-3i64, -2, -1, 0, 1, 2, 3];
    for value in zigzag_values.iter() {
        let encoded = varint::encode_zigzag_i64(*value);
        let (decoded, _) = varint::decode_zigzag_i64(&encoded).unwrap();

        println!("  {:3} -> {:?} (decoded: {})", value, encoded, decoded);
        assert_eq!(decoded, *value);
    }

    println!("\n=== Space Savings ===");
    println!("Comparing fixed-width (8 bytes) vs VarInt:");

    for value in [1u64, 100, 10000, 1000000, 100000000].iter() {
        let varint = varint::encode_varint_u64(*value);
        let savings = 8 - varint.len();
        println!("  {:12}: {} bytes (saves {} bytes, {:.1}%)",
                 value, varint.len(), savings, (savings as f64 / 8.0) * 100.0);
    }
}
