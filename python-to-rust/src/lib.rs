use pyo3::prelude::*;

// code snippet to append PYTHONPATH to sys.path
pub fn init_python_venv(py: Python) {
    // make sure to set PYTHONPATH environment first (from .cargo/config.toml or build.rs)
    let pythonpath = std::env::var("PYTHONPATH").unwrap_or_default();

    let sys = py.import("sys").expect("Failed to import sys module");
    let sys_path = sys.getattr("path").expect("Failed to get sys.path");
    sys_path
        .call_method1("append", (pythonpath,))
        .expect("Failed to append to sys.path");
}
