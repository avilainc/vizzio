//! Union array implementation

/// Union mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnionMode {
    Sparse,
    Dense,
}

/// Union array
#[derive(Debug, Clone)]
pub struct UnionArray {
    mode: UnionMode,
    // Placeholder implementation
}

impl UnionArray {
    pub fn mode(&self) -> UnionMode {
        self.mode
    }
}
