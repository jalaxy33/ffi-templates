cmake_minimum_required(VERSION 3.10.0)
project(python-to-cpp VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)

# --------------------------------------------------------------------------------
# Setup Python and pybind11
# --------------------------------------------------------------------------------

# setup python environment
set(PYTHON_VENV_DIR ${CMAKE_SOURCE_DIR}/.venv)

if (WIN32)
    set(PYTHON_EXECUTABLE ${PYTHON_VENV_DIR}/Scripts/python.exe)
else()
    set(PYTHON_EXECUTABLE ${PYTHON_VENV_DIR}/bin/python)
endif()
set(Python_EXECUTABLE ${PYTHON_EXECUTABLE})  # for compatibility with pybind11

if (NOT EXISTS ${PYTHON_EXECUTABLE})
    message(FATAL_ERROR "Python executable not found at ${PYTHON_EXECUTABLE}. Please create a virtual environment at ${PYTHON_VENV_DIR} and install the required packages.")   
endif()

# set pybind11 dependency
set(pybind11_ROOT ${PYTHON_VENV_DIR}/Lib/site-packages/pybind11)
set(pybind11_DIR ${pybind11_ROOT}/share/cmake/pybind11)

if (NOT EXISTS ${pybind11_ROOT})
    message(FATAL_ERROR "pybind11 not found at ${pybind11_ROOT}. Please install pybind11 in the virtual environment.")
endif()

# find pybind11 package for CMake
set(PYBIND11_FINDPYTHON ON)
find_package(pybind11 CONFIG REQUIRED)


# --------------------------------------------------------------------------------
# Setup target
# --------------------------------------------------------------------------------


# build executable
set(TARGET_NAME demo)
add_executable(${TARGET_NAME} src/main.cpp)
target_link_libraries(${TARGET_NAME} PRIVATE pybind11::embed)
target_include_directories(${TARGET_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/include)

# --------------------------------------------------------------------------------
# Ensure correct runtime python environment
# --------------------------------------------------------------------------------

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


# set runtime PYTHONHOME environment variable
set(PYTHON_HOME_PATH "${PYTHON_SYS_ROOT}")
string(REPLACE "\\" "\\\\" PYTHON_HOME_PATH_ESCAPED "${PYTHON_HOME_PATH}")
# set compile flag (PYTHON_HOME_PATH)
target_compile_definitions(${TARGET_NAME} PRIVATE PYTHON_HOME_PATH="${PYTHON_HOME_PATH_ESCAPED}")


# pass virtual environment site-packages to runtime
set(VENV_PACKAGES_DIR "${CMAKE_SOURCE_DIR}/.venv/Lib/site-packages")
string(REPLACE "\\" "\\\\" VENV_PACKAGES_DIR_ESCAPED "${VENV_PACKAGES_DIR}")
# set compile flag (VENV_PACKAGES_DIR)
target_compile_definitions(${TARGET_NAME} PRIVATE VENV_PACKAGES_DIR="${VENV_PACKAGES_DIR_ESCAPED}")