//! Primitive type encoding/decoding example

use avila_buffer::{ByteBuffer, codec::{PrimitiveEncoder, PrimitiveDecoder}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Primitive Codec Example ===\n");

    let mut buffer = ByteBuffer::with_capacity(128);

    // Write various primitive types
    println!("Writing primitives:");
    buffer.write_u8(42)?;
    println!("  u8: 42");

    buffer.write_u16_le(0x1234)?;
    println!("  u16 (LE): 0x1234");

    buffer.write_u32_le(0xDEADBEEF)?;
    println!("  u32 (LE): 0xDEADBEEF");

    buffer.write_u64_be(0x0123456789ABCDEF)?;
    println!("  u64 (BE): 0x0123456789ABCDEF");

    println!("\nBuffer size: {} bytes", buffer.len());
    println!("Buffer content (hex): {}", hex_dump(buffer.as_slice()));

    // Reset to read from beginning
    buffer.reset();

    // Read back the values
    println!("\nReading primitives:");
    let v1 = buffer.read_u8()?;
    println!("  u8: {}", v1);

    let v2 = buffer.read_u16_le()?;
    println!("  u16 (LE): 0x{:04X}", v2);

    let v3 = buffer.read_u32_le()?;
    println!("  u32 (LE): 0x{:08X}", v3);

    let v4 = buffer.read_u64_be()?;
    println!("  u64 (BE): 0x{:016X}", v4);

    println!("\n=== Network Protocol Example ===\n");

    // Simulate a simple network packet
    let mut packet = ByteBuffer::new();

    // Header
    packet.write_u16_be(0x1234)?; // Magic number
    packet.write_u16_be(100)?;     // Packet length
    packet.write_u32_be(42)?;      // Sequence number

    // Payload
    packet.write(b"Hello, Network!")?;

    println!("Packet created:");
    println!("  Total size: {} bytes", packet.len());
    println!("  Raw data: {}", hex_dump(packet.as_slice()));

    // Parse the packet
    packet.reset();

    let magic = packet.read_u16_be()?;
    let length = packet.read_u16_be()?;
    let seq = packet.read_u32_be()?;

    println!("\nParsed packet:");
    println!("  Magic: 0x{:04X}", magic);
    println!("  Length: {}", length);
    println!("  Sequence: {}", seq);

    let mut payload = vec![0u8; 15];
    packet.read(&mut payload)?;
    println!("  Payload: {:?}", std::str::from_utf8(&payload)?);

    Ok(())
}

fn hex_dump(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
