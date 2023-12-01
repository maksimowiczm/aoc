cmake_minimum_required(VERSION 3.14)
project(aoc C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

IF (WIN32)
  file(WRITE null.cpp "")
  add_library(${PROJECT_NAME} null.cpp)
ELSE (UNIX)
  add_library(${PROJECT_NAME} /dev/null)
ENDIF ()

add_subdirectory(day01)

# testing
enable_testing()
target_link_libraries(
    ${PROJECT_NAME}
    GTest::gtest_main
)
include(GoogleTest)
