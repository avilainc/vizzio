//! Buffer pool example for high-throughput scenarios

use avila_buffer::{BufferPool, PooledBuffer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== BufferPool Example ===\n");

    // Create a pool with 5 buffers of 1024 bytes each
    let mut pool = BufferPool::new(5, 1024);

    println!("Created pool:");
    println!("  Capacity: {} buffers", pool.capacity());
    println!("  Buffer size: {} bytes", pool.buffer_size());
    println!("  Available: {}", pool.available());

    println!("\n=== Basic Usage ===\n");

    // Acquire a buffer
    let mut buf1 = pool.acquire()?;
    println!("Acquired buffer 1");
    println!("  Available in pool: {}", pool.available());

    // Use the buffer
    buf1.write(b"Hello from pooled buffer!")?;
    println!("  Written {} bytes", buf1.len());

    // Release back to pool
    pool.release(buf1);
    println!("Released buffer 1");
    println!("  Available in pool: {}", pool.available());

    println!("\n=== RAII Pattern with PooledBuffer ===\n");

    {
        let mut buf = PooledBuffer::new(&mut pool)?;
        println!("Acquired with RAII guard");
        println!("  Available: {}", pool.available());

        buf.write(b"Auto-release on drop")?;
        println!("  Buffer contains: {:?}", std::str::from_utf8(buf.as_slice())?);

        // Buffer automatically released when going out of scope
    }

    println!("After scope ended:");
    println!("  Available: {}", pool.available());

    println!("\n=== High-Throughput Simulation ===\n");

    let mut pool = BufferPool::new(3, 512);

    for i in 0..10 {
        if let Ok(mut buf) = pool.acquire() {
            buf.write(format!("Message {}", i).as_bytes())?;
            println!("Processed message {}: {} bytes", i, buf.len());
            pool.release(buf);
        } else {
            println!("Pool exhausted at message {}", i);
            break;
        }
    }

    println!("\n=== Pool Statistics ===\n");

    println!("Total buffers created: {}", pool.total_created());
    println!("Total acquisitions: {}", pool.total_acquired());
    println!("Total releases: {}", pool.total_released());
    println!("Currently available: {}", pool.available());

    println!("\n=== Lazy Allocation Pool ===\n");

    let mut lazy_pool = BufferPool::with_capacity(10, 256);
    println!("Created lazy pool (no pre-allocation)");
    println!("  Available: {}", lazy_pool.available());

    let _buf1 = lazy_pool.acquire()?;
    println!("After first acquire:");
    println!("  Created: {}", lazy_pool.total_created());
    println!("  Available: {}", lazy_pool.available());

    println!("\n=== Memory Efficiency ===\n");

    let pool = BufferPool::new(100, 4096);
    let pool_memory = 100 * 4096;
    println!("Pool of 100 x 4KB buffers = {} KB total", pool_memory / 1024);
    println!("Reusable for unlimited operations!");

    Ok(())
}
