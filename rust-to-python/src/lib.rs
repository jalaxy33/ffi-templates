use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};

/// Formats the sum of two numbers as string.
#[pyfunction]
#[gen_stub_pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Complex data processing function
#[pyfunction]
#[gen_stub_pyfunction]
pub fn process_numbers(numbers: Vec<f64>) -> PyResult<Vec<f64>> {
    let result: Vec<f64> = numbers.iter().map(|&x| x * 2.0 + 1.0).collect();
    Ok(result)
}

/// Example using struct
#[pyclass]
#[gen_stub_pyclass]
pub struct Calculator {
    #[pyo3(get, set)]
    value: f64,
}

#[pymethods]
#[gen_stub_pymethods]
impl Calculator {
    #[new]
    pub fn new(value: f64) -> Self {
        Calculator { value }
    }

    pub fn add(&mut self, other: f64) -> f64 {
        self.value += other;
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Calculator(value={})", self.value))
    }
}

/// Export rust library as Python module.
#[pymodule]
fn rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(process_numbers, m)?)?;
    m.add_class::<Calculator>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);


