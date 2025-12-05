//! Basic usage examples for avila-alloc

use avila_alloc::{StackVec, StackString, StaticArena};

fn main() {
    println!("=== avila-alloc Examples ===\n");

    // StackVec example
    println!("1. StackVec - Fixed-capacity vector on stack");
    let mut vec = StackVec::<i32, 10>::new();
    vec.push(1).unwrap();
    vec.push(2).unwrap();
    vec.push(3).unwrap();
    println!("   Vec: {:?}", vec.as_slice());
    println!("   Length: {}, Capacity: {}\n", vec.len(), vec.capacity());

    // StackString example
    println!("2. StackString - UTF-8 string on stack");
    let mut s = StackString::<64>::new();
    s.push_str("Hello, ").unwrap();
    s.push_str("World!").unwrap();
    println!("   String: {}", s.as_str());
    println!("   Length: {} bytes\n", s.len());

    // StaticArena example
    println!("3. StaticArena - Bump allocator (no_std compatible)");
    let mut arena = StaticArena::<1024>::new();

    let val1 = arena.alloc::<u64>().unwrap();
    *val1 = 42;

    let val2 = arena.alloc::<u64>().unwrap();
    *val2 = 84;

    println!("   Allocated values: {}, {}", val1, val2);
    println!("   Arena used: {} / {} bytes\n", arena.used(), arena.capacity());

    println!("All examples completed successfully!");
}
