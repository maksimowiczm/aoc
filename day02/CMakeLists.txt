add_library(
    day02
    part1.c
    internal.c
)

target_include_directories(day02 PUBLIC include)

add_subdirectory(tests)
