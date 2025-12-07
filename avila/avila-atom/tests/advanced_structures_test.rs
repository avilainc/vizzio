//! Tests for advanced data structures

#[cfg(test)]
mod tests {
    use avila_atom::skiplist::SkipList;
    use avila_atom::trie::Trie;
    use avila_atom::bloom::BloomFilter;
    use avila_atom::dsu::DisjointSet;
    use avila_atom::fenwick::FenwickTree;

    #[test]
    fn test_skiplist_creation() {
        let list: SkipList<i32, String> = SkipList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_trie_operations() {
        let mut trie = Trie::new();

        trie.insert("hello", 1);
        trie.insert("world", 2);
        trie.insert("help", 3);

        assert_eq!(trie.get("hello"), Some(&1));
        assert_eq!(trie.get("world"), Some(&2));
        assert_eq!(trie.get("help"), Some(&3));
        assert_eq!(trie.get("hell"), None);

        let results = trie.keys_with_prefix("hel");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_bloom_filter() {
        let mut filter = BloomFilter::new(100, 0.01);

        filter.insert(&"test");
        filter.insert(&"rust");
        filter.insert(&"avila");

        assert!(filter.contains(&"test"));
        assert!(filter.contains(&"rust"));
        assert!(filter.contains(&"avila"));
        assert!(!filter.contains(&"python"));
    }

    #[test]
    fn test_disjoint_set() {
        let mut dsu = DisjointSet::new(10);

        assert_eq!(dsu.count(), 10);

        dsu.union(0, 1);
        dsu.union(2, 3);
        dsu.union(1, 3);

        assert!(dsu.connected(0, 2));
        assert!(dsu.connected(1, 3));
        assert!(!dsu.connected(0, 4));
    }

    #[test]
    fn test_fenwick_tree() {
        let arr = vec![1, 2, 3, 4, 5];
        let ft = FenwickTree::from_slice(&arr);

        assert_eq!(ft.prefix_sum(0), 1);
        assert_eq!(ft.prefix_sum(2), 6);
        assert_eq!(ft.range_sum(1, 3), 9);
    }
}
