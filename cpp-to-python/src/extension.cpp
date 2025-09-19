#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

#include "example.h"

namespace py = pybind11;

PYBIND11_MODULE(_core, m)
{
  m.doc() = "C++ extenstion module";

  // function bindings

  m.def("print_hello", &print_hello, "Print hello message from C++");

  m.def("add", &add, "Add two integers",
          py::arg("a"), py::arg("b"));

  m.def("multiply", &multiply, "Multiply two integers",
          py::arg("a"), py::arg("b"));
  
  m.def("greet", &greet, "Greet a person",
          py::arg("name"));
  m.def("square_vector", &square_vector, "Square each element in a vector",
          py::arg("input"));
  
  // class bindings
  
  py::class_<Calculator>(m, "Calculator")
      .def(py::init<double>(), py::arg("initial") = 0.0)
      .def("add", &Calculator::add, "Add a value to the calculator",
           py::arg("x"))
      .def("subtract", &Calculator::subtract, "Subtract a value from the calculator",
           py::arg("x"))
      .def("get_value", &Calculator::get_value, "Get the current value of the calculator")
      .def("reset", &Calculator::reset, "Reset the calculator to zero");
}
