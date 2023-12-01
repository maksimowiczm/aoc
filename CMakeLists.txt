cmake_minimum_required(VERSION 3.14)
project(aoc C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

add_executable(
    ${PROJECT_NAME}
    main.cpp
)

add_subdirectory(day01)
