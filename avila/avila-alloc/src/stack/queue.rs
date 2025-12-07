//! StackQueue - Circular queue on stack

use core::mem::MaybeUninit;

/// Stack-allocated circular queue
pub struct StackQueue<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const N: usize> StackQueue<T, N> {
    /// Creates a new empty queue
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Returns the number of elements in the queue
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the queue is empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the queue
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Pushes an element to the back of the queue
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(value);
        }
        unsafe {
            self.data[self.tail].as_mut_ptr().write(value);
        }
        self.tail = (self.tail + 1) % N;
        self.len += 1;
        Ok(())
    }

    /// Pops an element from the front of the queue
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let value = unsafe { self.data[self.head].as_ptr().read() };
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(value)
    }

    /// Returns a reference to the front element
    pub fn front(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { &*self.data[self.head].as_ptr() })
        }
    }

    /// Returns a reference to the back element
    pub fn back(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            let back_idx = if self.tail == 0 { N - 1 } else { self.tail - 1 };
            Some(unsafe { &*self.data[back_idx].as_ptr() })
        }
    }

    /// Returns the remaining capacity
    pub const fn remaining_capacity(&self) -> usize {
        N - self.len
    }

    /// Returns `true` if the queue is full
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Pushes an element to the front of the queue (deque operation)
    pub fn push_front(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(value);
        }
        self.head = if self.head == 0 { N - 1 } else { self.head - 1 };
        unsafe {
            self.data[self.head].as_mut_ptr().write(value);
        }
        self.len += 1;
        Ok(())
    }

    /// Pops an element from the back of the queue (deque operation)
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.tail = if self.tail == 0 { N - 1 } else { self.tail - 1 };
        let value = unsafe { self.data[self.tail].as_ptr().read() };
        self.len -= 1;
        Some(value)
    }

    /// Returns an iterator over the queue elements
    pub fn iter(&self) -> QueueIter<'_, T, N> {
        QueueIter {
            queue: self,
            pos: 0,
        }
    }

    /// Clears the queue
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

/// Iterator over queue elements
pub struct QueueIter<'a, T, const N: usize> {
    queue: &'a StackQueue<T, N>,
    pos: usize,
}

impl<'a, T, const N: usize> Iterator for QueueIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.queue.len {
            let idx = (self.queue.head + self.pos) % N;
            self.pos += 1;
            Some(unsafe { &*self.queue.data[idx].as_ptr() })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.queue.len - self.pos;
        (remaining, Some(remaining))
    }
}

impl<'a, T, const N: usize> ExactSizeIterator for QueueIter<'a, T, N> {}

impl<T, const N: usize> Drop for StackQueue<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Default for StackQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
