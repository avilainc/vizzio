//! Python bindings for avila-arrow (PyO3)

use pyo3::prelude::*;

pub mod array;
pub mod record_batch;
pub mod ipc;

#[pymodule]
fn avila_arrow(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<array::PyArray>()?;
    m.add_class::<record_batch::PyRecordBatch>()?;
    Ok(())
}
