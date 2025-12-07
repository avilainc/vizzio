//! Advanced usage demonstrating all allocators and features

#![allow(unused)]
use avila_alloc::prelude::*;

fn main() {
    println!("=== avila-alloc Advanced Features ===\n");

    demo_stack_vec();
    demo_stack_string();
    demo_stack_map();
    demo_stack_ring();
    demo_stack_queue();
    demo_static_arena();
    demo_pool();
}

fn demo_stack_vec() {
    println!("1. StackVec - Advanced operations");

    let mut vec = StackVec::<i32, 20>::new();

    // Push elements
    for i in 0..10 {
        vec.push(i * 2).unwrap();
    }

    // Insert in the middle
    vec.insert(5, 999).unwrap();
    println!("   After insert at 5: {:?}", vec.as_slice());

    // Remove element
    let removed = vec.remove(5).unwrap();
    println!("   Removed: {}", removed);

    // Reverse
    vec.reverse();
    println!("   After reverse: {:?}", vec.as_slice());

    // Retain only even numbers
    vec.retain(|&x| x % 2 == 0);
    println!("   After retain evens: {:?}", vec.as_slice());

    // Iterator
    let sum: i32 = vec.iter().sum();
    println!("   Sum: {}\n", sum);
}

fn demo_stack_string() {
    println!("2. StackString - String manipulation");

    let mut s = StackString::<128>::new();
    s.push_str("Hello, Rust!").unwrap();

    // Character operations
    s.push(' ').unwrap();
    s.push_str("🦀").unwrap();
    println!("   String: {}", s.as_str());

    // Unicode safe pop
    let last = s.pop();
    println!("   Popped: {:?}", last);

    // String queries
    println!("   Starts with 'Hello': {}", s.starts_with("Hello"));
    println!("   Contains 'Rust': {}", s.contains("Rust"));

    // Case conversion
    let upper = s.to_uppercase();
    println!("   Uppercase: {}\n", upper.as_str());
}

fn demo_stack_map() {
    println!("3. StackMap - Hash map on stack");

    let mut map = StackMap::<&str, i32, 16>::new();

    // Insert key-value pairs
    map.insert("one", 1).unwrap();
    map.insert("two", 2).unwrap();
    map.insert("three", 3).unwrap();

    // Lookup
    if let Some(&value) = map.get(&"two") {
        println!("   Found 'two': {}", value);
    }

    // Update
    map.insert("two", 22).unwrap();

    // Iterate
    print!("   All entries: ");
    for (k, v) in map.iter() {
        print!("{}={} ", k, v);
    }
    println!();

    // Remove
    let removed = map.remove(&"one");
    println!("   Removed 'one': {:?}", removed);
    println!("   Map size: {} / {}\n", map.len(), map.capacity());
}

fn demo_stack_ring() {
    println!("4. StackRing - Ring buffer");

    let mut ring = StackRing::<i32, 5>::new();

    // Fill the ring
    for i in 0..5 {
        ring.push(i);
    }
    println!("   Ring full: {:?}", ring.iter().collect::<Vec<_>>());

    // Overflow - oldest element is overwritten
    let overwritten = ring.push(100);
    println!("   After push 100, overwritten: {:?}", overwritten);
    println!("   Ring now: {:?}", ring.iter().collect::<Vec<_>>());

    // Access elements
    println!("   Front: {:?}", ring.front());
    println!("   Back: {:?}", ring.back());
    println!("   Element at index 2: {:?}\n", ring.get(2));
}

fn demo_stack_queue() {
    println!("5. StackQueue - Deque operations");

    let mut queue = StackQueue::<&str, 10>::new();

    // Standard queue operations
    queue.push("first").unwrap();
    queue.push("second").unwrap();
    queue.push("third").unwrap();

    // Deque operations
    queue.push_front("zero").unwrap();
    println!("   After push_front: {:?}",
             queue.iter().collect::<Vec<_>>());

    // Pop from both ends
    println!("   Pop front: {:?}", queue.pop());
    println!("   Pop back: {:?}", queue.pop_back());

    println!("   Remaining: {:?}\n",
             queue.iter().collect::<Vec<_>>());
}

fn demo_static_arena() {
    println!("6. StaticArena - Bump allocator");

    let mut arena = StaticArena::<1024>::new();

    // Allocate various types
    let num = arena.alloc_with(|| 42u64).unwrap();
    *num = 100;
    println!("   Allocated u64: {}", num);

    // Allocate slice
    let slice = arena.alloc_slice::<i32>(10).unwrap();
    for i in 0..10 {
        slice[i] = i as i32 * 10;
    }
    println!("   Allocated slice: {:?}", slice);

    // Check usage
    println!("   Arena usage: {} / {} bytes", arena.used(), arena.capacity());
    println!("   Utilization: {:.1}%", arena.utilization() * 100.0);

    // Scoped allocation
    {
        let mut scope = arena.scope();
        let temp = scope.alloc_slice::<u8>(100).unwrap();
        temp[0] = 0xFF;
        println!("   Scoped allocation: {} bytes", temp.len());
    }
    // Scope dropped, memory reclaimed

    println!("   After scope: {} bytes used\n", arena.used());
}

fn demo_pool() {
    println!("7. Pool - Object pool");

    let mut pool = Pool::<String, 8>::new();

    // Allocate objects
    let obj1 = pool.alloc("Hello".to_string()).unwrap();
    let obj2 = pool.alloc("World".to_string()).unwrap();

    println!("   Object 1: {}", obj1.get());
    println!("   Object 2: {}", obj2.get());

    println!("   Pool: {} / {} allocated",
             pool.allocated(), pool.capacity());

    // Objects are automatically returned on drop
    drop(obj1);
    drop(obj2);

    println!("   After drop: {} allocated\n", pool.allocated());
}
