#[unsafe(no_mangle)]
pub extern "C" fn say_hello() {
    println!("Hello from Rust!");
}

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn inner_function() {
    println!("This is an inner function, not exposed to C++.");
}

#[unsafe(no_mangle)]
pub extern "C" fn call_inner_function() {
    inner_function();
}

// ============================================================================
//                               UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 5), 5);
        assert_eq!(add(5, 0), 5);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_add_mixed_signs() {
        assert_eq!(add(-5, 3), -2);
        assert_eq!(add(5, -3), 2);
    }

    #[test]
    fn test_call_inner_function() {
        // This test just ensures the function doesn't panic
        call_inner_function();
    }

    #[test]
    fn test_say_hello() {
        // This test just ensures the function doesn't panic
        say_hello();
    }
}
