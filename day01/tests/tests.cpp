extern "C" {
#include "day01/lib.h"
}

#include "gtest/gtest.h"

TEST(DAY01_TESTS, trash) {
  const std::string input = "abc\n";

  const auto output = day01_result(input.c_str(), input.length());

  EXPECT_EQ(output, 0ull);
}

TEST(DAY01_TESTS, example_01) {
  const std::string input = "1abc2\n"
                            "pqr3stu8vwx\n"
                            "a1b2c3d4e5f\n"
                            "treb7uchet";

  const auto output = day01_result(input.c_str(), input.length());

  EXPECT_EQ(output, 142ull);
}

#include "input_01.hpp"

TEST(DAY01_TESTS, puzzle_01) {
  const std::string input(INPUT);

  const auto output = day01_result(input.c_str(), input.length());

  EXPECT_EQ(output, 56108ull);
}