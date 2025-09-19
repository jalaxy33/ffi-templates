#pragma once

#include <iostream>
#include <string>
#include <vector>

inline void print_hello()
{
    std::cout << "Hello from util.h!" << std::endl;
}

// Basic math functions
int add(int a, int b);
double multiply(double a, double b);

// string manipulation
std::string greet(const std::string &name);

// vector operations
std::vector<int> square_vector(const std::vector<int> &input);

// Simple Calculator class
class Calculator
{
private:
    double value_;

public:
    Calculator(double initial) : value_(initial) {}

    void add(double x) { value_ += x; }
    void subtract(double x) { value_ -= x; }
    double get_value() const { return value_; }
    void reset() { value_ = 0; }
};
