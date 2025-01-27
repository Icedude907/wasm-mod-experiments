cmake_minimum_required(VERSION 3.31)

# Cross compiling with clang - make sure everything is set up correctly
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR wasm32)
set(CLANG_TARGET_TRIPLE wasm32-unknown-unknown)
set(CMAKE_ASM_COMPILER clang)
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_ASM_COMPILER_TARGET ${CLANG_TARGET_TRIPLE})
set(CMAKE_C_COMPILER_TARGET ${CLANG_TARGET_TRIPLE})
set(CMAKE_CXX_COMPILER_TARGET ${CLANG_TARGET_TRIPLE})
set(CMAKE_EXECUTABLE_SUFFIX_CXX ".wasm") # IDK why this has to be overridden per-language. It should be global.
# set(CMAKE_ASM_FLAGS "")
# set(CMAKE_C_FLAGS "")
set(CMAKE_CXX_FLAGS "-fno-exceptions -fvisibility=hidden -mbulk-memory")
set(CMAKE_EXE_LINKER_FLAGS "-nostdlib -nostartfiles -Wl,--no-entry -Wl,--strip-all, -Wl,-z,stack-size=65536") # -v
# set(CMAKE_VERBOSE_MAKEFILE on)

# Actual CMake stuff
# --------------------------------
project(cpp_wasm VERSION 1.0)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED True)

file(GLOB srcs CONFIGURE_DEPENDS "*.cpp")
add_executable(cpp_wasm_mod ${srcs})
target_include_directories(cpp_wasm_mod PRIVATE "./include/")
target_link_options(cpp_wasm_mod PUBLIC LINKER:-Map=cpp_wasm_mod.map)