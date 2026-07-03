# Build and run the C and C++ examples against the wickra-xray C ABI.
#
#   cargo build --release -p wickra-xray-c
#   cmake -S examples/c -B examples/c/build
#   cmake --build examples/c/build --config Release
#   ctest --test-dir examples/c/build -C Release --output-on-failure
#
# WICKRA_XRAY_LIB_DIR defaults to the workspace target/release directory and can
# be overridden for out-of-tree builds.

cmake_minimum_required(VERSION 3.15)
project(wickra_xray_examples C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WICKRA_XRAY_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    CACHE PATH "Directory containing the built wickra-xray C ABI library")

set(WICKRA_XRAY_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/../../bindings/c/include")

find_library(WICKRA_XRAY_LIB
    NAMES wickra_xray libwickra_xray
    HINTS "${WICKRA_XRAY_LIB_DIR}"
    REQUIRED)

add_executable(frame frame.c)
target_include_directories(frame PRIVATE "${WICKRA_XRAY_INCLUDE}")
target_link_libraries(frame PRIVATE "${WICKRA_XRAY_LIB}")

add_executable(framepp frame.cpp)
target_include_directories(framepp PRIVATE "${WICKRA_XRAY_INCLUDE}")
target_link_libraries(framepp PRIVATE "${WICKRA_XRAY_LIB}")

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example frame framepp)
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_XRAY_LIB_DIR}/wickra_xray.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
add_test(NAME frame COMMAND frame)
add_test(NAME framepp COMMAND framepp)
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(frame framepp PROPERTIES TIMEOUT 60)
