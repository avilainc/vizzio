use avila_buffer::{ByteBuffer, RingBuffer};

#[test]
fn test_byte_buffer_from_vec() {
    let data = vec![1, 2, 3, 4, 5];
    let buffer = ByteBuffer::from_vec(data);

    assert_eq!(buffer.len(), 5);
    assert_eq!(buffer.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_byte_buffer_resize() {
    let mut buffer = ByteBuffer::with_capacity(4);

    // Write more than capacity
    buffer.write(&[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();

    assert_eq!(buffer.len(), 8);
    assert!(buffer.capacity() >= 8);
}

#[test]
fn test_byte_buffer_clear() {
    let mut buffer = ByteBuffer::from_vec(vec![1, 2, 3, 4]);

    buffer.clear();

    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
}

#[test]
fn test_ring_buffer_wraparound() {
    let mut ring = RingBuffer::<i32, 3>::new();

    // Fill the buffer
    ring.push(1).unwrap();
    ring.push(2).unwrap();
    ring.push(3).unwrap();

    // Remove one
    assert_eq!(ring.pop(), Some(1));

    // Add another (tests wraparound)
    ring.push(4).unwrap();

    assert_eq!(ring.pop(), Some(2));
    assert_eq!(ring.pop(), Some(3));
    assert_eq!(ring.pop(), Some(4));
    assert!(ring.is_empty());
}

#[test]
fn test_ring_buffer_clear() {
    let mut ring = RingBuffer::<i32, 5>::new();

    ring.push(1).unwrap();
    ring.push(2).unwrap();
    ring.push(3).unwrap();

    ring.clear();

    assert!(ring.is_empty());
    assert_eq!(ring.len(), 0);
}

#[test]
fn test_ring_buffer_peek() {
    let mut ring = RingBuffer::<i32, 3>::new();

    ring.push(42).unwrap();

    // Peek doesn't consume
    assert_eq!(ring.peek(), Some(&42));
    assert_eq!(ring.len(), 1);

    // Pop does consume
    assert_eq!(ring.pop(), Some(42));
    assert!(ring.is_empty());
}

#[test]
fn test_byte_buffer_partial_read() {
    let mut buffer = ByteBuffer::from_vec(vec![1, 2, 3, 4, 5]);

    let mut buf = [0u8; 2];
    let n = buffer.read(&mut buf).unwrap();

    assert_eq!(n, 2);
    assert_eq!(buf, [1, 2]);
    assert_eq!(buffer.len(), 3);
}

#[test]
fn test_byte_buffer_multiple_writes() {
    let mut buffer = ByteBuffer::with_capacity(16);

    buffer.write(b"Hello").unwrap();
    buffer.write(b" ").unwrap();
    buffer.write(b"World").unwrap();

    assert_eq!(buffer.as_slice(), b"Hello World");
}
