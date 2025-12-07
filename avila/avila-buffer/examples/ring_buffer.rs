//! Ring buffer usage example

use avila_buffer::RingBuffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RingBuffer Example ===\n");

    // Create a ring buffer for integers with capacity 5
    let mut ring: RingBuffer<i32, 5> = RingBuffer::new();

    println!("Created ring buffer with capacity: {}", ring.capacity());
    println!("Is empty: {}", ring.is_empty());

    // Push some items
    println!("\nPushing items: 10, 20, 30");
    ring.push(10)?;
    ring.push(20)?;
    ring.push(30)?;

    println!("Current size: {}", ring.len());
    println!("Is full: {}", ring.is_full());

    // Peek without removing
    if let Some(front) = ring.peek() {
        println!("Front item (peek): {}", front);
    }

    // Pop items
    println!("\nPopping items:");
    while let Some(item) = ring.pop() {
        println!("  Popped: {}", item);
    }

    println!("\n=== Wraparound Demonstration ===\n");

    // Fill the buffer
    println!("Filling buffer: 1, 2, 3, 4, 5");
    ring.push(1)?;
    ring.push(2)?;
    ring.push(3)?;
    ring.push(4)?;
    ring.push(5)?;

    println!("Buffer is full: {}", ring.is_full());

    // Try to push when full
    if let Err(e) = ring.push(6) {
        println!("Cannot push to full buffer: {}", e);
    }

    // Remove two items
    println!("\nRemoving two items:");
    println!("  Popped: {}", ring.pop().unwrap());
    println!("  Popped: {}", ring.pop().unwrap());

    // Add two more (demonstrates circular behavior)
    println!("\nAdding items 6 and 7:");
    ring.push(6)?;
    ring.push(7)?;

    println!("\nFinal contents:");
    while let Some(item) = ring.pop() {
        println!("  {}", item);
    }

    println!("\n=== Event Queue Example ===\n");

    // Simulate an event queue
    #[derive(Debug)]
    struct Event {
        id: u32,
        message: &'static str,
    }

    let mut event_queue: RingBuffer<Event, 3> = RingBuffer::new();

    // Add events
    event_queue.push(Event { id: 1, message: "User logged in" })?;
    event_queue.push(Event { id: 2, message: "File uploaded" })?;
    event_queue.push(Event { id: 3, message: "Cache cleared" })?;

    println!("Processing events:");
    while let Some(event) = event_queue.pop() {
        println!("  Event {}: {}", event.id, event.message);
    }

    Ok(())
}
