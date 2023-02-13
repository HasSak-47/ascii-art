cmake_minimum_required(VERSION 3.0.0)
project(ascii-art VERSION 0.1.0)

include(CTest)
enable_testing()


include_directories("src/headers" "dependecies/headers")
file(GLOB COMPILATION_UNITS "src/*.cpp" "src/*.c" "src/lib/*.c" "src/lib/*.cpp" "dependecies/lib/*.cpp" "dependecies/lib/*.c")
add_executable(ascii-art src/main.cpp ${COMPILATION_UNITS})

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
