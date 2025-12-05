//! StackVec - Fixed-capacity vector on stack

use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr, slice,
};

/// Stack-allocated vector with fixed capacity
#[derive(Debug)]
pub struct StackVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> StackVec<T, N> {
    /// Creates a new empty `StackVec`
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the number of elements in the vector
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector is empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the total capacity of the vector
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the remaining capacity
    pub const fn remaining_capacity(&self) -> usize {
        N - self.len
    }

    /// Pushes an element onto the vector
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(value);
        }
        unsafe {
            self.data[self.len].as_mut_ptr().write(value);
        }
        self.len += 1;
        Ok(())
    }

    /// Pops an element from the vector
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { self.data[self.len].as_ptr().read() })
    }

    /// Returns a slice of the vector's contents
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }

    /// Returns a mutable slice of the vector's contents
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.len) }
    }

    /// Clears the vector, dropping all elements
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    /// Returns a reference to an element at index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self[index])
        } else {
            None
        }
    }

    /// Returns a mutable reference to an element at index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(&mut self[index])
        } else {
            None
        }
    }

    /// Inserts an element at position, shifting all elements after it to the right
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
        if index > self.len {
            return Err(value);
        }
        if self.len >= N {
            return Err(value);
        }

        unsafe {
            let p = self.data.as_mut_ptr().add(index);
            ptr::copy(p, p.add(1), self.len - index);
            p.write(MaybeUninit::new(value));
        }
        self.len += 1;
        Ok(())
    }

    /// Removes and returns the element at position, shifting all elements after it to the left
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            let p = self.data.as_mut_ptr().add(index);
            let value = p.read().assume_init();
            ptr::copy(p.add(1), p, self.len - index - 1);
            self.len -= 1;
            Some(value)
        }
    }

    /// Swaps two elements in the vector
    pub fn swap(&mut self, a: usize, b: usize) {
        assert!(a < self.len && b < self.len);
        self.as_mut_slice().swap(a, b);
    }

    /// Reverses the order of elements in the vector
    pub fn reverse(&mut self) {
        self.as_mut_slice().reverse();
    }

    /// Returns an iterator over the vector
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the vector
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Extends the vector with the contents of a slice
    pub fn extend_from_slice(&mut self, other: &[T]) -> Result<(), ()>
    where
        T: Clone,
    {
        if self.len + other.len() > N {
            return Err(());
        }
        for item in other {
            let _ = self.push(item.clone());
        }
        Ok(())
    }

    /// Retains only the elements specified by the predicate
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut del = 0;
        let len = self.len;
        for i in 0..len {
            if !f(&self[i]) {
                del += 1;
            } else if del > 0 {
                self.as_mut_slice().swap(i - del, i);
            }
        }
        if del > 0 {
            self.truncate(len - del);
        }
    }

    /// Shortens the vector, keeping the first `len` elements
    pub fn truncate(&mut self, len: usize) {
        while self.len > len {
            self.pop();
        }
    }

    /// Returns the first element of the vector
    pub fn first(&self) -> Option<&T> {
        if self.len > 0 {
            Some(&self[0])
        } else {
            None
        }
    }

    /// Returns the last element of the vector
    pub fn last(&self) -> Option<&T> {
        if self.len > 0 {
            Some(&self[self.len - 1])
        } else {
            None
        }
    }

    /// Returns true if the vector contains an element
    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq,
    {
        self.as_slice().contains(x)
    }
}

impl<T, const N: usize> Drop for StackVec<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Deref for StackVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> DerefMut for StackVec<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> Default for StackVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Index<usize> for StackVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.as_slice()[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for StackVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.as_mut_slice()[index]
    }
}

impl<T: Clone, const N: usize> Clone for StackVec<T, N> {
    fn clone(&self) -> Self {
        let mut new_vec = Self::new();
        for item in self.iter() {
            let _ = new_vec.push(item.clone());
        }
        new_vec
    }
}

impl<T: PartialEq, const N: usize> PartialEq for StackVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, const N: usize> Eq for StackVec<T, N> {}

impl<T, const N: usize> IntoIterator for StackVec<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: self,
            pos: 0,
        }
    }
}

/// Iterator that consumes StackVec
pub struct IntoIter<T, const N: usize> {
    vec: StackVec<T, N>,
    pos: usize,
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.vec.len {
            let item = unsafe {
                self.vec.data[self.pos].as_ptr().read()
            };
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.len - self.pos;
        (remaining, Some(remaining))
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {}

impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        // Drop remaining elements
        while self.next().is_some() {}
        // Prevent StackVec from dropping already-consumed elements
        self.vec.len = 0;
    }
}
