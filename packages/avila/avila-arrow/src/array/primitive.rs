//! Primitive arrays: Int, UInt, Float arrays

use crate::error::Result;

/// Primitive array trait
pub trait PrimitiveArray {
    type Native;

    fn value(&self, index: usize) -> Self::Native;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Int32 array
#[derive(Debug, Clone)]
pub struct Int32Array {
    data: Vec<i32>,
}

/// Float64 array
#[derive(Debug, Clone)]
pub struct Float64Array {
    data: Vec<f64>,
}
