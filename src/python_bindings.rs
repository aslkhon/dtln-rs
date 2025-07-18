// src/python_bindings.rs
use crate::dtln_processor::{DtlnDeferredProcessor, DtlnProcessEngine};
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[pyclass]
pub struct DtlnProcessor {
    processor: Arc<Mutex<DtlnDeferredProcessor>>,
}

#[pymethods]
impl DtlnProcessor {
    #[new]
    fn new() -> PyResult<Self> {
        let processor = DtlnDeferredProcessor::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create processor: {}",
                e
            ))
        })?;

        Ok(DtlnProcessor {
            processor: Arc::new(Mutex::new(processor)),
        })
    }

    fn denoise(&self, samples: Vec<f32>) -> PyResult<(Vec<f32>, bool)> {
        let mut processor = self.processor.lock().unwrap();

        let result = processor.denoise(&samples).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Denoise failed: {}", e))
        })?;

        Ok((result.samples, result.processor_starved))
    }

    fn stop(&self) {
        let mut processor = self.processor.lock().unwrap();
        processor.stop();
    }
}

#[pyfunction]
fn create_processor() -> PyResult<DtlnProcessor> {
    DtlnProcessor::new()
}

#[pymodule]
fn dtln_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DtlnProcessor>()?;
    m.add_function(wrap_pyfunction!(create_processor, m)?)?;
    Ok(())
}
