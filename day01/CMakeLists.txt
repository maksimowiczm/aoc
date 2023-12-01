add_library(
    day01
    part1.c
    part2.c
)

target_include_directories(day01 PUBLIC include)

add_subdirectory(tests)