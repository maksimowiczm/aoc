add_library(
    day01
    lib.c
)

target_include_directories(day01 PUBLIC include)

add_subdirectory(tests)