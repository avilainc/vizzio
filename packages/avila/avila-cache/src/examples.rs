//! Usage examples for the cache library
//!
//! This module contains commented-out examples showing how to use the cache.

#[cfg(test)]
mod examples {
    use crate::*;

    #[test]
    fn example_basic_cache() {
        // Create a simple cache
        let mut cache = DistributedCache::new();

        // Insert values
        cache.insert("user:1", "Alice");
        cache.insert("user:2", "Bob");

        // Get values
        assert_eq!(cache.get(&"user:1"), Some(&"Alice"));

        // Remove values
        cache.remove(&"user:1");
        assert_eq!(cache.get(&"user:1"), None);
    }

    #[test]
    fn example_lru_cache() {
        use crate::builder::CacheBuilder;

        // Create LRU cache with capacity of 3
        let mut cache = CacheBuilder::new()
            .max_capacity(3)
            .with_lru()
            .unwrap();

        // Insert 3 items
        cache.insert(1, "one");
        cache.insert(2, "two");
        cache.insert(3, "three");

        // Access item 1 (makes it most recently used)
        cache.get(&1);

        // Insert 4th item - evicts least recently used (2)
        cache.insert(4, "four");

        assert_eq!(cache.get(&2), None); // Evicted
        assert_eq!(cache.get(&1), Some(&"one")); // Still there

        // Check statistics
        let stats = cache.stats();
        println!("Hit rate: {:.2}%", stats.hit_rate() * 100.0);
        println!("Evictions: {}", stats.evictions);
    }

    #[test]
    fn example_lfu_cache() {
        use crate::builder::CacheBuilder;

        // Create LFU cache
        let mut cache = CacheBuilder::new()
            .max_capacity(2)
            .with_lfu()
            .unwrap();

        cache.insert(1, "one");
        cache.insert(2, "two");

        // Access item 1 multiple times
        cache.get(&1);
        cache.get(&1);
        cache.get(&1);

        // Access item 2 once
        cache.get(&2);

        // Insert new item - evicts least frequently used (2)
        cache.insert(3, "three");

        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), None); // Evicted (lower frequency)
    }

    #[test]
    fn example_fifo_cache() {
        use crate::builder::CacheBuilder;

        // Create FIFO cache
        let mut cache = CacheBuilder::new()
            .max_capacity(2)
            .with_fifo()
            .unwrap();

        cache.insert(1, "first");
        cache.insert(2, "second");

        // Insert third - evicts first one inserted
        cache.insert(3, "third");

        assert_eq!(cache.get(&1), None); // First in, first out
        assert_eq!(cache.get(&2), Some(&"second"));
    }

    #[test]
    fn example_sharded_cache() {
        use crate::ShardedCache;

        // Create cache with 4 shards
        let mut cache = ShardedCache::new(4);

        // Insert across shards
        for i in 0..100 {
            cache.insert(i, i * 2);
        }

        // Retrieve values
        assert_eq!(cache.get(&42), Some(&84));

        // Total size across all shards
        assert_eq!(cache.total_len(), 100);
    }

    #[test]
    fn example_cache_with_config() {
        // Create cache with configuration
        let config = CacheConfig::new()
            .with_capacity(1000)
            .with_stats(true);

        let cache = DistributedCache::<String, Vec<u8>>::with_config(config).unwrap();

        assert!(cache.config().is_some());
        assert!(cache.get_stats().is_some());
    }

    #[test]
    fn example_iterator_usage() {
        let mut cache = DistributedCache::new();

        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.insert(3, "c");

        // Iterate over keys
        let keys: Vec<_> = cache.keys().copied().collect();
        assert_eq!(keys, vec![1, 2, 3]);

        // Iterate over values
        let values: Vec<_> = cache.values().copied().collect();
        assert_eq!(values, vec!["a", "b", "c"]);

        // Iterate over pairs
        for (key, value) in cache.iter() {
            println!("{}: {}", key, value);
        }
    }

    #[test]
    fn example_statistics() {
        use crate::builder::CacheBuilder;

        let mut cache = CacheBuilder::new()
            .max_capacity(10)
            .with_lru()
            .unwrap();

        // Perform operations
        cache.insert(1, "value");
        cache.get(&1); // hit
        cache.get(&2); // miss
        cache.get(&1); // hit

        // Check statistics
        let stats = cache.stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.insertions, 1);

        println!("Hit rate: {:.2}%", stats.hit_rate() * 100.0);
        println!("Miss rate: {:.2}%", stats.miss_rate() * 100.0);
    }
}
