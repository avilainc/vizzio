//! Advanced multibase example with all encoding variants

use avila_codec::prelude::*;
use avila_codec::multibase::Encoding;

fn main() {
    println!("=== Multibase Encoding Examples ===\n");

    let data = b"Hello, IPFS!";
    println!("Original data: {:?}\n", core::str::from_utf8(data).unwrap());

    // Try all multibase encodings
    let encodings = [
        (Encoding::Base16, "Base16 (hex lowercase)"),
        (Encoding::Base16Upper, "Base16 (hex uppercase)"),
        (Encoding::Base32, "Base32 (lowercase)"),
        (Encoding::Base32Upper, "Base32 (uppercase)"),
        (Encoding::Base58Btc, "Base58 (Bitcoin)"),
        (Encoding::Base64, "Base64 (standard)"),
        (Encoding::Base64Url, "Base64 (URL-safe)"),
    ];

    for (encoding, name) in encodings.iter() {
        let encoded = multibase::encode(data, *encoding);
        let decoded = multibase::decode(&encoded).unwrap();

        println!("{:20} | Prefix: '{}' | {}", name, encoding.prefix(), encoded);
        assert_eq!(decoded, data, "Roundtrip failed for {:?}", name);
    }

    println!("\n=== Auto-Detection ===");
    let mb_encoded = multibase::encode_base58btc(data);
    println!("Encoded: {}", mb_encoded);
    println!("Detected prefix: '{}'", mb_encoded.chars().next().unwrap());

    let decoded = multibase::decode(&mb_encoded).unwrap();
    println!("Decoded: {:?}", core::str::from_utf8(&decoded).unwrap());
}
