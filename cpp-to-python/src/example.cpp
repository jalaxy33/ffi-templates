#include "example.h"

// Basic math functions

int add(int a, int b)
{
    return a + b;
}

double multiply(double a, double b)
{
    return a * b;
}


// string manipulation

std::string greet(const std::string &name)
{
    return "Hello, " + name + "!";
}


// vector operations

std::vector<int> square_vector(const std::vector<int> &input)
{
    std::vector<int> result;
    for (int num : input)
    {
        result.push_back(num * num);
    }
    return result;
}



