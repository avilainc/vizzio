//! Python RecordBatch bindings

use pyo3::prelude::*;

#[pyclass]
pub struct PyRecordBatch {
    // Placeholder
}

#[pymethods]
impl PyRecordBatch {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn num_rows(&self) -> usize {
        0
    }

    pub fn num_columns(&self) -> usize {
        0
    }
}
