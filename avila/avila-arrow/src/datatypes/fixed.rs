//! Fixed-size data types: FixedSizeBinary, FixedSizeList

use std::sync::Arc;
use super::nested::Field;

/// Fixed-size binary type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedSizeBinaryType {
    pub byte_width: i32,
}

/// Fixed-size list type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedSizeListType {
    pub field: Arc<Field>,
    pub list_size: i32,
}
