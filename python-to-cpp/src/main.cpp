#include <pybind11/embed.h>
#include <iostream>
#include <string>

#include "python_helper.h"

namespace py = pybind11;
using namespace py::literals;

// define a new module
PYBIND11_EMBEDDED_MODULE(new_module, m)
{
    m.def("minus", [](int a, int b)
          { return a - b; });
}

int main(int, char **)
{
    std::cout << "Hello, from python-to-cpp!\n"
              << std::endl;

    // setup Python environment
    configure_python_home();
    py::scoped_interpreter guard{}; // start the interpreter and keep it alive
    setup_virtual_environment();

    // ---------- Use embedded Python API ------------

    py::print("Hello, World! from embedded Python API\n");

    // ---------- Execute Python code ------------

    py::print("Equivalent code execution:");

    py::exec(R"(
        kwargs = dict(name="World", number=42)
        message = "Hello, {name}! The answer is {number}".format(**kwargs)
        print(message)
    )");

    // Equivalent to:
    auto kwargs = py::dict("name"_a = "World", "number"_a = 42); // after using namespace py::literals;
    auto message = "Hello, {name}! The answer is {number}"_s.format(**kwargs);
    py::print(message);

    // two approches can also be combined
    auto locals = py::dict("name"_a = "World", "number"_a = 42);
    py::exec(R"(
        message = "Hello, {name}! The answer is {number}".format(**locals())
    )",
             py::globals(), locals);

    auto message_ = locals["message"].cast<std::string>();
    std::cout << message_ << std::endl;

    // ---------- Import Python module ------------

    py::print("\nImporting and using Python module:");

    py::module_ sys = py::module_::import("sys");
    py::print("sys.platform:", sys.attr("platform"));

    py::module_ np = py::module_::import("numpy");
    py::print("numpy version:", np.attr("__version__"));

    auto np_array = np.attr("array")(py::make_tuple(1, 2, 3, 4, 5));
    py::print("numpy array:", np_array,
              ", numpy array mean:", np.attr("mean")(np_array));

    // equivalent to:
    py::dict locals_;
    py::exec(R"(
        import sys
        import numpy as np
        np_array = np.array((1, 2, 3, 4, 5))
        arr_mean = np.mean(np_array)
        print("numpy array:", np_array, ", numpy array mean:", arr_mean)
    )",
             py::globals(), locals_);
    auto np_array_ = locals_["np_array"];
    py::print("From locals: ",
              "numpy array:", np_array_,
              ", numpy array mean:", np.attr("mean")(np_array_));

    // ---------- Call function from file ------------

    py::print("\nCalling function from Python file:");

    setup_script_directory("src");                          // add script directory to sys.path
    py::module_ my_module = py::module_::import("example"); // load src/example.py

    // call function 'add' from example.py
    py::object add_func = my_module.attr("add");
    int result = add_func(3, 4).cast<int>();
    std::cout << "Result of example.add(3, 4)=" << result << std::endl;

    // call class 'Calculator' from example.py
    py::print("\nCalling class from Python file:");

    py::object calc_class = my_module.attr("Calculator");
    py::object calc = calc_class(10.0);
    double initial_value = calc.attr("get_value")().cast<double>();
    std::cout << "Calculator(10), value=" << initial_value << std::endl;

    calc.attr("add")(5.0);
    double result_value = calc.attr("get_value")().cast<double>();
    std::cout << "After add(5), value=" << result_value << std::endl;

    // ---------- Add new module ------------

    py::print("\nCalling from new defined module in PYBIND11_EMBEDDED_MODULE:");

    auto new_module = py::module_::import("new_module");
    int result_minus = new_module.attr("minus")(10, 4).cast<int>();
    std::cout << "Result of new_module.minus(10, 4)=" << result_minus << std::endl;

    return 0;
}
