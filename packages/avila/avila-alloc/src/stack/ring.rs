//! StackRing - Ring buffer implementation on stack

use core::mem::MaybeUninit;

/// Stack-allocated ring buffer with overwrite capability
pub struct StackRing<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    head: usize,
    len: usize,
}

impl<T, const N: usize> StackRing<T, N> {
    /// Creates a new empty ring buffer
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            head: 0,
            len: 0,
        }
    }

    /// Returns the number of elements in the buffer
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer is empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the buffer
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns `true` if the buffer is full
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Pushes an element, overwriting the oldest element if full
    pub fn push(&mut self, value: T) -> Option<T> {
        let tail = (self.head + self.len) % N;

        let old_value = if self.len == N {
            // Buffer is full, overwrite oldest
            let old = unsafe { self.data[tail].as_ptr().read() };
            self.head = (self.head + 1) % N;
            Some(old)
        } else {
            self.len += 1;
            None
        };

        unsafe {
            self.data[tail].as_mut_ptr().write(value);
        }

        old_value
    }

    /// Pops the oldest element from the buffer
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let value = unsafe { self.data[self.head].as_ptr().read() };
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(value)
    }

    /// Returns a reference to the oldest element
    pub fn front(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { &*self.data[self.head].as_ptr() })
        }
    }

    /// Returns a reference to the newest element
    pub fn back(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            let idx = (self.head + self.len - 1) % N;
            Some(unsafe { &*self.data[idx].as_ptr() })
        }
    }

    /// Returns a reference to an element at index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            let idx = (self.head + index) % N;
            Some(unsafe { &*self.data[idx].as_ptr() })
        } else {
            None
        }
    }

    /// Returns a mutable reference to an element at index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            let idx = (self.head + index) % N;
            Some(unsafe { &mut *self.data[idx].as_mut_ptr() })
        } else {
            None
        }
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    /// Returns an iterator over the buffer
    pub fn iter(&self) -> RingIter<'_, T, N> {
        RingIter {
            ring: self,
            pos: 0,
        }
    }

    /// Applies a function to each element in order
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        for i in 0..self.len {
            let idx = (self.head + i) % N;
            unsafe { f(&*self.data[idx].as_ptr()) }
        }
    }

    /// Fills the buffer with copies of a value
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.clear();
        for _ in 0..N {
            self.push(value.clone());
        }
    }
}

/// Iterator over ring buffer elements
pub struct RingIter<'a, T, const N: usize> {
    ring: &'a StackRing<T, N>,
    pos: usize,
}

impl<'a, T, const N: usize> Iterator for RingIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.ring.len {
            let idx = (self.ring.head + self.pos) % N;
            self.pos += 1;
            Some(unsafe { &*self.ring.data[idx].as_ptr() })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.ring.len - self.pos;
        (remaining, Some(remaining))
    }
}

impl<'a, T, const N: usize> ExactSizeIterator for RingIter<'a, T, N> {}

impl<T, const N: usize> Drop for StackRing<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Default for StackRing<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: core::fmt::Debug, const N: usize> core::fmt::Debug for StackRing<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Clone, const N: usize> Clone for StackRing<T, N> {
    fn clone(&self) -> Self {
        let mut new_ring = Self::new();
        for item in self.iter() {
            new_ring.push(item.clone());
        }
        new_ring
    }
}
