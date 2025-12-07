//! Scientific array types

use super::quaternion::Quaternion;
use super::complex::Complex64;

/// Array of quaternions
#[derive(Debug, Clone)]
pub struct QuaternionArray {
    data: Vec<Quaternion>,
}

impl QuaternionArray {
    pub fn new(data: Vec<Quaternion>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Quaternion> {
        self.data.get(index)
    }
}

/// Array of complex numbers
#[derive(Debug, Clone)]
pub struct ComplexArray {
    data: Vec<Complex64>,
}

impl ComplexArray {
    pub fn new(data: Vec<Complex64>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
