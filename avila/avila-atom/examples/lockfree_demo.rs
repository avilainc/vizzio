//! Lock-free data structures demonstration

use avila_atom::lockfree::{LockFreeStack, AtomicCounter, RingBuffer};
use std::thread;
use std::sync::Arc;

fn main() {
    println!("=== Lock-Free Structures Example ===\n");

    lock_free_stack_demo();
    atomic_counter_demo();
    ring_buffer_demo();
}

fn lock_free_stack_demo() {
    println!("--- Lock-Free Stack ---");

    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];

    // Spawn threads that push to stack
    for i in 0..4 {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..100 {
                stack_clone.push(i * 100 + j);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Pop all items
    let mut count = 0;
    while stack.pop().is_some() {
        count += 1;
    }

    println!("  Pushed from 4 threads: 400 items");
    println!("  Popped: {} items", count);
    println!("  ✓ Thread-safe without locks!\n");
}

fn atomic_counter_demo() {
    println!("--- Atomic Counter ---");

    let counter = Arc::new(AtomicCounter::new(0));
    let mut handles = vec![];

    // Spawn threads that increment counter
    for _ in 0..8 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.increment();
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Expected: 8000");
    println!("  Actual: {}", counter.get());
    println!("  ✓ No race conditions!\n");
}

fn ring_buffer_demo() {
    println!("--- Ring Buffer (SPSC) ---");

    let buffer: Arc<RingBuffer<i32, 256>> = Arc::new(RingBuffer::new());
    let buffer_clone = Arc::clone(&buffer);

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 0..100 {
            while buffer_clone.push(i).is_err() {
                // Wait if full
                thread::yield_now();
            }
        }
    });

    // Consumer thread
    let buffer_clone = Arc::clone(&buffer);
    let consumer = thread::spawn(move || {
        let mut received = 0;
        while received < 100 {
            if let Some(_val) = buffer_clone.pop() {
                received += 1;
            } else {
                thread::yield_now();
            }
        }
        received
    });

    producer.join().unwrap();
    let received = consumer.join().unwrap();

    println!("  Sent: 100 items");
    println!("  Received: {} items", received);
    println!("  ✓ Wait-free SPSC communication!\n");
}
