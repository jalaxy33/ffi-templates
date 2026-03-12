# =============================================================================
# Preamble
# =============================================================================

cmake_minimum_required(VERSION 3.10.0)

# generate `compile_commands.json` for clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# =============================================================================
# Project Configuration
# =============================================================================

project(rust-to-cpp VERSION 0.1.0 LANGUAGES C CXX)
set(CMAKE_CXX_STANDARD 17)


# =============================================================================
# Rust Library Configuration
# =============================================================================
function(setup_rust_library targetname cratename rustlibname)
    # Fix Windows MSVC runtime library compatibility
    # Always use /MD (Release CRT) to match Rust and vcpkg Release libraries
    if(WIN32 AND MSVC)
        target_compile_options(${targetname} PRIVATE /MD)

        # Disable iterator debug level in Debug builds to match Release libraries
        # Use generator expression to work with multi-config generators like Visual Studio
        target_compile_definitions(${targetname} PRIVATE
            $<$<CONFIG:Debug>:_ITERATOR_DEBUG_LEVEL=0>
        )
    endif()

    # Set Rust build directories
    set(RUST_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)
    set(RUST_BUILD_TYPE release) # always use release build for compatibility
    set(RUST_LIB_DIR ${RUST_TARGET_DIR}/${RUST_BUILD_TYPE})

    # Ensure Rust library is built
    if(NOT EXISTS ${RUST_LIB_DIR})
        message(WARNING "Rust library not found. Building Rust library now...")
        execute_process(
            COMMAND cargo build --release
            WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        )
    endif()

    # Set Rust generated headers directory
    target_include_directories(${targetname} PRIVATE
        ${RUST_TARGET_DIR}/cxxbridge
        ${RUST_TARGET_DIR}/cxxbridge/rust
    )

    # Gather Rust generated source files
    file(GLOB rust_src
        "${RUST_TARGET_DIR}/cxxbridge/${cratename}/src/*.rs.cpp"
        "${RUST_TARGET_DIR}/cxxbridge/${cratename}/src/*.rs.cc"
    )

    if(rust_src)
        target_sources(${targetname} PRIVATE ${rust_src})
    endif()

    # Link Rust library
    if(NOT WIN32)
        set(rustlibname "lib${rustlibname}")
    endif()

    set(rust_lib "${RUST_LIB_DIR}/${rustlibname}${CMAKE_SHARED_LIBRARY_SUFFIX}")

    if(WIN32)
        set(rust_dll "${rust_lib}")
        set(rust_lib "${rust_lib}.lib")
    endif()

    if(NOT EXISTS ${rust_lib})
        message(FATAL_ERROR "Rust library not found at ${rust_lib}. Please build the Rust library.")
    endif()

    target_link_libraries(${targetname} PRIVATE "${rust_lib}")

    # link cxx library
    file(GLOB cxx_lib_files
        "${RUST_LIB_DIR}/build/cxx-*/out/*${CMAKE_STATIC_LIBRARY_SUFFIX}"
    )

    foreach(cxx_lib IN LISTS cxx_lib_files)
        target_link_libraries(${targetname} PRIVATE ${cxx_lib})
    endforeach()

    # Copy DLL on Windows
    if(WIN32)
        add_custom_command(TARGET ${targetname} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy
            "${rust_dll}"
            $<TARGET_FILE_DIR:${targetname}>
            COMMENT "Copying Rust DLL for $<CONFIG>."
        )
    endif()
endfunction()




# =============================================================================
# Target Configuration
# =============================================================================

set(targetname demo)
add_executable(${targetname} src/main.cpp)

# Setup Rust library build and linking
setup_rust_library(${targetname} "rust-to-cpp" "librust")
