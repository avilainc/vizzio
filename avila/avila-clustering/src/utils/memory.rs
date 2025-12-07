//! Gerenciamento de memória para grandes datasets

use std::collections::VecDeque;

pub struct MemoryManager {
    max_memory_mb: usize,
    current_usage_mb: usize,
}

impl MemoryManager {
    pub fn new(max_memory_mb: usize) -> Self {
        Self {
            max_memory_mb,
            current_usage_mb: 0,
        }
    }

    pub fn can_allocate(&self, size_mb: usize) -> bool {
        self.current_usage_mb + size_mb <= self.max_memory_mb
    }

    pub fn allocate(&mut self, size_mb: usize) -> Result<(), String> {
        if !self.can_allocate(size_mb) {
            return Err(format!(
                "Memória insuficiente: {} MB necessários, {} MB disponíveis",
                size_mb,
                self.max_memory_mb - self.current_usage_mb
            ));
        }
        self.current_usage_mb += size_mb;
        Ok(())
    }

    pub fn deallocate(&mut self, size_mb: usize) {
        self.current_usage_mb = self.current_usage_mb.saturating_sub(size_mb);
    }

    pub fn get_usage(&self) -> usize {
        self.current_usage_mb
    }
}

pub struct ChunkedDataIterator<T> {
    data: VecDeque<Vec<T>>,
    chunk_size: usize,
}

impl<T: Clone> ChunkedDataIterator<T> {
    pub fn new(data: Vec<T>, chunk_size: usize) -> Self {
        let mut chunks = VecDeque::new();
        for chunk in data.chunks(chunk_size) {
            chunks.push_back(chunk.to_vec());
        }
        Self {
            data: chunks,
            chunk_size,
        }
    }
}

impl<T> Iterator for ChunkedDataIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front()
    }
}
