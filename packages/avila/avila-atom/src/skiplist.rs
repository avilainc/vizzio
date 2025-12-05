//! Skip List - Probabilistic balanced search structure
//!
//! **Complexity**: O(log n) average for insert, delete, search
//! **Space**: O(n) with probabilistic extra pointers
//! **Use case**: Alternative to balanced trees, simpler implementation

use core::cmp::Ordering;
use core::mem;

const MAX_LEVEL: usize = 16;

/// Simple PRNG state for skip list
static mut RNG_STATE: u64 = 0x853c49e6748fea9b;

/// Node in the skip list
struct Node<K, V> {
    key: K,
    value: V,
    forward: [Option<*mut Node<K, V>>; MAX_LEVEL],
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            forward: [None; MAX_LEVEL],
        }
    }
}

/// Skip list data structure
///
/// **Performance**:
/// - Insert: O(log n) average
/// - Delete: O(log n) average
/// - Search: O(log n) average
///
/// **Advantages**:
/// - Simpler than balanced trees
/// - Better for concurrent implementations
/// - Probabilistically balanced
pub struct SkipList<K: Ord, V> {
    head: *mut Node<K, V>,
    level: usize,
    len: usize,
}

impl<K: Ord + Default, V: Default> SkipList<K, V> {
    /// Create new empty skip list
    pub fn new() -> Self {
        let head = Box::into_raw(Box::new(Node::new(K::default(), V::default())));
        Self {
            head,
            level: 0,
            len: 0,
        }
    }

    /// Insert key-value pair
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut update = [core::ptr::null_mut(); MAX_LEVEL];
        let mut current = self.head;

        // Find position and track update pointers
        for i in (0..=self.level).rev() {
            unsafe {
                while let Some(next) = (*current).forward[i] {
                    match (*next).key.cmp(&key) {
                        Ordering::Less => current = next,
                        _ => break,
                    }
                }
            }
            update[i] = current;
        }

        // Move to next node
        unsafe {
            if let Some(next) = (*current).forward[0] {
                if (*next).key == key {
                    // Update existing value
                    let old_value = mem::replace(&mut (*next).value, value);
                    return Some(old_value);
                }
            }
        }

        // Insert new node
        let new_level = self.random_level();
        if new_level > self.level {
            for i in (self.level + 1)..=new_level {
                update[i] = self.head;
            }
            self.level = new_level;
        }

        let new_node = Box::into_raw(Box::new(Node::new(key, value)));

        unsafe {
            for i in 0..=new_level {
                (*new_node).forward[i] = (*update[i]).forward[i];
                (*update[i]).forward[i] = Some(new_node);
            }
        }

        self.len += 1;
        None
    }

    /// Get value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut current = self.head;

        unsafe {
            for i in (0..=self.level).rev() {
                while let Some(next) = (*current).forward[i] {
                    match (*next).key.cmp(key) {
                        Ordering::Less => current = next,
                        Ordering::Equal => return Some(&(*next).value),
                        Ordering::Greater => break,
                    }
                }
            }
        }

        None
    }

    /// Get mutable value by key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut current = self.head;

        unsafe {
            for i in (0..=self.level).rev() {
                while let Some(next) = (*current).forward[i] {
                    match (*next).key.cmp(key) {
                        Ordering::Less => current = next,
                        Ordering::Equal => return Some(&mut (*next).value),
                        Ordering::Greater => break,
                    }
                }
            }
        }

        None
    }

    /// Remove key-value pair
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut update = [core::ptr::null_mut(); MAX_LEVEL];
        let mut current = self.head;

        // Find position
        for i in (0..=self.level).rev() {
            unsafe {
                while let Some(next) = (*current).forward[i] {
                    if (*next).key < *key {
                        current = next;
                    } else {
                        break;
                    }
                }
            }
            update[i] = current;
        }

        unsafe {
            if let Some(target) = (*current).forward[0] {
                if (*target).key == *key {
                    // Remove node
                    for i in 0..=self.level {
                        if let Some(next) = (*update[i]).forward[i] {
                            if next == target {
                                (*update[i]).forward[i] = (*target).forward[i];
                            }
                        }
                    }

                    // Update level
                    while self.level > 0 && (*self.head).forward[self.level].is_none() {
                        self.level -= 1;
                    }

                    self.len -= 1;
                    let node = Box::from_raw(target);
                    return Some(node.value);
                }
            }
        }

        None
    }

    /// Check if key exists
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Get number of elements
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clear all elements
    pub fn clear(&mut self) {
        unsafe {
            let mut current = (*self.head).forward[0];
            while let Some(node) = current {
                current = (*node).forward[0];
                drop(Box::from_raw(node));
            }

            for i in 0..MAX_LEVEL {
                (*self.head).forward[i] = None;
            }
        }

        self.level = 0;
        self.len = 0;
    }

    /// Generate random level for new node using simple PRNG
    fn random_level(&self) -> usize {
        let mut level = 0;
        let mut rng = unsafe {
            RNG_STATE = RNG_STATE.wrapping_mul(6364136223846793005).wrapping_add(1);
            RNG_STATE
        };

        while level < MAX_LEVEL - 1 && (rng & 1) == 0 {
            level += 1;
            rng >>= 1;
        }

        level
    }
}

impl<K: Ord, V> Drop for SkipList<K, V> {
    fn drop(&mut self) {
        // Manual cleanup without calling clear() which requires Default
        unsafe {
            for level in 0..MAX_LEVEL {
                let mut current = self.head;
                while !current.is_null() {
                    let next = (*current).next[level];
                    if level == 0 {
                        // Only drop once at level 0
                        let _ = Box::from_raw(current);
                    }
                    current = next;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skiplist_basic() {
        let mut list = SkipList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_skiplist_insert_get() {
        let mut list = SkipList::new();

        list.insert(5, "five");
        list.insert(3, "three");
        list.insert(7, "seven");
        list.insert(1, "one");

        assert_eq!(list.len(), 4);
        assert_eq!(list.get(&3), Some(&"three"));
        assert_eq!(list.get(&5), Some(&"five"));
        assert_eq!(list.get(&7), Some(&"seven"));
        assert_eq!(list.get(&1), Some(&"one"));
        assert_eq!(list.get(&10), None);
    }

    #[test]
    fn test_skiplist_update() {
        let mut list = SkipList::new();

        list.insert(5, "five");
        assert_eq!(list.get(&5), Some(&"five"));

        let old = list.insert(5, "FIVE");
        assert_eq!(old, Some("five"));
        assert_eq!(list.get(&5), Some(&"FIVE"));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_skiplist_remove() {
        let mut list = SkipList::new();

        list.insert(1, "one");
        list.insert(2, "two");
        list.insert(3, "three");

        assert_eq!(list.remove(&2), Some("two"));
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(&2), None);
        assert_eq!(list.get(&1), Some(&"one"));
        assert_eq!(list.get(&3), Some(&"three"));
    }

    #[test]
    fn test_skiplist_contains() {
        let mut list = SkipList::new();

        list.insert(10, 100);
        list.insert(20, 200);

        assert!(list.contains(&10));
        assert!(list.contains(&20));
        assert!(!list.contains(&15));
    }

    #[test]
    fn test_skiplist_clear() {
        let mut list = SkipList::new();

        for i in 0..100 {
            list.insert(i, i * 2);
        }

        assert_eq!(list.len(), 100);
        list.clear();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }
}
