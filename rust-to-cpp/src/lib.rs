// src/lib.rs

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn add(a: i32, b: i32) -> i32;
        fn greet(name: &str) -> String;
        
        // MyStruct related functions
        type MyStruct;
        fn create_my_struct(name: &str) -> Box<MyStruct>;
        fn add_value_to_struct(s: &mut MyStruct, val: i32);
        fn get_struct_values(s: &MyStruct) -> Vec<i32>;
        fn get_struct_name(s: &MyStruct) -> String;
    }
}


#[allow(dead_code)]
#[derive(Debug)]
struct MyStruct {
    value: Vec<i32>,
    name: String,
}

#[allow(dead_code)]
impl MyStruct {
    fn new(name: &str) -> Self {
        MyStruct {
            value: Vec::new(),
            name: name.to_string(),
        }
    }

    fn add_value(&mut self, val: i32) {
        self.value.push(val);
    }

    fn get_values(&self) -> &[i32] {
        &self.value
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

// --- Basic Functions ---

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// --- MyStruct FFI ---

fn create_my_struct(name: &str) -> Box<MyStruct> {
    Box::new(MyStruct::new(name))
}

fn add_value_to_struct(s: &mut MyStruct, val: i32) {
    s.add_value(val);
}

fn get_struct_values(s: &MyStruct) -> Vec<i32> {
    s.get_values().to_vec()
}

fn get_struct_name(s: &MyStruct) -> String {
    s.get_name().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_my_struct() {
        let mut my_struct = MyStruct::new("TestStruct");
        my_struct.add_value(10);
        my_struct.add_value(20);
        assert_eq!(my_struct.get_values(), &[10, 20]);
        assert_eq!(my_struct.get_name(), "TestStruct");
    }

    #[test]
    fn test_my_struct_ffi() {
        let mut my_struct = create_my_struct("TestStruct");
        add_value_to_struct(&mut my_struct, 10);
        add_value_to_struct(&mut my_struct, 20);
        assert_eq!(get_struct_values(&my_struct), vec![10, 20]);
        assert_eq!(get_struct_name(&my_struct), "TestStruct");
    }
}
