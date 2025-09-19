cmake_minimum_required(VERSION 3.10.0)
project(rust-to-cpp VERSION 0.1.0 LANGUAGES C CXX)

# =============================================================================
# Rust Library Configuration
# =============================================================================

set(RUST_LIB_NAME "rust_to_cpp")  # change this to actual Rust library name
set(RUST_TARGET_DIR "${CMAKE_SOURCE_DIR}/target")
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RUST_LIB_DIR "${RUST_TARGET_DIR}/debug")
    message(STATUS "Debug build")
else()
    set(RUST_LIB_DIR "${RUST_TARGET_DIR}/release")
    message(STATUS "Release build")
endif()

if (WIN32)
    set(RUST_LIB_PATH "${RUST_LIB_DIR}/${RUST_LIB_NAME}.dll")
elseif(APPLE)
    set(RUST_LIB_PATH "${RUST_LIB_DIR}/lib${RUST_LIB_NAME}.dylib")
else()
    set(RUST_LIB_PATH "${RUST_LIB_DIR}/lib${RUST_LIB_NAME}.so")
endif()

if (EXISTS ${RUST_LIB_PATH})
    message(STATUS "Rust lib exists")
else()
    message(FATAL_ERROR "Rust lib does not exist: ${RUST_LIB_PATH}. Please build the Rust library first.")
endif()


# =============================================================================
# Executable Target Configuration
# =============================================================================

set(targetname demo)
add_executable(${targetname} src/main.cpp)
target_include_directories(${targetname} PRIVATE ${CMAKE_SOURCE_DIR}/include)


# =============================================================================
# Link Rust Shared Library
# =============================================================================

# link the Rust shared library
if(WIN32) 
    # NOTE: On Windows, there are two types of libraries: the import library (.dll.lib) and the runtime library (.dll)
    target_link_libraries(${targetname} PRIVATE "${RUST_LIB_PATH}.lib")

    # Copy DLL to output directory on Windows
    add_custom_command(TARGET ${targetname} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        "${RUST_LIB_PATH}"
        $<TARGET_FILE_DIR:${targetname}>
        COMMENT "Copying ${RUST_LIB_NAME}.dll to output directory"
    )
elseif(APPLE)
    target_link_libraries(${targetname} PRIVATE "${RUST_LIB_PATH}")
else()
    target_link_libraries(${targetname} PRIVATE "${RUST_LIB_PATH}")
endif()

