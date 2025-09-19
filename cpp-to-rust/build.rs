// build.rs

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
