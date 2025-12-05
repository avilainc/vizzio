//! List arrays: List, LargeList, FixedSizeList

use std::sync::Arc;

/// List array
#[derive(Debug, Clone)]
pub struct ListArray {
    // Placeholder implementation
    offsets: Vec<i32>,
}

/// Large list array
#[derive(Debug, Clone)]
pub struct LargeListArray {
    // Placeholder implementation
    offsets: Vec<i64>,
}

/// Fixed-size list array
#[derive(Debug, Clone)]
pub struct FixedSizeListArray {
    // Placeholder implementation
    list_size: i32,
}
