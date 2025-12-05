//! Variable-length integer encoding example

use avila_buffer::{ByteBuffer, codec::{VarintEncoder, VarintDecoder}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Varint Encoding Example ===\n");

    let mut buffer = ByteBuffer::new();

    println!("=== Unsigned Integers (u64) ===\n");

    // Small values use fewer bytes
    let values = [0u64, 1, 127, 128, 255, 16383, 16384, u64::MAX];

    for &value in &values {
        let pos_before = buffer.write_position();
        buffer.write_varint_u64(value)?;
        let pos_after = buffer.write_position();
        let bytes_used = pos_after - pos_before;

        println!("Value: {:20} -> {} bytes", value, bytes_used);
    }

    println!("\nTotal bytes written: {}", buffer.len());
    println!("Fixed encoding would use: {} bytes", values.len() * 8);

    // Read them back
    println!("\n=== Reading Back ===\n");
    buffer.reset();

    for &expected in &values {
        let value = buffer.read_varint_u64()?;
        println!("Read: {:20} (expected: {})", value, expected);
        assert_eq!(value, expected);
    }

    println!("\n=== Signed Integers (i64) with ZigZag ===\n");

    let mut buffer = ByteBuffer::new();

    let signed_values = [-1000i64, -100, -10, -1, 0, 1, 10, 100, 1000];

    println!("ZigZag encoding efficiently handles negative numbers:\n");

    for &value in &signed_values {
        let pos_before = buffer.write_position();
        buffer.write_varint_i64(value)?;
        let pos_after = buffer.write_position();
        let bytes_used = pos_after - pos_before;

        println!("Value: {:6} -> {} bytes", value, bytes_used);
    }

    println!("\nTotal bytes: {}", buffer.len());

    // Read back
    buffer.reset();
    for &expected in &signed_values {
        let value = buffer.read_varint_i64()?;
        assert_eq!(value, expected);
    }

    println!("\n=== u32 and i32 Variants ===\n");

    let mut buffer = ByteBuffer::new();

    buffer.write_varint_u32(42)?;
    buffer.write_varint_i32(-42)?;

    let v1 = buffer.read_varint_u32()?;
    let v2 = buffer.read_varint_i32()?;

    println!("u32: {}", v1);
    println!("i32: {}", v2);

    println!("\n=== Protocol Buffer Style Message ===\n");

    #[derive(Debug)]
    struct Message {
        id: u64,
        user_id: i32,
        timestamp: u64,
        priority: u32,
    }

    impl Message {
        fn encode(&self, buf: &mut ByteBuffer) -> Result<(), Box<dyn std::error::Error>> {
            buf.write_varint_u64(self.id)?;
            buf.write_varint_i32(self.user_id)?;
            buf.write_varint_u64(self.timestamp)?;
            buf.write_varint_u32(self.priority)?;
            Ok(())
        }

        fn decode(buf: &mut ByteBuffer) -> Result<Self, Box<dyn std::error::Error>> {
            Ok(Self {
                id: buf.read_varint_u64()?,
                user_id: buf.read_varint_i32()?,
                timestamp: buf.read_varint_u64()?,
                priority: buf.read_varint_u32()?,
            })
        }
    }

    let mut buffer = ByteBuffer::new();

    let msg = Message {
        id: 12345,
        user_id: -999,
        timestamp: 1638360000,
        priority: 5,
    };

    println!("Original message: {:?}", msg);

    msg.encode(&mut buffer)?;
    println!("Encoded size: {} bytes", buffer.len());

    buffer.reset();
    let decoded = Message::decode(&mut buffer)?;
    println!("Decoded message: {:?}", decoded);

    println!("\n=== Space Savings ===\n");

    let fixed_size = 8 + 4 + 8 + 4; // Fixed-size encoding
    let varint_size = buffer.len();
    let savings = ((fixed_size - varint_size) as f64 / fixed_size as f64) * 100.0;

    println!("Fixed encoding: {} bytes", fixed_size);
    println!("Varint encoding: {} bytes", varint_size);
    println!("Space savings: {:.1}%", savings);

    Ok(())
}
