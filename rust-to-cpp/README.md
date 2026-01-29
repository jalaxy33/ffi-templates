# Rust to C++ template

A simple template for creating a Rust library that can be called from C++ using [cxx](https://cxx.rs/).

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

1. Configure and build the C++ project using CMake:

    ```bash
    # Debug
    cmake -B build/Debug -DCMAKE_BUILD_TYPE=Debug
    cmake --build build/Debug

    # Release
    cmake -B build/Release -DCMAKE_BUILD_TYPE=Release
    cmake --build build/Release
    ```
    
3. Run the executable (change `demo` to actual target name):
    ```bash
    # Debug build
    ./build/Debug/demo.exe  # On Windows
    ./build/Debug/demo      # On Linux/Mac

    # Release build
    ./build/Release/demo.exe  # On Windows
    ./build/Release/demo      # On Linux/Mac
    ```


## Notes

### 1. Build Rust shared library

Build the Rust code as a shared library by specifying the crate type `cdylib` in [`Cargo.toml`](Cargo.toml):

```toml
[lib]
name = "librust"
crate-type = ["rlib", "cdylib"]
```

### 2. Define bridge modules

Define the exported functions and types in a module annotated with `#[cxx::bridge]` in [`src/lib.rs`](src/lib.rs) (or other Rust source files):

```rust
// src/lib.rs (or other Rust source file)

#[cxx::bridge]
mod ffi {

    // Wrap within `extern "Rust"` block
    extern "Rust" {
        fn add(a: i32, b: i32) -> i32;
        fn greet(name: &str) -> String;
        
        // Export Rust types
        type MyStruct;
        fn create_my_struct(name: &str) -> Box<MyStruct>;
        fn add_value_to_struct(s: &mut MyStruct, val: i32);
        fn get_struct_values(s: &MyStruct) -> Vec<i32>;
        fn get_struct_name(s: &MyStruct) -> String;
    }
}

// Implement the exported functions and types
// ...

```

### 3. Export Rust library using `cxx-build`

Mark the Rust source file containing the `#[cxx::bridge]` module in [`build.rs`](build.rs) using `cxx_build::bridge`:

```rust
// build.rs

fn main() {
    // specify source file containing #[cxx::bridge] modules
    let _ = cxx_build::bridge("src/lib.rs");   
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
```


### 4. Link Rust library in CMake

Link the generated Rust shared library in your CMake project by defining the following function to your [`CMakeLists.txt`](CMakeLists.txt):

```cmake
set(RUST_LIB_NAME librust)   # replace `librust` with actual Rust library name
set(RUST_PROJECT_NAME rust-to-cpp)  # replace `rust-to-cpp` with actual Cargo project name
set(RUST_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

# Setup Rust library integration for target
function(setup_rust_library target_name)
    # Find Cargo executable
    find_program(CARGO_EXECUTABLE cargo REQUIRED)
    
    # Build Rust library in release mode during configuration
    execute_process(
        COMMAND cargo build --release
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
    
    # Define library paths (always use release)
    set(rust_lib_dir "${RUST_TARGET_DIR}/release")
    set(rust_lib_file "${CMAKE_STATIC_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}")
    
    # Fix Windows MSVC runtime library compatibility
    if(WIN32 AND MSVC)
        target_compile_options(${target_name} PRIVATE /MD)
    endif()
    
    # Setup CXX bridge include paths and sources
    target_include_directories(${target_name} PRIVATE 
        ${RUST_TARGET_DIR}/cxxbridge/rust
        ${RUST_TARGET_DIR}/cxxbridge/${RUST_PROJECT_NAME}/src
    )
    
    file(GLOB cxx_bridge_src "${RUST_TARGET_DIR}/cxxbridge/${RUST_PROJECT_NAME}/src/*.cc")
    if(cxx_bridge_src)
        target_sources(${target_name} PRIVATE ${cxx_bridge_src})
    endif()
        
    # Link Rust library
    set(rust_lib_suffix "$<IF:$<PLATFORM_ID:Windows>,.lib,>")
    target_link_libraries(${target_name} PRIVATE "${rust_lib_dir}/${rust_lib_file}${rust_lib_suffix}")
    
    # Link CXX runtime library
    file(GLOB cxx_dirs "${rust_lib_dir}/build/cxx-*/out")
    if(cxx_dirs)
        list(GET cxx_dirs 0 cxx_dir)
        file(GLOB cxx_lib "${cxx_dir}/*${CMAKE_STATIC_LIBRARY_SUFFIX}")
        if(cxx_lib)
            target_link_libraries(${target_name} PRIVATE ${cxx_lib})
        endif()
    else()
        message(WARNING "CXX library not found for release build")
    endif()
    
    # Copy DLL on Windows
    if(WIN32)
        add_custom_command(TARGET ${target_name} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            "${rust_lib_dir}/${rust_lib_file}"
            $<TARGET_FILE_DIR:${target_name}>
            COMMENT "Copying Rust DLL for $<CONFIG>"
        )
    endif()
endfunction()
```

And then call `setup_rust_library` with your target name after defining the executable or library target:

```cmake
set(targetname demo)
add_executable(${targetname} src/main.cpp)

# Setup Rust library build and linking
setup_rust_library(${targetname})
```


