extern "C" {
#include "day02/solution.h"
}

#include "gtest/gtest.h"

TEST(DAY01_TESTS, example_01) {
  const std::string input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n"
                            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n"
                            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n"
                            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n"
                            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

  const auto output = day02_solution_01(input.c_str(), input.length());

  EXPECT_EQ(output, 8ull);
}

#include "input.hpp"

TEST(DAY01_TESTS, puzzle_01) {
  const std::string input{INPUT};

  const auto output = day02_solution_01(input.c_str(), input.length());

  EXPECT_EQ(output, 2207ull);
}
