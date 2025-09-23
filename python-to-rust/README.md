# Python to Rust Templates

A template repository for calling python functions from Rust using [PyO3](https://pyo3.rs/) and [Maturin](https://maturin.rs/).


**Hint**: You can initialize a new rust-python interop project with the following commands:

```bash
# install maturin if you haven't already
uv tool install maturin  # or pip install maturin

# create a new project with pyo3 bindings
uv tool run maturin init <your_project_name> --bindings pyo3  # or simply `maturin init <your_project_name> --bindings pyo3`
```

## Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [uv toolchain](https://docs.astral.sh/uv/getting-started/installation/): for managing Python versions and virtual environments


## Toolchain

- [maturin](https://maturin.rs/): for building and publishing Rust-based Python packages. Install it via `uv tool install maturin`.
- [PyO3](https://pyo3.rs/): for creating Python bindings in Rust. Adding it via `cargo add pyo3 --features auto-initialize`.


## Usage

```bash
# build 
cargo b   

# run
cargo r --bin main    # or simply `cargo r`
```


## Notes

### 1. Initializing python interpreter automatically

Add `auto-initialize` feature to `pyo3` dependency to avoid manually initializing a Python interpreter (by calling `Python::initialize`) in your Rust code. Do this by executing `cargo add pyo3 --features auto-initialize` or by adding the following to your [Cargo.toml](./Cargo.toml):

```toml
[dependencies]
pyo3 = { version = "0.26.0", features = ["auto-initialize"] } # set the version to the latest
```

### 2. Setting up the Python environment

Make sure to set up python environment properly for pyo3. `pyo3` requires Python executable to be found in the environment. You can set the `VIRTUAL_ENV` environment variable in [.cargo/config.toml](./.cargo/config.toml):

```toml
[env]
# for running pyo3 in rust
VIRTUAL_ENV = { value = ".venv", relative = true }
```

In windows, you may also need to append the python DLL directory to the `PATH` variable. You can set this in [build.rs](./build.rs):

```rs
use std::path::Path;

fn main() {
    check_python_venv();
    link_windows_python_dll();
}

fn check_python_venv() {
    let venv_dir_str = std::env::var("VIRTUAL_ENV").unwrap_or_default();
    let venv_dir = Path::new(&venv_dir_str);
    let python_executable = if cfg!(windows) {
        &venv_dir.join("Scripts/python.exe")
    } else {
        &venv_dir.join("bin/python")
    };

    let should_init_venv = !venv_dir.exists() || !python_executable.exists();
    if should_init_venv {
        println!(
            "cargo:warning=Virtual environment not found at {:?}. Running `uv sync`...",
            venv_dir
        );

        // Activate virtual environment using `uv`
        std::process::Command::new("uv")
            .arg("sync")
            .status()
            .expect("Failed to execute `uv sync`");
    }

    assert!(
        python_executable.exists(),
        "Python executable not found at {:?}",
        python_executable
    );


    // Set PYTHONPATH for searching local packages
    let site_packages_dir = venv_dir.join("Lib/site-packages");
    assert!(
        site_packages_dir.exists(),
        "site-packages directory not found at {:?}",
        site_packages_dir
    );
    println!("cargo:rustc-env=PYTHONPATH={}", site_packages_dir.to_str().unwrap());

    println!("cargo:rerun-if-changed={}", venv_dir.to_str().unwrap());
    println!(
        "cargo:rerun-if-changed={}",
        python_executable.to_str().unwrap()
    );
}

#[cfg(target_os = "windows")]
fn link_windows_python_dll() {
    let venv_dir_str = std::env::var("VIRTUAL_ENV").unwrap_or_default();
    let venv_dir = Path::new(&venv_dir_str);
    let python_executable = &venv_dir.join("Scripts/python.exe");
    let python_exe_str = &python_executable.to_str().unwrap();

    check_python_venv();
    assert!(
        python_executable.exists(),
        "Python executable not found at {:?}",
        python_exe_str
    );

    // Add python DLL directory to PATH
    let output = std::process::Command::new(python_exe_str)
        .args(&["-c", "import sys; print(sys.base_exec_prefix)"])
        .output()
        .expect("Failed to execute python command to get executable path");
    match output.status.success() {
        true => {
            let py_dll_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path_env = std::env::var("PATH").unwrap_or_default();
            println!("cargo:rustc-env=PATH={};{}", path_env, py_dll_dir);
        }
        false => {
            println!("cargo:warning=Failed to get Python executable path");
        }
    }
}
```


### 3. Import packages from local virtual environment

Before importing packages installed in the local virtual environment ( e.g. `numpy`), make sure to append the [.venv/Lib/site-pacages](.venv/Lib/site-packages) directory to the runtime `sys.path` list before importing the packages. 

You can do this by adding the following code snippet in your Rust code before importing any packages ( e.g. `src/lib.rs` ):

```rs
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


// example usage
pub fn import_numpy() -> PyResult<()> {
    Python::attach(|py| {
        // initialize python venv
        init_python_venv(py);

        // import numpy and use it
        let np = py.import("numpy")?;
        let array = np.call_method1("array", (vec![1, 2, 3],))?;
        let sum: i32 = np.getattr("sum")?.call1((&array,))?.extract()?;
        assert_eq!(sum, 6);
        Ok(())
    })
}
```


### 4. Call functions from python scripts

Read python scripts and call functions from them using `PyModule::from_code` ( e.g. [src/main.rs](src/main.rs) ):

```rs
pub fn from_py_code_file() -> PyResult<()> {
    Python::attach(|py| {
        let code_path = std::env::current_dir()?.join("src/example.py");
        let code = std::fs::read_to_string(code_path)?;

        let python_file = PyModule::from_code(
            py,
            CString::new(code)?.as_c_str(),
            c_str!("example.py"),
            c_str!("example"),
        )?;

        let add_result: i32 = python_file.getattr("add")?.call1((3, 5))?.extract()?;
        assert_eq!(add_result, 8);
        Ok(())
    })
}
```

