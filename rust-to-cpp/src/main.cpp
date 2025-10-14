#include <iostream>
#include "cxx.h"
#include "lib.rs.h"

int main(int, char**){
    
    // --- Testing basic functions ---
    std::cout << "--- Testing basic functions ---\n";

    std::cout << "2 + 3 = " << add(2, 3) << "\n";
    std::cout << greet("World") << "\n";

    // --- Testing MyStruct ---
    std::cout << "\n--- Testing MyStruct ---\n";
    auto my_struct = create_my_struct("CppStruct");
    std::cout << "Created struct with name: " << get_struct_name(*my_struct) << "\n";

    add_value_to_struct(*my_struct, 10);
    add_value_to_struct(*my_struct, 20);
    add_value_to_struct(*my_struct, 30);
    
    auto values = get_struct_values(*my_struct);
    std::cout << "Values in struct: ";
    for (size_t i = 0; i < values.size(); ++i) {
        std::cout << values[i];
        if (i < values.size() - 1) std::cout << ", ";
    }
    std::cout << "\n";
    
    return 0;
}
