use pyo3::prelude::*;

pub fn init_python_venv(py: Python) {
    // append PYTHONPATH to sys.path
    let pythonpath = std::env::var("PYTHONPATH").unwrap_or_default();

    let sys = py.import("sys").expect("Failed to import sys module");
    let sys_path = sys.getattr("path").expect("Failed to get sys.path");
    sys_path
        .call_method1("append", (pythonpath,))
        .expect("Failed to append to sys.path");
}
