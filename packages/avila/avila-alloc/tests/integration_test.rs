//! Integration tests for avila-alloc

use avila_alloc::{StackVec, StackString, StaticArena};

#[test]
fn test_stack_vec_basic() {
    let mut vec = StackVec::<i32, 10>::new();
    assert!(vec.is_empty());
    assert_eq!(vec.capacity(), 10);

    vec.push(1).unwrap();
    vec.push(2).unwrap();
    vec.push(3).unwrap();

    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);
}

#[test]
fn test_stack_vec_overflow() {
    let mut vec = StackVec::<u8, 2>::new();
    assert!(vec.push(1).is_ok());
    assert!(vec.push(2).is_ok());
    assert!(vec.push(3).is_err());
}

#[test]
fn test_stack_string_basic() {
    let mut s = StackString::<32>::new();
    assert!(s.is_empty());

    s.push_str("Hello").unwrap();
    assert_eq!(s.as_str(), "Hello");

    s.push_str(", World!").unwrap();
    assert_eq!(s.as_str(), "Hello, World!");
    assert_eq!(s.len(), 13);
}

#[test]
fn test_stack_string_unicode() {
    let mut s = StackString::<64>::new();
    s.push_str("こんにちは").unwrap();
    assert_eq!(s.as_str(), "こんにちは");

    s.push_str(" 世界").unwrap();
    assert_eq!(s.as_str(), "こんにちは 世界");
}

#[test]
fn test_static_arena() {
    let mut arena = StaticArena::<1024>::new();

    let val1 = arena.alloc::<u64>().unwrap();
    *val1 = 42;
    assert_eq!(*val1, 42);

    let slice = arena.alloc_slice::<u32>(5).unwrap();
    slice[0] = 100;
    slice[4] = 200;
    assert_eq!(slice[0], 100);
    assert_eq!(slice[4], 200);
}

#[test]
fn test_arena_reset() {
    let mut arena = StaticArena::<512>::new();

    arena.alloc::<u64>().unwrap();
    let used = arena.used();
    assert!(used > 0);

    arena.reset();
    assert_eq!(arena.used(), 0);
}

#[cfg(feature = "std")]
#[test]
fn test_std_arena() {
    use avila_alloc::Arena;

    let mut arena = Arena::new(1024);
    let val = arena.alloc::<i32>();
    *val = 999;
    assert_eq!(*val, 999);
}
