//! Dictionary batch support for IPC

use crate::error::Result;

/// Dictionary batch
#[derive(Debug, Clone)]
pub struct DictionaryBatch {
    pub id: i64,
    pub data: Vec<u8>,
    pub is_delta: bool,
}

impl DictionaryBatch {
    pub fn new(id: i64, data: Vec<u8>) -> Self {
        Self {
            id,
            data,
            is_delta: false,
        }
    }

    pub fn with_delta(mut self, is_delta: bool) -> Self {
        self.is_delta = is_delta;
        self
    }
}
