//! String arrays: Utf8, Binary

/// UTF-8 string array
#[derive(Debug, Clone)]
pub struct StringArray {
    data: Vec<String>,
}

impl StringArray {
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }

    pub fn value(&self, index: usize) -> &str {
        &self.data[index]
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Binary array
#[derive(Debug, Clone)]
pub struct BinaryArray {
    data: Vec<Vec<u8>>,
}

impl BinaryArray {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        Self { data }
    }

    pub fn value(&self, index: usize) -> &[u8] {
        &self.data[index]
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
