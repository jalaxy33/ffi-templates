# Python to C++ template

A simple template to demonstrate how to call python functions in C++.

## Requirements

- **C++ compiler**: g++/clang++/MSVC (C++17 or above)
- **[CMake](https://cmake.org/download/) build tools**
- **[uv](https://docs.astral.sh/uv/) python manager** 


## Toolchain

- [pybind11](https://pybind11.readthedocs.io/): for seamless integration between C++ and Python. Install it via uv: `uv add pybind11 --dev`. 


## Usage

```bash
# 1. Install python dependencies (pybind11 is required, others are optional)
uv sync

# 2. Configure and build C++ target
cmake -B build && cmake --build build  # Debug build
cmake -B build && cmake --build build --config Release  # Release build

# 3. Run the executable, change `demo` to actual target name 
./build/Debug/demo.exe  # On Windows
./build/Debug/demo      # On Linux/Mac
```


## Keypoints

### 1. Setting up Python Environment in C++

Set up `pybind11` in [CMakeLists.txt](./CMakeLists.txt):

```cmake
# setup python environment
set(PYTHON_VENV_DIR ${CMAKE_SOURCE_DIR}/.venv)

if (WIN32)
    set(PYTHON_EXECUTABLE ${PYTHON_VENV_DIR}/Scripts/python.exe)
else()
    set(PYTHON_EXECUTABLE ${PYTHON_VENV_DIR}/bin/python)
endif()
set(Python_EXECUTABLE ${PYTHON_EXECUTABLE})  # for compatibility with pybind11


# set pybind11 dependency
set(pybind11_ROOT ${PYTHON_VENV_DIR}/Lib/site-packages/pybind11)
set(pybind11_DIR ${pybind11_ROOT}/share/cmake/pybind11)


# find pybind11 package for CMake
set(PYBIND11_FINDPYTHON ON)
find_package(pybind11 CONFIG REQUIRED)

```

Link `pybind11::embed` to your target:

```cmake
target_link_libraries(${TARGET_NAME} PRIVATE pybind11::embed)
```


**Windows specific**: Copy `pythonXX.dll` to target output directory:

```cmake 
# Get python info
execute_process(
    COMMAND ${PYTHON_EXECUTABLE} -c "import sysconfig; print(sysconfig.get_config_var('installed_platbase'))"
    OUTPUT_VARIABLE PYTHON_SYS_ROOT
    OUTPUT_STRIP_TRAILING_WHITESPACE
)
execute_process(
    COMMAND ${PYTHON_EXECUTABLE} -c "import sysconfig; print(sysconfig.get_config_var('py_version_nodot'))"
    OUTPUT_VARIABLE PYTHON_VERSION_NODOT
    OUTPUT_STRIP_TRAILING_WHITESPACE
)


# Windows specific settings: copy python DLL to output directory
if(WIN32)
    set(PYTHON_DLL_PATH "${PYTHON_SYS_ROOT}/python${PYTHON_VERSION_NODOT}.dll")

    # Copy Python DLL to output directory
    if(EXISTS ${PYTHON_DLL_PATH})
        add_custom_command(
            TARGET ${TARGET_NAME}
            POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                ${PYTHON_DLL_PATH}
                $<TARGET_FILE_DIR:${TARGET_NAME}>
            COMMENT "Copying Python DLL to output directory"
        )
    else()
        message(FATAL_ERROR "Python DLL not found at: ${PYTHON_DLL_PATH}")
    endif()
    
endif()
```


## 2. Setup runtime python interpreter environment

In order to run python code in C++ normally, `PYTHONPATH` environment variable needs to be set correctly to the directory of system python libraries. This could be done half-automatically by setting a compile flag and calling `SetEnvironmentVariableA` before initializing python interpreter in C++ code.

**step 1**: Set compile flag in [CMakeLists.txt](./CMakeLists.txt):

```cmake
# set runtime PYTHONHOME environment variable
set(PYTHON_HOME_PATH "${PYTHON_SYS_ROOT}")
string(REPLACE "\\" "\\\\" PYTHON_HOME_PATH_ESCAPED "${PYTHON_HOME_PATH}")
# set compile flag (PYTHON_HOME_PATH)
target_compile_definitions(${TARGET_NAME} PRIVATE PYTHON_HOME_PATH="${PYTHON_HOME_PATH_ESCAPED}")
```

**step 2**: define a function to call `SetEnvironmentVariableA` ( e.g. [python_helper.h](./include/python_helper.h) ):

```cpp
// Configure Python home environment (must be called before interpreter initialization)
void configure_python_home()
{
#ifdef _WIN32
#ifdef PYTHON_HOME_PATH
    SetEnvironmentVariableA("PYTHONHOME", PYTHON_HOME_PATH);
#endif
#endif
}
```

**step 3**: call `configure_python_home()` before `py::initialize_interpreter()` ( e.g. [main.cpp](./src/main.cpp) ):

```cpp
// setup Python environment
configure_python_home();
py::scoped_interpreter guard{}; // start the interpreter and keep it alive
```

## 3. Add virtual environment packages to `sys.path`

The package installed in virtual environment (e.g. `.venv/Lib/site-packages`) is not included in `sys.path` by default. To use these packages in C++, you need to manually add the path to `sys.path`. We can also set a compile flag to pass the path from CMake to C++ code.

**step 1**: Set compile flag in [CMakeLists.txt](./CMakeLists.txt):

```cmake
# pass virtual environment site-packages to runtime
set(VENV_PACKAGES_DIR "${CMAKE_SOURCE_DIR}/.venv/Lib/site-packages")
string(REPLACE "\\" "\\\\" VENV_PACKAGES_DIR_ESCAPED "${VENV_PACKAGES_DIR}")
# set compile flag (VENV_PACKAGES_DIR)
target_compile_definitions(${TARGET_NAME} PRIVATE VENV_PACKAGES_DIR="${VENV_PACKAGES_DIR_ESCAPED}")
```

**step 2**: define a function to add the path to `sys.path` ( e.g. [python_helper.h](./include/python_helper.h) ):

```cpp
// Setup virtual environment paths (must be called after interpreter initialization)
void setup_virtual_environment()
{
#ifdef VENV_PACKAGES_DIR
    std::string venv_packages_dir = VENV_PACKAGES_DIR;

    // Convert backslashes to forward slashes for Python path compatibility
    for (char &c : venv_packages_dir)
    {
        if (c == '\\')
            c = '/';
    }

    // Add virtual environment path to sys.path if it exists and not already added
    std::string setup_code =
        "import sys\n"
        "import os\n"
        "venv_path = r'" + venv_packages_dir + "'\n"
        "if os.path.exists(venv_path) and venv_path not in sys.path:\n"
        "    sys.path.insert(0, venv_path)\n";

    py::exec(setup_code);
#endif
}
```

**step 3**: call `setup_virtual_environment()` after `py::initialize_interpreter()` ( e.g. [main.cpp](./src/main.cpp) ):

```cpp
// setup Python environment
configure_python_home();
py::scoped_interpreter guard{}; // start the interpreter and keep it alive

setup_virtual_environment();
```


## 4. Call functions from python files

In order to call functions from python files wherever they are, you need to add the script directory to `sys.path`. This can be done by defining a function that takes the script directory as an argument and adds it to `sys.path`.

**step 1**: define a function to add the script directory to `sys.path` ( e.g. [python_helper.h](./include/python_helper.h) ):

```cpp
// Add python script directory to sys.path (must be called after interpreter initialization).
// `script_dir` is relative to project root, default is "src"
void setup_script_directory(std::string script_dir = "src")
{
    // this would be different based on your project structure
    auto project_root = std::filesystem::path(__FILE__).parent_path().parent_path();
    auto script_abs_dir = project_root.append(script_dir).string();

    // Check if the directory exists
    if (!std::filesystem::exists(script_abs_dir) || !std::filesystem::is_directory(script_abs_dir))
    {
        std::cerr << "Script directory does not exist: " << script_abs_dir << std::endl;
        throw std::runtime_error("Script directory does not exist: " + script_abs_dir);
    }
    
    // Convert backslashes to forward slashes for Python path compatibility
    for (char &c : script_abs_dir)
    {
        if (c == '\\')
            c = '/';
    }

    std::string setup_code =
        "import sys, os\n"
        "src_path = os.path.abspath('" + script_abs_dir + "')\n"
        "if src_path not in sys.path:\n"
        "    sys.path.insert(0, src_path)\n";

    py::exec(setup_code);
}
```

**step 2**: call `setup_script_directory("your_script_dir")` before importing the python module ( e.g. [main.cpp](./src/main.cpp) ):

```cpp
setup_script_directory("src");  // add script directory to sys.path
py::module_ my_module = py::module_::import("example"); // load src/example.py

// call function 'add' from example.py
py::object add_func = my_module.attr("add");
int result = add_func(3, 4).cast<int>();
std::cout << "Result of example.add(3, 4)=" << result << std::endl;

```







