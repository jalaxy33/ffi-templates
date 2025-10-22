# Rust to Python Template


A template project for creating Rust extensions for Python using [PyO3](https://pyo3.rs/) and [Maturin](https://maturin.rs/).



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
- [task](https://taskfile.dev/): for task management (optional, but recommended for running commands easily)


## Toolchain

- [maturin](https://maturin.rs/): for building and publishing Rust-based Python packages. Install it via `uv tool install maturin` or `pip install maturin`.
- [PyO3](https://pyo3.rs/): for creating Python bindings in Rust. Adding it via `maturin init --bindings pyo3` or `cargo add pyo3 --features extension-module`.
- [pyo3-stub-gen](https://github.com/Jij-Inc/pyo3-stub-gen): for generating Python stubs from PyO3 Rust bindings.


## Usage

**[Option 1] Step-by-step guide:**

```bash
# install python dependencies
uv sync

# generate stub files
cargo r --bin stubgen

# build rust extension
uv run maturin develop

# run python script (any script you want)
uv run tests/test.py
```

**[Option 2] Run task in [Taskfile.yml](./Taskfile.yml):**

```bash
# Install dependencies, generate stubs and build the extension
task develop    # or just `task`

# Test
task test-py   # equivalent to `uv run tests/test.py`
task test-rs   # equivalent to `cargo test`
```

## Notes

### 1. Python metadata

maturin merges metadata from both [`Cargo.toml`](./Cargo.toml) and [`pyproject.toml`](./pyproject.toml), `pyproject.toml` takes precedence over `Cargo.toml`.


### 2. Create your Rust extension library

Make sure to compile the rust codes as a C-compatible library. This is done by setting the `crate-type` in [Cargo.toml](./Cargo.toml):

```toml
[lib]
name = "rust_to_python"   # Name of the Rust library
crate-type = ["rlib", "cdylib"]   # `cdylib` is necessary for Python FFI
```

### 3. Specify the Extension module name

Set the module name in [pyproject.toml](./pyproject.toml):

```toml
[tool.maturin]
features = ["pyo3/extension-module"]
module-name = "rust_module"   # Name of the Python module
```

As well as in the Rust code ( e.g. [src/lib.rs](./src/lib.rs) ):

```rs
use pyo3::prelude::*;

// ...

/// [IMPORTANT!!!] Set your module name to match `module-name` in pyproject.toml
#[pymodule]
fn rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // ...
    Ok(())
}
```


### 4. Generate Python stubs for IDE type hints

We can generate python `*.pyi` stub files for pyo3 rust bindings using the [pyo3-stub-gen](https://github.com/Jij-Inc/pyo3-stub-gen) crate. Install it via:

```bash
cargo add pyo3-stub-gen
```

Add attributes to the functions and classes in your Rust code to generate the stubs correctly ( e.g. [src/lib.rs](./src/lib.rs) ):

```rs
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};

// Example function
#[pyfunction]
#[gen_stub_pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// Example struct
#[pyclass]
#[gen_stub_pyclass]
pub struct Calculator {
    #[pyo3(get, set)]
    value: f64,
}

// Example method
#[pymethods]
#[gen_stub_pymethods]
impl Calculator {
    #[new]
    pub fn new(value: f64) -> Self {
        Calculator { value }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}


// generate stub info for the module
define_stub_info_gatherer!(stub_info);

```

Then create a source file for the stub generator (e.g. [src/stubgen.rs](./src/stubgen.rs)):

```rs
use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    // `stub_info` is a function defined by `define_stub_info_gatherer!` macro.
    let stub = rust_to_python::stub_info()?;   // Change `rust_to_python` to your crate name
    stub.generate()?;
    Ok(())
}
```

And add a binary target in [Cargo.toml](./Cargo.toml):

```toml
[[bin]]
name = "stubgen"
path = "src/bin/stubgen.rs"
```

Now you can generate the stub files by running:

```bash
cargo r --bin stubgen
```

A `.pyi` file will be generated in the crate root directory (e.g. `rust_to_python.pyi`). It will be installed to the environment package directory when you run `maturin develop` or `maturin build` (check `.venv/Lib/site-packages/<your-module-name>` after running these commands).


### [Optional] 5. Run pyo3 in Rust codes

If you want to run rust codes that use pyo3 (e.g. [tests/test.rs](./tests/test.rs)), make sure the Python environment is correctly set.

`pyo3` requires Python executable to be found in the environment. You can set the `VIRTUAL_ENV` environment variable in [.cargo/config.toml](./.cargo/config.toml):

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
