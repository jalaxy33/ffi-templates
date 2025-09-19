#include "example.h"
#include <iostream>

CppClass::CppClass() {}

std::unique_ptr<CppClass> new_cpp_class()
{
    std::cout << "Creating a new CppClass instance." << std::endl;
    return std::unique_ptr<CppClass>(new CppClass());
}

int c_add(int a, int b)
{
    return a + b;
}
