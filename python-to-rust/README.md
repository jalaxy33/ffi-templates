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


TODO



