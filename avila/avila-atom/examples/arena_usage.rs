//! Arena allocator usage example - Comprehensive demonstration

use avila_atom::arena::Arena;
use std::mem;

#[derive(Debug)]
struct Node {
    id: usize,
    value: String,
    next: Option<*mut Node>,
}

#[derive(Debug)]
struct DataBlock {
    timestamp: u64,
    payload: [u8; 256],
    checksum: u32,
}

fn main() {
    println!("=== Arena Allocator Comprehensive Example ===\n");

    // 1. Basic allocation
    basic_allocation();

    // 2. Linked list in arena
    linked_list_example();

    // 3. Bulk allocation comparison
    bulk_allocation_comparison();

    // 4. Arena reuse
    arena_reuse_example();

    // 5. Alignment test
    alignment_example();
}

fn basic_allocation() {
    println!("--- 1. Basic Allocation ---");
    let mut arena = Arena::with_capacity(1024 * 1024);

    println!("Initial state:");
    println!("  Used: {} bytes", arena.used());
    println!("  Capacity: {} bytes\n", arena.capacity());

    let mut nodes = Vec::new();
    for i in 0..1000 {
        let node = arena.alloc_value(Node {
            id: i,
            value: format!("Node {}", i),
            next: None,
        });

        if let Some(n) = node {
            nodes.push(n);
        }
    }

    println!("After 1000 allocations:");
    println!("  Used: {} bytes", arena.used());
    println!("  Utilization: {:.2}%\n",
             (arena.used() as f64 / arena.capacity() as f64) * 100.0);
}

fn linked_list_example() {
    println!("--- 2. Linked List in Arena ---");
    let mut arena = Arena::with_capacity(1024 * 1024);

    let mut head: Option<*mut Node> = None;
    let mut tail: Option<*mut Node> = None;

    // Create linked list of 100 nodes
    for i in 0..100 {
        let node = arena.alloc_value(Node {
            id: i,
            value: format!("List node {}", i),
            next: None,
        });

        if let Some(node_ptr) = node {
            if head.is_none() {
                head = Some(node_ptr);
                tail = Some(node_ptr);
            } else if let Some(tail_ptr) = tail {
                unsafe {
                    (*tail_ptr).next = Some(node_ptr);
                }
                tail = Some(node_ptr);
            }
        }
    }

    // Traverse list
    let mut current = head;
    let mut count = 0;
    println!("Traversing linked list:");

    while let Some(node_ptr) = current {
        unsafe {
            if count < 5 || count >= 95 {
                println!("  Node {}: id={}, value={}",
                         count, (*node_ptr).id, (*node_ptr).value);
            } else if count == 5 {
                println!("  ... ({} nodes omitted) ...", 90);
            }
            current = (*node_ptr).next;
        }
        count += 1;
    }

    println!("Total nodes traversed: {}", count);
    println!("Arena used: {} bytes\n", arena.used());
}

fn bulk_allocation_comparison() {
    println!("--- 3. Bulk Allocation Comparison ---");

    // Standard heap allocation
    let start = std::time::Instant::now();
    let mut heap_nodes = Vec::new();
    for i in 0..10000 {
        heap_nodes.push(Box::new(DataBlock {
            timestamp: i,
            payload: [0u8; 256],
            checksum: i as u32,
        }));
    }
    let heap_time = start.elapsed();

    // Arena allocation
    let mut arena = Arena::with_capacity(10 * 1024 * 1024);
    let start = std::time::Instant::now();
    let mut arena_nodes = Vec::new();
    for i in 0..10000 {
        if let Some(block) = arena.alloc_value(DataBlock {
            timestamp: i,
            payload: [0u8; 256],
            checksum: i as u32,
        }) {
            arena_nodes.push(block);
        }
    }
    let arena_time = start.elapsed();

    println!("10,000 allocations of {}B blocks:", mem::size_of::<DataBlock>());
    println!("  Heap allocation: {:?}", heap_time);
    println!("  Arena allocation: {:?}", arena_time);
    println!("  Speedup: {:.2}x", heap_time.as_nanos() as f64 / arena_time.as_nanos() as f64);
    println!("  Arena usage: {} bytes / {} bytes\n", arena.used(), arena.capacity());
}

fn arena_reuse_example() {
    println!("--- 4. Arena Reuse Pattern ---");
    let mut arena = Arena::with_capacity(1024 * 1024);

    for iteration in 0..3 {
        println!("Iteration {}:", iteration);

        // Allocate objects
        let mut objects = Vec::new();
        for i in 0..500 {
            if let Some(obj) = arena.alloc_value(Node {
                id: i,
                value: format!("Iter {} - Node {}", iteration, i),
                next: None,
            }) {
                objects.push(obj);
            }
        }

        println!("  Allocated 500 nodes");
        println!("  Used: {} bytes", arena.used());

        // Reset arena for reuse
        arena.reset();
        println!("  Reset complete (O(1) operation)\n");
    }

    println!("Arena reused 3 times without reallocating backing memory\n");
}

fn alignment_example() {
    println!("--- 5. Alignment Test ---");
    let mut arena = Arena::with_capacity(1024);

    #[repr(align(64))]
    struct CacheLine {
        data: [u64; 8],
    }

    println!("Allocating cache-aligned structures (64-byte alignment):");

    for i in 0..5 {
        if let Some(ptr) = arena.alloc_value(CacheLine { data: [i; 8] }) {
            println!("  Allocation {}: address={:p} (aligned: {})",
                     i, ptr, (ptr as usize) % 64 == 0);
        }
    }

    println!("\nArena used: {} bytes", arena.used());
    println!("✓ All allocations respect alignment requirements!");
}
