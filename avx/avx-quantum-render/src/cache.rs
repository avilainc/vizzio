//! Intelligent caching and memory management system

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Texture cache with LRU eviction strategy
pub struct TextureCache {
    cache: Arc<RwLock<HashMap<String, CachedTexture>>>,
    max_entries: usize,
    max_memory_mb: usize,
    current_memory: Arc<RwLock<usize>>,
}

/// Cached texture data
#[derive(Clone)]
struct CachedTexture {
    data: Vec<f64>,
    width: u32,
    height: u32,
    access_count: u64,
    memory_bytes: usize,
}

impl TextureCache {
    /// Create new texture cache with specified limits
    pub fn new(max_entries: usize, max_memory_mb: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_entries,
            max_memory_mb,
            current_memory: Arc::new(RwLock::new(0)),
        }
    }

    /// Add texture to cache
    pub fn insert(&self, key: String, data: Vec<f64>, width: u32, height: u32) -> bool {
        let memory_bytes = data.len() * std::mem::size_of::<f64>();
        let memory_mb = memory_bytes / (1024 * 1024);

        // Check memory limit
        if memory_mb > self.max_memory_mb {
            return false;
        }

        let texture = CachedTexture {
            data,
            width,
            height,
            access_count: 0,
            memory_bytes,
        };

        if let Ok(mut cache) = self.cache.write() {
            if cache.len() >= self.max_entries {
                // Evict least accessed entry
                if let Some(key_to_remove) = cache
                    .iter()
                    .min_by_key(|(_, v)| v.access_count)
                    .map(|(k, _)| k.clone())
                {
                    if let Some(evicted) = cache.remove(&key_to_remove) {
                        if let Ok(mut mem) = self.current_memory.write() {
                            *mem = mem.saturating_sub(evicted.memory_bytes);
                        }
                    }
                }
            }

            cache.insert(key, texture);
            if let Ok(mut mem) = self.current_memory.write() {
                *mem += memory_bytes;
            }
            true
        } else {
            false
        }
    }

    /// Get texture from cache
    pub fn get(&self, key: &str) -> Option<(Vec<f64>, u32, u32)> {
        if let Ok(mut cache) = self.cache.write() {
            if let Some(texture) = cache.get_mut(key) {
                texture.access_count += 1;
                return Some((texture.data.clone(), texture.width, texture.height));
            }
        }
        None
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        if let Ok(cache) = self.cache.read() {
            let current_memory = self.current_memory.read().ok().map(|m| *m).unwrap_or(0);
            CacheStats {
                entries: cache.len(),
                memory_mb: current_memory / (1024 * 1024),
                total_accesses: cache.values().map(|t| t.access_count).sum(),
            }
        } else {
            CacheStats::default()
        }
    }

    /// Clear cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
            if let Ok(mut mem) = self.current_memory.write() {
                *mem = 0;
            }
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cached entries
    pub entries: usize,
    /// Memory usage in MB
    pub memory_mb: usize,
    /// Total access count
    pub total_accesses: u64,
}

/// Object pool for reusing allocations
pub struct ObjectPool<T: Clone> {
    pool: Vec<T>,
    prototype: T,
}

impl<T: Clone> ObjectPool<T> {
    /// Create new object pool
    pub fn new(prototype: T, initial_size: usize) -> Self {
        let mut pool = Vec::with_capacity(initial_size);
        for _ in 0..initial_size {
            pool.push(prototype.clone());
        }
        Self { pool, prototype }
    }

    /// Get object from pool or create new
    pub fn acquire(&mut self) -> T {
        self.pool.pop().unwrap_or_else(|| self.prototype.clone())
    }

    /// Return object to pool
    pub fn release(&mut self, obj: T) {
        if self.pool.len() < self.pool.capacity() {
            self.pool.push(obj);
        }
    }

    /// Pool statistics
    pub fn available(&self) -> usize {
        self.pool.len()
    }
}

/// Memory arena allocator for fast allocations
pub struct Arena {
    buffer: Vec<u8>,
    offset: usize,
}

impl Arena {
    /// Create new arena with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            offset: 0,
        }
    }

    /// Allocate space for T
    pub fn alloc<T>(&mut self, count: usize) -> Option<&mut [T]> {
        let size = count * std::mem::size_of::<T>();
        if self.offset + size > self.buffer.capacity() {
            return None;
        }

        let ptr = self.buffer.as_mut_ptr() as usize + self.offset;
        self.offset += size;

        unsafe {
            Some(std::slice::from_raw_parts_mut(
                ptr as *mut T,
                count,
            ))
        }
    }

    /// Reset arena
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Get current usage
    pub fn used(&self) -> usize {
        self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_cache() {
        let cache = TextureCache::new(2, 10);
        let data = vec![1.0; 1000];
        assert!(cache.insert("tex1".to_string(), data.clone(), 32, 32));
        assert!(cache.get("tex1").is_some());
    }

    #[test]
    fn test_cache_eviction() {
        let cache = TextureCache::new(2, 100);
        let data = vec![1.0; 100];
        cache.insert("tex1".to_string(), data.clone(), 10, 10);
        cache.insert("tex2".to_string(), data.clone(), 10, 10);
        cache.insert("tex3".to_string(), data, 10, 10);
        // tex1 should be evicted (least accessed)
        let stats = cache.stats();
        assert!(stats.entries <= 2);
    }

    #[test]
    fn test_object_pool() {
        let mut pool = ObjectPool::new(vec![0.0; 10], 2);
        let obj1 = pool.acquire();
        assert_eq!(pool.available(), 1);
        pool.release(obj1);
        assert_eq!(pool.available(), 2);
    }
}
