#pragma once
#include <memory>


class CppClass {
public:
  CppClass();
};

std::unique_ptr<CppClass> new_cpp_class();

int c_add(int a, int b);
