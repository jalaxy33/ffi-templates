#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("example.h");

        pub type CppClass;

        pub fn new_cpp_class() -> UniquePtr<CppClass>;

        pub fn c_add(a: i32, b: i32) -> i32;
    }
}


fn main() {
    ffi::new_cpp_class();
    let result = ffi::c_add(5, 7);
    println!("Result of addition: {}", result);
}
