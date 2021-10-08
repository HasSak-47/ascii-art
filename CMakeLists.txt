cmake_minimum_required(VERSION 3.2)
project(tree_maker)

file(GLOB SOURCES "src/*.cpp" "src/*.c" "src/lib/*.c" "src/lib/*.cpp")
add_executable(trees ${SOURCES})