//! Struct array implementation

use std::sync::Arc;

/// Struct array
#[derive(Debug, Clone)]
pub struct StructArray {
    // Placeholder implementation
    fields: Vec<String>,
}

impl StructArray {
    pub fn new(fields: Vec<String>) -> Self {
        Self { fields }
    }

    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }
}
