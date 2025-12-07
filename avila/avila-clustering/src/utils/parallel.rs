//! Helpers para paralelização

use std::sync::{Arc, Mutex};

pub struct ParallelExecutor;

impl ParallelExecutor {
    pub fn parallel_map<T, F>(data: Vec<T>, f: F) -> Vec<T>
    where
        T: Send + 'static,
        F: Fn(T) -> T + Send + Sync + 'static,
    {
        // Implementação simplificada - sequencial
        data.into_iter().map(f).collect()
    }
}

pub struct ThreadPool {
    size: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f();
    }
}
