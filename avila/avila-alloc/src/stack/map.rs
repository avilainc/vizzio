//! StackMap - Fixed-capacity hash map on stack

use core::hash::{Hash, Hasher};
use core::mem::MaybeUninit;

/// Simple FNV-1a hasher for no_std environments
struct FnvHasher {
    state: u64,
}

impl FnvHasher {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    const fn new() -> Self {
        Self {
            state: Self::OFFSET_BASIS,
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

impl Hasher for FnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(Self::PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

/// Entry in the hash map
struct Entry<K, V> {
    key: K,
    value: V,
    occupied: bool,
}

/// Stack-allocated hash map with fixed capacity
pub struct StackMap<K, V, const N: usize> {
    entries: [MaybeUninit<Entry<K, V>>; N],
    len: usize,
}

impl<K, V, const N: usize> StackMap<K, V, N> {
    /// Creates a new empty map
    pub const fn new() -> Self {
        Self {
            entries: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the number of elements in the map
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the map is empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the map
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Computes hash for a key
    fn hash_key(key: &K) -> u64
    where
        K: Hash,
    {
        let mut hasher = FnvHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    /// Finds the index for a key using linear probing
    fn find_slot(&self, key: &K) -> Option<usize>
    where
        K: Hash + Eq,
    {
        let hash = Self::hash_key(key);
        let mut idx = (hash % N as u64) as usize;

        for _ in 0..N {
            unsafe {
                let entry_ptr = self.entries[idx].as_ptr();
                if (*entry_ptr).occupied && (*entry_ptr).key == *key {
                    return Some(idx);
                }
                if !(*entry_ptr).occupied {
                    return None;
                }
            }
            idx = (idx + 1) % N;
        }
        None
    }

    /// Finds an empty slot for insertion
    fn find_empty_slot(&self, key: &K) -> Option<usize>
    where
        K: Hash,
    {
        let hash = Self::hash_key(key);
        let mut idx = (hash % N as u64) as usize;

        for _ in 0..N {
            unsafe {
                let entry_ptr = self.entries[idx].as_ptr();
                if !(*entry_ptr).occupied {
                    return Some(idx);
                }
            }
            idx = (idx + 1) % N;
        }
        None
    }

    /// Inserts a key-value pair into the map
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, (K, V)>
    where
        K: Hash + Eq,
    {
        // Check if key exists
        if let Some(idx) = self.find_slot(&key) {
            unsafe {
                let entry = &mut *self.entries[idx].as_mut_ptr();
                let old_value = core::ptr::read(&entry.value);
                entry.value = value;
                return Ok(Some(old_value));
            }
        }

        // Find empty slot
        if let Some(idx) = self.find_empty_slot(&key) {
            self.entries[idx] = MaybeUninit::new(Entry {
                key,
                value,
                occupied: true,
            });
            self.len += 1;
            Ok(None)
        } else {
            Err((key, value))
        }
    }

    /// Gets a reference to the value associated with the key
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Hash + Eq,
    {
        self.find_slot(key).map(|idx| unsafe {
            &(*self.entries[idx].as_ptr()).value
        })
    }

    /// Gets a mutable reference to the value associated with the key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    where
        K: Hash + Eq,
    {
        self.find_slot(key).map(|idx| unsafe {
            &mut (*self.entries[idx].as_mut_ptr()).value
        })
    }

    /// Removes a key from the map, returning the value if the key was present
    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Hash + Eq,
    {
        if let Some(idx) = self.find_slot(key) {
            unsafe {
                let entry_ptr = self.entries[idx].as_mut_ptr();
                let value = core::ptr::read(&(*entry_ptr).value);
                (*entry_ptr).occupied = false;
                self.len -= 1;
                Some(value)
            }
        } else {
            None
        }
    }

    /// Returns `true` if the map contains the key
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Hash + Eq,
    {
        self.find_slot(key).is_some()
    }

    /// Clears the map, removing all entries
    pub fn clear(&mut self) {
        for i in 0..N {
            unsafe {
                let entry_ptr = self.entries[i].as_mut_ptr();
                if (*entry_ptr).occupied {
                    core::ptr::drop_in_place(&mut (*entry_ptr).key);
                    core::ptr::drop_in_place(&mut (*entry_ptr).value);
                    (*entry_ptr).occupied = false;
                }
            }
        }
        self.len = 0;
    }

    /// Returns an iterator over the entries
    pub fn iter(&self) -> MapIter<'_, K, V, N> {
        MapIter {
            map: self,
            pos: 0,
        }
    }
}

/// Iterator over map entries
pub struct MapIter<'a, K, V, const N: usize> {
    map: &'a StackMap<K, V, N>,
    pos: usize,
}

impl<'a, K, V, const N: usize> Iterator for MapIter<'a, K, V, N> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < N {
            unsafe {
                let entry_ptr = self.map.entries[self.pos].as_ptr();
                self.pos += 1;
                if (*entry_ptr).occupied {
                    return Some((&(*entry_ptr).key, &(*entry_ptr).value));
                }
            }
        }
        None
    }
}

impl<K, V, const N: usize> Drop for StackMap<K, V, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<K, V, const N: usize> Default for StackMap<K, V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, const N: usize> core::fmt::Debug for StackMap<K, V, N>
where
    K: core::fmt::Debug,
    V: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
