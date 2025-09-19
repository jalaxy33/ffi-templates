#include <iostream>
#include <string>
#include <stdexcept>
#include "rust_to_cpp.h" // Header for Rust functions

class TestRunner {
public:
    // Simple test runner with basic assertions
    
    // Run a simple test case
    static void run_test(const std::string& test_name, bool condition) {
        std::cout << "Running: " << test_name << " ... ";
        if (condition) {
            std::cout << "PASS" << std::endl;
        } else {
            std::cout << "FAIL" << std::endl;
        }
    }
    
    // Assert that two integers are equal
    static void assert_equal(int expected, int actual) {
        if (expected != actual) {
            throw std::runtime_error("Expected " + std::to_string(expected) + 
                                   " but got " + std::to_string(actual));
        }
    }
    
    // Example test method
    static void test_add_function() {
        std::cout << "\n=== Running Unit Tests ===" << std::endl;
        
        // Test case 1: Basic addition
        try {
            int result = add(2, 3);
            assert_equal(5, result);
            run_test("test_basic_addition", true);
        } catch (...) {
            run_test("test_basic_addition", false);
        }
        
        // Test case 2: Adding zero
        try {
            int result = add(5, 0);
            assert_equal(5, result);
            run_test("test_add_zero", true);
        } catch (...) {
            run_test("test_add_zero", false);
        }
        
        // Test case 3: Negative numbers
        try {
            int result = add(-2, -3);
            assert_equal(-5, result);
            run_test("test_negative_numbers", true);
        } catch (...) {
            run_test("test_negative_numbers", false);
        }
        
        std::cout << "=== Tests Complete ===" << std::endl;
    }
};

int main()
{
    say_hello();
    call_inner_function();
    
    // Run unit tests
    TestRunner::test_add_function();
    
    return 0;
}