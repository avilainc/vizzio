//! Basic encoding example

use avila_codec::prelude::*;

fn main() {
    println!("=== Avila Codec Examples ===\n");

    // Hex encoding
    let data = b"Hello, World!";
    println!("Original: {:?}", core::str::from_utf8(data).unwrap());

    let hex_encoded = hex::encode(data);
    println!("Hex: {}", hex_encoded);

    // Base64 encoding
    let b64_encoded = base64::encode(data);
    println!("Base64: {}", b64_encoded);

    // Base58 encoding
    let b58_encoded = base58::encode(data);
    println!("Base58: {}", b58_encoded);

    // Base32 encoding
    let b32_encoded = base32::encode(data);
    println!("Base32: {}", b32_encoded);

    // URL encoding
    let url_data = "hello world?test=123";
    let url_encoded = url::encode(url_data);
    println!("\nURL encoding: {} -> {}", url_data, url_encoded);

    // Multibase (auto-detecting)
    println!("\n=== Multibase ===");
    let mb_encoded = multibase::encode_base58btc(data);
    println!("Multibase (Base58): {}", mb_encoded);

    // Checksums
    println!("\n=== Checksums ===");
    println!("CRC32: 0x{:08X}", crc::crc32(data));
    println!("XXHash32: 0x{:08X}", xxhash::xxhash32(data));

    // VarInt encoding
    println!("\n=== VarInt ===");
    let varint_data = varint::encode_varint_u64(12345);
    println!("VarInt(12345): {:?}", varint_data);
}
