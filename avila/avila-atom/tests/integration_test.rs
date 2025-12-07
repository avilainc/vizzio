//! Integration tests for core data structures

use avila_atom::{DynamicArray, AssociativeArray, StringBuffer};

#[test]
fn test_dynamic_array_operations() {
    let mut vec = DynamicArray::new();

    // Push elements
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec[0], 0);
    assert_eq!(vec[99], 99);

    // Pop elements
    for _ in 0..50 {
        vec.pop();
    }

    assert_eq!(vec.len(), 50);
}

#[test]
fn test_associative_array_operations() {
    let mut map = AssociativeArray::new();

    // Insert
    for i in 0..50 {
        map.insert(i, i * 2);
    }

    assert_eq!(map.len(), 50);

    // Get
    assert_eq!(map.get(&25), Some(&50));
    assert_eq!(map.get(&100), None);

    // Remove
    map.remove(&25);
    assert_eq!(map.get(&25), None);
    assert_eq!(map.len(), 49);
}

#[test]
fn test_string_buffer_operations() {
    let mut s = StringBuffer::new();

    s.push_str("Hello");
    assert_eq!(s.len(), 5);

    s.push_str(", World!");
    assert_eq!(s, "Hello, World!");

    s.clear();
    assert!(s.is_empty());
}

#[test]
fn test_collect_into_dynamic_array() {
    let v: DynamicArray<_> = (0..10).collect();
    assert_eq!(v.len(), 10);
    assert_eq!(v[5], 5);
}

#[test]
fn test_from_iterator() {
    let v: DynamicArray<_> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(v, vec![1, 2, 3]);
}
