//! Basic buffer operations example

use avila_buffer::ByteBuffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic ByteBuffer Example ===\n");

    // Create a buffer with capacity
    let mut buffer = ByteBuffer::with_capacity(1024);
    println!("Created buffer with capacity: {}", buffer.capacity());

    // Write some data
    let message = b"Hello, avila-buffer!";
    let written = buffer.write(message)?;
    println!("Written {} bytes: {:?}", written, std::str::from_utf8(message)?);

    // Check available data
    println!("Available to read: {} bytes", buffer.available());

    // Read the data
    let mut output = vec![0u8; message.len()];
    let read = buffer.read(&mut output)?;
    println!("Read {} bytes: {:?}", read, std::str::from_utf8(&output)?);

    println!("\n=== Multiple Write/Read Operations ===\n");

    // Multiple writes
    buffer.write(b"First ")?;
    buffer.write(b"Second ")?;
    buffer.write(b"Third")?;

    println!("Buffer content: {:?}", std::str::from_utf8(buffer.as_slice())?);

    // Partial read
    let mut partial = vec![0u8; 6];
    buffer.read(&mut partial)?;
    println!("Partial read: {:?}", std::str::from_utf8(&partial)?);

    // Compact the buffer
    println!("Before compact - available: {}", buffer.available());
    buffer.compact();
    println!("After compact - available: {}", buffer.available());

    // Read remaining
    let remaining = buffer.as_slice();
    println!("Remaining data: {:?}", std::str::from_utf8(remaining)?);

    Ok(())
}
