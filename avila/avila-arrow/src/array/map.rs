//! Map array implementation

/// Map array
#[derive(Debug, Clone)]
pub struct MapArray {
    // Placeholder implementation
    entries: Vec<(String, String)>,
}

impl MapArray {
    pub fn new(entries: Vec<(String, String)>) -> Self {
        Self { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
