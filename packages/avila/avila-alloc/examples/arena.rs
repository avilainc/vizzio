//! Arena allocator example for temporary allocations

#[cfg(feature = "std")]
fn main() {
    use avila_alloc::Arena;

    println!("=== Arena Allocator Example ===\n");

    // Create an arena with 4KB initial capacity
    let mut arena = Arena::new(4096);

    // Allocate various types
    let number = arena.alloc(42u64);
    println!("Allocated u64: {}", number);

    let slice = arena.alloc_slice::<i32>(10);
    for i in 0..10 {
        slice[i] = i as i32 * 10;
    }
    println!("Allocated slice: {:?}", slice);

    // Check usage
    println!("\nArena statistics:");
    println!("  Used: {} bytes", arena.used_bytes());
    println!("  Total: {} bytes", arena.total_allocated());

    // Reset arena to reuse memory
    arena.reset();
    println!("\nAfter reset:");
    println!("  Used: {} bytes", arena.used_bytes());

    // Allocate again in the same memory
    let data = arena.alloc_slice::<u8>(100);
    data[0] = 0xFF;
    data[99] = 0xAA;
    println!("  Reused arena for {} bytes", data.len());
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the 'std' feature.");
    println!("Run with: cargo run --example arena --features std");
}
