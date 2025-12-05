//! Fixed buffer example for embedded/no_std environments

use avila_buffer::FixedBuffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FixedBuffer Example ===\n");

    // Stack-allocated buffer - no heap allocation!
    let mut buffer: FixedBuffer<256> = FixedBuffer::new();

    println!("Created fixed buffer with capacity: {}", buffer.capacity());
    println!("Size on stack: {} bytes", std::mem::size_of_val(&buffer));

    // Write data
    let message = b"Fixed buffers are perfect for embedded systems!";
    buffer.write(message)?;
    println!("Written {} bytes", buffer.len());

    // Read back
    let mut output = [0u8; 47];
    buffer.read(&mut output)?;
    println!("Read: {:?}", std::str::from_utf8(&output)?);

    println!("\n=== Performance Test ===\n");

    // Multiple operations
    let mut buf: FixedBuffer<1024> = FixedBuffer::new();

    for i in 0..10 {
        buf.write(&[i as u8; 10])?;
    }

    println!("Filled buffer with {} bytes", buf.len());
    println!("Remaining space: {} bytes", buf.remaining());

    // Compact
    let mut temp = [0u8; 50];
    buf.read(&mut temp)?;
    buf.compact();

    println!("After compact - available: {}", buf.len());

    println!("\n=== Overflow Handling ===\n");

    let mut small: FixedBuffer<10> = FixedBuffer::new();
    small.write(b"12345")?;

    match small.write(b"67890123456") {
        Ok(_) => println!("Write succeeded"),
        Err(e) => println!("Write failed as expected: {}", e),
    }

    println!("\n=== Clone Performance (Copy) ===\n");

    let original: FixedBuffer<128> = FixedBuffer::new();
    let cloned = original; // Copy, not expensive clone!

    println!("FixedBuffer implements Copy trait - very efficient!");
    println!("Original and cloned are independent stack values");

    Ok(())
}
