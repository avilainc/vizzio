//! Advanced ByteBuffer features showcase

use avila_buffer::{ByteBuffer, PrimitiveEncoder, PrimitiveDecoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced ByteBuffer Features ===\n");

    // Iterator support
    println!("=== Iterators ===\n");

    let buffer = ByteBuffer::from(b"Hello, World!" as &[u8]);

    println!("Iterating over bytes:");
    for (i, byte) in buffer.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }
    println!("\n");

    // Chunks
    println!("=== Chunks (size 5) ===\n");
    for (i, chunk) in buffer.chunks(5).enumerate() {
        println!("Chunk {}: {:?}", i, std::str::from_utf8(chunk)?);
    }

    // Windows
    println!("\n=== Sliding Windows (size 3) ===\n");
    for (i, window) in buffer.windows(3).enumerate().take(5) {
        println!("Window {}: {:?}", i, std::str::from_utf8(window)?);
    }

    // Pattern matching
    println!("\n=== Pattern Matching ===\n");

    let buffer = ByteBuffer::from(b"The quick brown fox jumps" as &[u8]);

    if let Some(pos) = buffer.find(b'q') {
        println!("Found 'q' at position: {}", pos);
    }

    if let Some(pos) = buffer.find_pattern(b"brown") {
        println!("Found 'brown' at position: {}", pos);
    }

    println!("Starts with 'The': {}", buffer.starts_with(b"The"));
    println!("Ends with 'jumps': {}", buffer.ends_with(b"jumps"));

    // Splitting
    println!("\n=== Buffer Splitting ===\n");

    let buffer = ByteBuffer::from(b"HEADER|PAYLOAD|FOOTER" as &[u8]);

    if let Some(pos) = buffer.find(b'|') {
        let (left, right) = buffer.split_at(pos)?;
        println!("Left: {:?}", std::str::from_utf8(left.as_slice())?);
        println!("Right: {:?}", std::str::from_utf8(right.as_slice())?);
    }

    // Peeking
    println!("\n=== Peeking (non-destructive read) ===\n");

    let mut buffer = ByteBuffer::from(b"Peek at me!" as &[u8]);

    let mut peek_buf = vec![0u8; 4];
    buffer.peek(&mut peek_buf)?;
    println!("Peeked: {:?}", std::str::from_utf8(&peek_buf)?);
    println!("Buffer still has: {} bytes", buffer.len());

    let mut read_buf = vec![0u8; 4];
    buffer.read(&mut read_buf)?;
    println!("After read: {} bytes remaining", buffer.len());

    // Skipping
    println!("\n=== Skipping Bytes ===\n");

    let mut buffer = ByteBuffer::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    println!("Initial length: {}", buffer.len());
    buffer.skip(5)?;
    println!("After skip(5): {}", buffer.len());
    println!("Remaining data: {:?}", buffer.as_slice());

    // Clone and equality
    println!("\n=== Clone and Equality ===\n");

    let buffer1 = ByteBuffer::from(b"Test" as &[u8]);
    let buffer2 = buffer1.clone();
    let buffer3 = ByteBuffer::from(b"Test" as &[u8]);
    let buffer4 = ByteBuffer::from(b"Different" as &[u8]);

    println!("buffer1 == buffer2 (cloned): {}", buffer1 == buffer2);
    println!("buffer1 == buffer3 (same content): {}", buffer1 == buffer3);
    println!("buffer1 == buffer4 (different): {}", buffer1 == buffer4);

    // Extend from iterator
    println!("\n=== Extend from Iterator ===\n");

    let mut buffer = ByteBuffer::new();
    let data = vec![65, 66, 67, 68, 69]; // ASCII A-E

    buffer.extend(data.into_iter())?;
    println!("Extended buffer: {:?}", std::str::from_utf8(buffer.as_slice())?);

    // Position tracking
    println!("\n=== Position Tracking ===\n");

    let mut buffer = ByteBuffer::from(b"Position tracking" as &[u8]);

    println!("Read position: {}", buffer.read_position());
    println!("Write position: {}", buffer.write_position());

    let mut temp = [0u8; 8];
    buffer.read(&mut temp)?;

    println!("After reading 8 bytes:");
    println!("  Read position: {}", buffer.read_position());
    println!("  Write position: {}", buffer.write_position());

    // Complex protocol example
    println!("\n=== Protocol Parsing Example ===\n");

    #[derive(Debug)]
    struct Packet {
        version: u8,
        packet_type: u8,
        length: u16,
        checksum: u32,
        payload: Vec<u8>,
    }

    impl Packet {
        fn encode(&self, buf: &mut ByteBuffer) -> Result<(), Box<dyn std::error::Error>> {
            buf.write_u8(self.version)?;
            buf.write_u8(self.packet_type)?;
            buf.write_u16_be(self.length)?;
            buf.write_u32_be(self.checksum)?;
            buf.write(&self.payload)?;
            Ok(())
        }

        fn decode(buf: &mut ByteBuffer) -> Result<Self, Box<dyn std::error::Error>> {
            let version = buf.read_u8()?;
            let packet_type = buf.read_u8()?;
            let length = buf.read_u16_be()?;
            let checksum = buf.read_u32_be()?;

            let mut payload = vec![0u8; length as usize];
            buf.read(&mut payload)?;

            Ok(Self {
                version,
                packet_type,
                length,
                checksum,
                payload,
            })
        }
    }

    let mut buffer = ByteBuffer::new();

    let packet = Packet {
        version: 1,
        packet_type: 42,
        length: 11,
        checksum: 0xDEADBEEF,
        payload: b"Hello World".to_vec(),
    };

    println!("Original packet: {:?}", packet);

    packet.encode(&mut buffer)?;
    println!("Encoded size: {} bytes", buffer.len());

    buffer.reset();
    let decoded = Packet::decode(&mut buffer)?;
    println!("Decoded packet: {:?}", decoded);

    println!("\n=== Memory Management ===\n");

    let mut buffer = ByteBuffer::with_capacity(64);
    println!("Initial capacity: {}", buffer.capacity());

    buffer.write(&[0u8; 100])?;
    println!("After writing 100 bytes:");
    println!("  Capacity: {}", buffer.capacity());
    println!("  Length: {}", buffer.len());

    buffer.clear();
    buffer.shrink_to_fit();
    println!("After shrink_to_fit:");
    println!("  Capacity: {}", buffer.capacity());

    Ok(())
}
