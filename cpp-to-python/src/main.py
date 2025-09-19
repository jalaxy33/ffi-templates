#!/usr/bin/env python3
"""
Simple C++ Python extension module test
"""

import unittest
import sys


class TestCppExtension(unittest.TestCase):
    """Unit tests for C++ Python extension module"""
    
    @classmethod
    def setUpClass(cls):
        """Set up test class by importing the module"""
        try:
            import cpp_to_python
            cls.module = cpp_to_python
            print("✅ Successfully imported cpp_to_python module")
            print(f"📦 Module version: {cls.module.__version__}")
        except ImportError as e:
            print(f"❌ Failed to import module: {e}")
            print("Please build module first")
            raise unittest.SkipTest("Module not available")
    
    def test_basic_functions(self):
        """🧪 Test basic math and string functions"""
        print("\n🧪 === Testing Basic Functions ===")
        
        # Test print_hello (no assertion needed, just verify it doesn't crash)
        try:
            self.module.print_hello()
            print("✅ print_hello() executed successfully")
        except Exception as e:
            self.fail(f"print_hello() failed: {e}")
        
        # Test add function
        result_add = self.module.add(5, 3)
        print(f"➕ add(5, 3) = {result_add}")
        self.assertEqual(result_add, 8, f"Expected 8, got {result_add}")
        
        # Test multiply function
        result_multiply = self.module.multiply(2.5, 4.0)
        print(f"✖️ multiply(2.5, 4.0) = {result_multiply}")
        self.assertAlmostEqual(result_multiply, 10.0, places=7, 
                              msg=f"Expected 10.0, got {result_multiply}")
        
        # Test greet function
        result_greet = self.module.greet("World")
        print(f"👋 greet('World') = '{result_greet}'")
        self.assertEqual(result_greet, "Hello, World!", 
                        f"Expected 'Hello, World!', got '{result_greet}'")
    
    def test_vector_operations(self):
        """📊 Test vector processing functions"""
        print("\n📊 === Testing Vector Processing ===")
        
        # Test normal vector
        test_vec = [1, 2, 3, 4]
        result_vector = self.module.square_vector(test_vec)
        print(f"📊 square_vector({test_vec}) = {result_vector}")
        expected_vector = [1, 4, 9, 16]
        self.assertEqual(result_vector, expected_vector, 
                        f"Expected {expected_vector}, got {result_vector}")
        
        # Test empty vector
        empty_result = self.module.square_vector([])
        print(f"📊 square_vector([]) = {empty_result}")
        self.assertEqual(empty_result, [], f"Expected [], got {empty_result}")
        
        # Test negative numbers
        neg_result = self.module.square_vector([-2, -3])
        print(f"📊 square_vector([-2, -3]) = {neg_result}")
        self.assertEqual(neg_result, [4, 9], f"Expected [4, 9], got {neg_result}")
    
    def test_calculator_class(self):
        """🧮 Test Calculator class functionality"""
        print("\n🧮 === Testing Calculator Class ===")
        
        # Test initialization
        calc = self.module.Calculator(10.0)
        print(f"🔢 Initial value: {calc.get_value()}")
        self.assertAlmostEqual(calc.get_value(), 10.0, places=7,
                              msg=f"Expected 10.0, got {calc.get_value()}")
        
        # Test add operation
        calc.add(5.0)
        print(f"➕ After adding 5: {calc.get_value()}")
        self.assertAlmostEqual(calc.get_value(), 15.0, places=7,
                              msg=f"Expected 15.0, got {calc.get_value()}")
        
        # Test subtract operation
        calc.subtract(3.0)
        print(f"➖ After subtracting 3: {calc.get_value()}")
        self.assertAlmostEqual(calc.get_value(), 12.0, places=7,
                              msg=f"Expected 12.0, got {calc.get_value()}")
        
        # Test reset operation
        calc.reset()
        print(f"🔄 After reset: {calc.get_value()}")
        self.assertAlmostEqual(calc.get_value(), 0.0, places=7,
                              msg=f"Expected 0.0, got {calc.get_value()}")
    
    def test_calculator_edge_cases(self):
        """🔍 Test Calculator class edge cases"""
        print("\n🔍 === Testing Calculator Edge Cases ===")
        
        # Test default initialization
        calc_default = self.module.Calculator()
        print(f"� Default initialization: {calc_default.get_value()}")
        self.assertAlmostEqual(calc_default.get_value(), 0.0, places=7)
        
        # Test negative operations
        calc = self.module.Calculator(5.0)
        calc.add(-3.0)
        print(f"➕ After adding -3: {calc.get_value()}")
        self.assertAlmostEqual(calc.get_value(), 2.0, places=7)
        
        calc.subtract(-2.0)  # subtracting negative = adding positive
        print(f"➖ After subtracting -2: {calc.get_value()}")
        self.assertAlmostEqual(calc.get_value(), 4.0, places=7)
    
    def test_string_edge_cases(self):
        """🔍 Test string function edge cases"""
        print("\n🔍 === Testing String Edge Cases ===")
        
        # Test greet with empty string
        empty_greet = self.module.greet("")
        print(f"👋 greet('') = '{empty_greet}'")
        self.assertEqual(empty_greet, "Hello, !", 
                        f"Expected 'Hello, !', got '{empty_greet}'")
        
        # Test greet with special characters
        special_greet = self.module.greet("C++")
        print(f"👋 greet('C++') = '{special_greet}'")
        self.assertEqual(special_greet, "Hello, C++!", 
                        f"Expected 'Hello, C++!', got '{special_greet}'")


def run_cpp_extension_tests():
    """Run all tests with custom test runner"""
    print("🚀 C++ Python Extension Module Unit Tests")
    print("=" * 50)
    
    # Create test suite
    loader = unittest.TestLoader()
    suite = loader.loadTestsFromTestCase(TestCppExtension)
    
    # Run tests with verbose output
    runner = unittest.TextTestRunner(verbosity=2, stream=sys.stdout)
    result = runner.run(suite)
    
    # Print summary
    print("\n" + "=" * 50)
    if result.wasSuccessful():
        print("🎉 All tests passed successfully!")
    else:
        print(f"❌ {len(result.failures)} test(s) failed, {len(result.errors)} error(s)")
        
    return result.wasSuccessful()


if __name__ == "__main__":
    success = run_cpp_extension_tests()
    sys.exit(0 if success else 1)