//! Tests for stack allocators

use avila_alloc::{StackVec, StackString, StackBox, StackQueue};

#[test]
fn stack_vec_operations() {
    let mut vec = StackVec::<i32, 5>::new();

    // Push elements
    for i in 0..5 {
        assert!(vec.push(i).is_ok());
    }

    // Pop elements
    assert_eq!(vec.pop(), Some(4));
    assert_eq!(vec.len(), 4);

    // Clear
    vec.clear();
    assert!(vec.is_empty());
}

#[test]
fn stack_string_operations() {
    let mut s = StackString::<128>::new();

    s.push_str("Rust").unwrap();
    s.push(' ').unwrap();
    s.push_str("Programming").unwrap();

    assert_eq!(s.as_str(), "Rust Programming");

    s.clear();
    assert!(s.is_empty());
}

#[test]
fn stack_box_operations() {
    let mut b = StackBox::<u64, 8>::new();

    assert!(b.get().is_none());

    b.store(42);
    assert_eq!(*b.get().unwrap(), 42);

    let val = b.take();
    assert_eq!(val, Some(42));
    assert!(b.get().is_none());
}

#[test]
fn stack_queue_operations() {
    let mut q = StackQueue::<i32, 4>::new();

    q.push(1).unwrap();
    q.push(2).unwrap();
    q.push(3).unwrap();

    assert_eq!(q.front(), Some(&1));
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.len(), 1);
}

#[test]
fn stack_queue_circular() {
    let mut q = StackQueue::<u8, 3>::new();

    q.push(1).unwrap();
    q.push(2).unwrap();
    q.pop();
    q.push(3).unwrap();
    q.push(4).unwrap();

    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.pop(), Some(3));
    assert_eq!(q.pop(), Some(4));
}
