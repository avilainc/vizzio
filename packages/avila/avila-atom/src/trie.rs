//! Trie and Radix Tree implementations
//!
//! **Complexity**: O(m) where m is key length
//! **Use cases**: Autocomplete, prefix search, routing tables

use crate::DynamicArray;
use crate::AssociativeArray;

/// Standard Trie (prefix tree)
///
/// **Performance**:
/// - Insert: O(m) where m = key length
/// - Search: O(m)
/// - Prefix search: O(m + k) where k = results
///
/// **Use cases**:
/// - Autocomplete
/// - Spell checking
/// - IP routing
pub struct Trie<V> {
    root: TrieNode<V>,
    size: usize,
}

struct TrieNode<V> {
    children: AssociativeArray<char, Box<TrieNode<V>>>,
    value: Option<V>,
    is_end: bool,
}

impl<V> Trie<V> {
    /// Create new empty trie
    pub fn new() -> Self {
        Self {
            root: TrieNode {
                children: AssociativeArray::new(),
                value: None,
                is_end: false,
            },
            size: 0,
        }
    }

    /// Insert key-value pair
    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        let mut node = &mut self.root;

        for ch in key.chars() {
            node = node.children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode {
                    children: AssociativeArray::new(),
                    value: None,
                    is_end: false,
                }));
        }

        let old_value = node.value.replace(value);
        if old_value.is_none() {
            self.size += 1;
            node.is_end = true;
        }
        old_value
    }

    /// Search for exact key
    pub fn get(&self, key: &str) -> Option<&V> {
        let mut node = &self.root;

        for ch in key.chars() {
            node = node.children.get(&ch)?.as_ref();
        }

        if node.is_end {
            node.value.as_ref()
        } else {
            None
        }
    }

    /// Check if key exists
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Find all keys with given prefix
    pub fn keys_with_prefix(&self, prefix: &str) -> DynamicArray<String> {
        let mut results = DynamicArray::new();
        let mut node = &self.root;

        // Navigate to prefix node
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(n) => node = n.as_ref(),
                None => return results,
            }
        }

        // Collect all keys from this point
        self.collect_keys(node, &mut String::from(prefix), &mut results);
        results
    }

    fn collect_keys(&self, node: &TrieNode<V>, current: &mut String, results: &mut DynamicArray<String>) {
        if node.is_end {
            results.push(current.clone());
        }

        for (ch, child) in &node.children {
            current.push(*ch);
            self.collect_keys(child.as_ref(), current, results);
            current.pop();
        }
    }

    /// Get number of keys
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

/// Radix Tree (Compressed Trie)
///
/// **Performance**: Same as Trie but more space-efficient
/// **Optimization**: Path compression - merge single-child chains
pub struct RadixTree<V> {
    root: RadixNode<V>,
    size: usize,
}

struct RadixNode<V> {
    children: AssociativeArray<String, Box<RadixNode<V>>>,
    value: Option<V>,
}

impl<V> RadixTree<V> {
    /// Create new radix tree
    pub fn new() -> Self {
        Self {
            root: RadixNode {
                children: AssociativeArray::new(),
                value: None,
            },
            size: 0,
        }
    }

    /// Insert key-value pair with path compression
    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        if key.is_empty() {
            let old = self.root.value.replace(value);
            if old.is_none() {
                self.size += 1;
            }
            return old;
        }

