cmake_minimum_required(VERSION 3.27)
project(day02 C CXX)

add_library(
    day02
    STATIC
    part1.c
    internal.c
    part2.c
)

install(TARGETS day02 DESTINATION .)

target_include_directories(day02 PUBLIC include)

enable_testing()
target_link_libraries(
    ${PROJECT_NAME}
    GTest::gtest_main
)
include(GoogleTest)
add_subdirectory(tests)