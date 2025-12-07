//! StackBox - Single value storage on stack

use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

/// Stack-allocated box for a single value
pub struct StackBox<T, const N: usize> {
    data: MaybeUninit<T>,
    initialized: bool,
}

impl<T, const N: usize> StackBox<T, N> {
    /// Creates a new empty `StackBox`
    pub const fn new() -> Self {
        Self {
            data: MaybeUninit::uninit(),
            initialized: false,
        }
    }

    /// Stores a value in the box
    pub fn store(&mut self, value: T) {
        if self.initialized {
            unsafe {
                self.data.assume_init_drop();
            }
        }
        self.data = MaybeUninit::new(value);
        self.initialized = true;
    }

    /// Takes the value out of the box
    pub fn take(&mut self) -> Option<T> {
        if !self.initialized {
            return None;
        }
        self.initialized = false;
        Some(unsafe { self.data.assume_init_read() })
    }

    /// Returns a reference to the value
    pub fn get(&self) -> Option<&T> {
        if self.initialized {
            Some(unsafe { self.data.assume_init_ref() })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.initialized {
            Some(unsafe { self.data.assume_init_mut() })
        } else {
            None
        }
    }

    /// Returns `true` if the box contains a value
    pub const fn is_some(&self) -> bool {
        self.initialized
    }

    /// Returns `true` if the box is empty
    pub const fn is_none(&self) -> bool {
        !self.initialized
    }

    /// Replaces the value in the box, returning the old value
    pub fn replace(&mut self, value: T) -> Option<T> {
        let old = self.take();
        self.store(value);
        old
    }

    /// Maps the contained value by applying a function
    pub fn map<U, F>(self, f: F) -> StackBox<U, N>
    where
        F: FnOnce(T) -> U,
    {
        let mut result = StackBox::new();
        if self.initialized {
            let value = unsafe { self.data.assume_init_read() };
            result.store(f(value));
            core::mem::forget(self);
        }
        result
    }

    /// Returns the contained value or a default
    pub fn unwrap_or(mut self, default: T) -> T {
        self.take().unwrap_or(default)
    }

    /// Returns the contained value or computes it from a closure
    pub fn unwrap_or_else<F>(mut self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.take().unwrap_or_else(f)
    }

    /// Creates a new StackBox with a value
    pub fn new_with(value: T) -> Self {
        Self {
            data: MaybeUninit::new(value),
            initialized: true,
        }
    }
}

impl<T, const N: usize> Drop for StackBox<T, N> {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                self.data.assume_init_drop();
            }
        }
    }
}

impl<T, const N: usize> Default for StackBox<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const N: usize> Clone for StackBox<T, N> {
    fn clone(&self) -> Self {
        let mut new_box = Self::new();
        if let Some(value) = self.get() {
            new_box.store(value.clone());
        }
        new_box
    }
}

impl<T, const N: usize> From<T> for StackBox<T, N> {
    fn from(value: T) -> Self {
        Self::new_with(value)
    }
}

impl<T: core::fmt::Debug, const N: usize> core::fmt::Debug for StackBox<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.get() {
            Some(value) => write!(f, "StackBox({:?})", value),
            None => write!(f, "StackBox(None)"),
        }
    }
}
