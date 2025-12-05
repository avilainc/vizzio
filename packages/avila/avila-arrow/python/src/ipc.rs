//! Python IPC bindings

use pyo3::prelude::*;

#[pyclass]
pub struct PyIpcReader {
    // Placeholder
}

#[pymethods]
impl PyIpcReader {
    #[new]
    pub fn new(_path: String) -> Self {
        Self {}
    }
}
