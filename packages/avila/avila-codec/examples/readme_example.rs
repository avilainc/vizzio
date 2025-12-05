//! README example verification

use avila_codec::prelude::*;

fn main() {
    // Hex encoding
    let data = b"Hello, World!";
    let hex = hex::encode(data);
    println!("Hex: {}", hex);

    // Base64 encoding
    let b64 = base64::encode(data);
    println!("Base64: {}", b64);

    // Base58 (Bitcoin-style)
    let b58 = base58::encode(data);
    println!("Base58: {}", b58);

    // URL encoding
    let url = url::encode("hello world?test=123");
    println!("URL: {}", url);

    // Multibase (auto-detecting)
    let mb = multibase::encode_base58btc(data);
    println!("Multibase: {}", mb);

    // Checksums
    println!("CRC32: 0x{:08X}", crc::crc32(data));
    println!("XXHash: 0x{:08X}", xxhash::xxhash32(data));

    // LZ4 compression
    let compressed = compression::lz4::compress(data).unwrap();
    let decompressed = compression::lz4::decompress(&compressed).unwrap();
    println!("Compressed: {} -> {} bytes", data.len(), compressed.len());
    assert_eq!(data, &decompressed[..]);
}
