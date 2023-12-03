cmake_minimum_required(VERSION 3.27)
project(day01 C CXX)

add_library(
    day01
    STATIC
    part1.c
    part2.c
)

install(TARGETS day01 DESTINATION .)

target_include_directories(day01 PUBLIC include)

enable_testing()
target_link_libraries(
    ${PROJECT_NAME}
    GTest::gtest_main
)
include(GoogleTest)
add_subdirectory(tests)