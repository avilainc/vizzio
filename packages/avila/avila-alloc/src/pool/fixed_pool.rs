//! Fixed-size object pool

use core::mem::MaybeUninit;
use core::marker::PhantomData;

/// Fixed-size object pool with improved API
pub struct Pool<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    free_list: [bool; N],
    allocated: usize,
}

impl<T, const N: usize> Pool<T, N> {
    /// Creates a new pool
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            free_list: [true; N],
            allocated: 0,
        }
    }

    /// Allocates and initializes an object from the pool
    pub fn alloc(&mut self, value: T) -> Result<PoolSlot<T, N>, T> {
        for (idx, &free) in self.free_list.iter().enumerate() {
            if free {
                self.data[idx] = MaybeUninit::new(value);
                self.free_list[idx] = false;
                self.allocated += 1;
                return Ok(PoolSlot {
                    pool: self as *mut _,
                    idx,
                    _phantom: PhantomData,
                });
            }
        }
        Err(value)
    }

    /// Allocates a slot without initializing
    pub fn alloc_uninit(&mut self) -> Option<PoolHandle<T, N>> {
        for (idx, &free) in self.free_list.iter().enumerate() {
            if free {
                self.free_list[idx] = false;
                self.allocated += 1;
                return Some(PoolHandle { pool: self, idx });
            }
        }
        None
    }

    /// Tries to allocate with a constructor function
    pub fn alloc_with<F>(&mut self, f: F) -> Option<PoolSlot<T, N>>
    where
        F: FnOnce() -> T,
    {
        match self.alloc(f()) {
            Ok(slot) => Some(slot),
            Err(_) => None,
        }
    }

    /// Deallocates an object back to the pool
    fn dealloc(&mut self, idx: usize) {
        if idx < N && !self.free_list[idx] {
            unsafe {
                self.data[idx].assume_init_drop();
            }
            self.free_list[idx] = true;
            self.allocated -= 1;
        }
    }

    /// Returns the total capacity
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the number of allocated slots
    pub const fn allocated(&self) -> usize {
        self.allocated
    }

    /// Returns the number of free slots
    pub const fn available(&self) -> usize {
        N - self.allocated
    }

    /// Returns true if the pool is full
    pub const fn is_full(&self) -> bool {
        self.allocated == N
    }

    /// Returns true if the pool is empty
    pub const fn is_empty(&self) -> bool {
        self.allocated == 0
    }

    /// Clears the pool, dropping all allocated objects
    pub fn clear(&mut self) {
        for i in 0..N {
            if !self.free_list[i] {
                unsafe {
                    self.data[i].assume_init_drop();
                }
                self.free_list[i] = true;
            }
        }
        self.allocated = 0;
    }
}

impl<T, const N: usize> Drop for Pool<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Default for Pool<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle to an uninitialized pool slot
pub struct PoolHandle<'a, T, const N: usize> {
    pool: &'a mut Pool<T, N>,
    idx: usize,
}

impl<'a, T, const N: usize> PoolHandle<'a, T, N> {
    /// Stores a value in this pool slot
    pub fn store(self, value: T) -> PoolSlot<'a, T, N> {
        self.pool.data[self.idx] = MaybeUninit::new(value);
        PoolSlot {
            pool: self.pool as *mut _,
            idx: self.idx,
            _phantom: PhantomData,
        }
    }
}

/// Smart pointer to a pooled object
pub struct PoolSlot<'a, T, const N: usize> {
    pool: *mut Pool<T, N>,
    idx: usize,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T, const N: usize> PoolSlot<'a, T, N> {
    /// Gets a reference to the value
    pub fn get(&self) -> &T {
        unsafe { (*self.pool).data[self.idx].assume_init_ref() }
    }

    /// Gets a mutable reference to the value
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { (*self.pool).data[self.idx].assume_init_mut() }
    }

    /// Leaks the slot, preventing deallocation
    pub fn leak(self) -> &'a mut T {
        let ptr = unsafe { (*self.pool).data[self.idx].assume_init_mut() };
        core::mem::forget(self);
        ptr
    }
}

impl<'a, T, const N: usize> core::ops::Deref for PoolSlot<'a, T, N> {
    type Target = T;

    fn deref(&self) -> &T {
        self.get()
    }
}

impl<'a, T, const N: usize> core::ops::DerefMut for PoolSlot<'a, T, N> {
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<'a, T, const N: usize> Drop for PoolSlot<'a, T, N> {
    fn drop(&mut self) {
        unsafe {
            (*self.pool).dealloc(self.idx);
        }
    }
}

impl<'a, T: core::fmt::Debug, const N: usize> core::fmt::Debug for PoolSlot<'a, T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
