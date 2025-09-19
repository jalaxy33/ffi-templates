# C++ to Python templates


Create C++ extension for python using [pybind11](https://pybind11.readthedocs.io/).

**Hint**: You can inititialize a starter template by calling:
```bash
uv init --lib --build-backend=scikit <project-name>
```

## Requirements

- **C++ compiler**: g++/clang++/MSVC
- **[CMake](https://cmake.org/download/) build tools**
- **[uv](https://docs.astral.sh/uv/) python manager** 


## Toolchain

- [pybind11](https://pybind11.readthedocs.io/): for seamless integration between C++ and Python.
- [scikit-build-core](https://scikit-build-core.readthedocs.io/): modern build system for Python C++ extensions.
- [pybind11-stubgen](https://github.com/sizmailov/pybind11-stubgen): auto-generate type stubs for better IDE support.


## Usage


```bash
# Build C++ extension module
uv sync

# run python script (any script you want)
uv run src/main.py
```


## Keypoints

1. Include C++ source files in [CMakeLists.txt](./CMakeLists.txt):
```cmake
pybind11_add_module(${MODULE_NAME} MODULE src/extension.cpp src/example.cpp)  # replace with actual source files
target_include_directories(${MODULE_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
```


2. Generate module `__init__.py` automatically by setting template in [pyproject.toml](./pyproject.toml):
```toml
[[tool.scikit-build.generate]]
path = "cpp_to_python/__init__.py"   # replace cpp_to_python to actual module name
template = '''
from ._core import *

__version__ = "${version}"
'''
```

3. Generate module type stub file (`*.pyi`) automatically by adding a post build command in [CMakeLists.txt](./CMakeLists.txt):
```cmake
# generate stub files
add_custom_command(TARGET ${MODULE_NAME}
    POST_BUILD
    COMMAND ${PYTHON_EXECUTABLE} -m pybind11_stubgen ${MODULE_NAME} -o ${CMAKE_CURRENT_BINARY_DIR}/${SKBUILD_PROJECT_NAME}
    WORKING_DIRECTORY $<TARGET_FILE_DIR:${MODULE_NAME}>
    COMMENT "Generating stub files for ${SKBUILD_PROJECT_NAME} module"
    VERBATIM
)
install(FILES
    ${CMAKE_CURRENT_BINARY_DIR}/${SKBUILD_PROJECT_NAME}/${MODULE_NAME}.pyi
    DESTINATION ${SKBUILD_PROJECT_NAME}
    OPTIONAL
)
```

