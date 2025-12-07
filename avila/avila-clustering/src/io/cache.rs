//! Cache inteligente de resultados

use std::collections::HashMap;

pub struct ResultCache<T> {
    cache: HashMap<String, T>,
    max_size: usize,
}

impl<T: Clone> ResultCache<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.cache.get(key)
    }

    pub fn insert(&mut self, key: String, value: T) {
        if self.cache.len() >= self.max_size {
            // Simple eviction: remove first item
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }
        self.cache.insert(key, value);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
