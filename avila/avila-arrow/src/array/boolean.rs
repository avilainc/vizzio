//! Boolean array implementation

/// Boolean array
#[derive(Debug, Clone)]
pub struct BooleanArray {
    data: Vec<bool>,
}

impl BooleanArray {
    pub fn new(data: Vec<bool>) -> Self {
        Self { data }
    }

    pub fn value(&self, index: usize) -> bool {
        self.data[index]
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