        let result = Self::insert_helper_static(&mut self.root, key, value, &mut self.size);
        result
    }

    fn insert_helper_static(node: &mut RadixNode<V>, key: &str, value: V, size: &mut usize) -> Option<V> {
        // Find matching child edge
        for (edge, child) in node.children.iter_mut() {
            let common = common_prefix(edge, key);

            if common > 0 {
                if common == edge.len() && common == key.len() {
                    // Exact match
                    let old = child.value.replace(value);
                    if old.is_none() {
                        self.size += 1;
                    }
                    return old;
                } else if common == edge.len() {
                    // Edge fully matched, continue with remainder
                    return self.insert_helper(child, &key[common..], value);
                } else {
                    // Split edge
                    let split_edge = edge[..common].to_string();
                    let old_edge_suffix = edge[common..].to_string();
                    let new_key_suffix = &key[common..];

                    // Create new intermediate node
                    let mut intermediate = Box::new(RadixNode {
                        children: AssociativeArray::new(),
                        value: None,
                    });

                    // Move old child under new suffix
                    let old_child = core::mem::replace(
                        child.as_mut(),
                        RadixNode {
                            children: AssociativeArray::new(),
                            value: None,
                        },
                    );
                    intermediate.children.insert(old_edge_suffix, Box::new(old_child));

                    // Add new key
                    if new_key_suffix.is_empty() {
                        intermediate.value = Some(value);
                        self.size += 1;
                    } else {
                        let mut new_node = Box::new(RadixNode {
                            children: AssociativeArray::new(),
                            value: Some(value),
                        });
                        intermediate.children.insert(new_key_suffix.to_string(), new_node);
                        self.size += 1;
                    }

                    // Replace edge
                    let edge_key = edge.clone();
                    node.children.remove(&edge_key);
                    node.children.insert(split_edge, intermediate);

                    return None;
                }
            }
        }

        // No matching edge, create new
        let new_node = Box::new(RadixNode {
            children: AssociativeArray::new(),
            value: Some(value),
        });
        node.children.insert(key.to_string(), new_node);
        self.size += 1;
        None
    }

    /// Get value by key
    pub fn get(&self, key: &str) -> Option<&V> {
        if key.is_empty() {
            return self.root.value.as_ref();
        }

        self.get_helper(&self.root, key)
    }

    fn get_helper(&self, node: &RadixNode<V>, key: &str) -> Option<&V> {
        for (edge, child) in &node.children {
            if key.starts_with(edge.as_str()) {
                if key.len() == edge.len() {
                    return child.value.as_ref();
                } else {
                    return self.get_helper(child, &key[edge.len()..]);
                }
            }
        }
        None
    }

    /// Check if key exists
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Get all keys with given prefix
    pub fn keys_with_prefix(&self, prefix: &str) -> DynamicArray<String> {
        let mut results = DynamicArray::new();

        if prefix.is_empty() {
            self.collect_all_keys(&self.root, &mut String::new(), &mut results);
            return results;
        }

        // Navigate to prefix
        let node = self.find_prefix_node(&self.root, prefix);
        if let Some((n, matched_prefix)) = node {
            self.collect_all_keys(n, &mut matched_prefix.to_string(), &mut results);
        }

        results
    }

    fn find_prefix_node<'a>(&'a self, node: &'a RadixNode<V>, prefix: &str) -> Option<(&'a RadixNode<V>, String)> {
        if prefix.is_empty() {
            return Some((node, String::new()));
        }

        for (edge, child) in &node.children {
            let common = common_prefix(edge, prefix);

            if common > 0 {
                if common == prefix.len() {
                    // Prefix fully matched
                    let mut matched = String::new();
                    matched.push_str(&edge[..common]);
                    return Some((child.as_ref(), matched));
                } else if common == edge.len() {
                    // Edge fully matched, continue
                    if let Some((n, mut m)) = self.find_prefix_node(child, &prefix[common..]) {
                        let mut matched = edge.clone();
                        matched.push_str(&m);
                        return Some((n, matched));
                    }
                }
            }
        }

        None
    }

    fn collect_all_keys(&self, node: &RadixNode<V>, current: &mut String, results: &mut DynamicArray<String>) {
        if node.value.is_some() {
            results.push(current.clone());
        }

        for (edge, child) in &node.children {
            let len = current.len();
            current.push_str(edge);
            self.collect_all_keys(child, current, results);
            current.truncate(len);
        }
    }

    /// Number of keys
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.root.children.clear();
        self.root.value = None;
        self.size = 0;
    }
}

/// Find common prefix length between two strings
fn common_prefix(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .take_while(|(ca, cb)| ca == cb)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_basic() {
        let mut trie = Trie::new();
        trie.insert("hello", 1);
        trie.insert("world", 2);
        trie.insert("help", 3);

        assert_eq!(trie.get("hello"), Some(&1));
        assert_eq!(trie.get("world"), Some(&2));
        assert_eq!(trie.get("help"), Some(&3));
        assert_eq!(trie.get("hell"), None);
        assert_eq!(trie.len(), 3);
    }

    #[test]
    fn test_trie_prefix_search() {
        let mut trie = Trie::new();
        trie.insert("hello", 1);
        trie.insert("help", 2);
        trie.insert("world", 3);

        let results = trie.keys_with_prefix("hel");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"hello".to_string()));
        assert!(results.contains(&"help".to_string()));
    }

    #[test]
    fn test_radix_tree() {
        let mut tree = RadixTree::new();
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_radix_insert_get() {
        let mut tree = RadixTree::new();

        tree.insert("romane", 1);
        tree.insert("romanus", 2);
        tree.insert("romulus", 3);
        tree.insert("rubens", 4);
        tree.insert("ruber", 5);
        tree.insert("rubicon", 6);
        tree.insert("rubicundus", 7);

        assert_eq!(tree.get("romane"), Some(&1));
        assert_eq!(tree.get("romanus"), Some(&2));
        assert_eq!(tree.get("rubens"), Some(&4));
        assert_eq!(tree.get("rubicon"), Some(&6));
        assert_eq!(tree.len(), 7);
    }

    #[test]
    fn test_radix_contains() {
        let mut tree = RadixTree::new();

        tree.insert("test", 100);
        tree.insert("testing", 200);

        assert!(tree.contains("test"));
        assert!(tree.contains("testing"));
        assert!(!tree.contains("tes"));
        assert!(!tree.contains("tests"));
    }

    #[test]
    fn test_radix_prefix_search() {
        let mut tree = RadixTree::new();

        tree.insert("romane", 1);
        tree.insert("romanus", 2);
        tree.insert("romulus", 3);
        tree.insert("rubens", 4);

        let results = tree.keys_with_prefix("rom");
        assert_eq!(results.len(), 3);

        let results = tree.keys_with_prefix("rub");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_radix_clear() {
        let mut tree = RadixTree::new();

        tree.insert("a", 1);
        tree.insert("b", 2);
        tree.insert("c", 3);

        assert_eq!(tree.len(), 3);
        tree.clear();
        assert_eq!(tree.len(), 0);
        assert!(tree.is_empty());
    }
}
