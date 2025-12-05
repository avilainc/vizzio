// Unit tests for cache system
#[cfg(test)]
mod cache_tests {
    use crate::cache::*;
    use std::time::Duration;

    #[test]
    fn test_memory_cache() {
        let mut cache = memory::MemoryCache::new(10);
        let key = "test_key";
        let value = b"test_value".to_vec();

        assert!(cache.set(key, value.clone(), Duration::from_secs(60)).is_ok());
        assert_eq!(cache.get(key).unwrap(), Some(value));
    }

    #[test]
    fn test_cache_expiration() {
        let mut cache = memory::MemoryCache::new(10);
        let key = "test_key";
        let value = b"test_value".to_vec();

        cache.set(key, value, Duration::from_millis(1)).unwrap();
        std::thread::sleep(Duration::from_millis(10));

        assert!(cache.get(key).unwrap().is_none());
    }
}
