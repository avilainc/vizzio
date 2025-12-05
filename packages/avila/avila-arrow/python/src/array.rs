//! Python array bindings

use pyo3::prelude::*;

#[pyclass]
pub struct PyArray {
    // Placeholder
}

#[pymethods]
impl PyArray {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn len(&self) -> usize {
        0
    }
}
