# C++ to Rust template

A project template for calling C++ code from Rust using [cxx](https://cxx.rs/).


## Requirements

- **C++ compiler**: g++/clang++/MSVC
- **[CMake](https://cmake.org/download/) build tools**
- **[Rust toolchain](https://www.rust-lang.org/tools/install)**


## Toolchain

- **[cxx](https://cxx.rs/)**: Safe interop between Rust and C++. Install it with：
```bash
cargo add cxx
cargo add cxx-build --build --features parallel
```

## Usage

```bash
# build only
cargo b     # Debug build
cargo b -r  # Release build

# build and run
cargo r     # Debug build
cargo r -r  # Release build
```


## Keypoints

### 1. Define C++ interface in Rust

Define a interface module manually in Rust code ( e.g. [main.rs](src/main.rs) ) according to your C++ code (e.g. [example.h](include/example.h) ):

```rust
// in src/main.rs

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("example.h");

        pub type CppClass;

        pub fn new_cpp_class() -> UniquePtr<CppClass>;

        pub fn c_add(a: i32, b: i32) -> i32;
    }
}
```

### 2. Generate C++ bindings


Generate C++ bindings by defining the compile behavior in [build.rs](./build.rs):

```rust
fn main() {    
    // C++ to Rust (change file names as needed)
    cxx_build::bridge("src/main.rs")
    .file("src/example.cpp")  
    .include("include")   // add C++ header file directory
    .std("c++17")
    .compile("cpp-to-rust");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/example.cpp");
    println!("cargo:rerun-if-changed=include/example.h");
}
```

