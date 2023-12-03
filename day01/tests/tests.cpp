extern "C" {
#include "day01/lib.h"
}

#include "gtest/gtest.h"

TEST(DAY01, trash) {
  const std::string input = "abc\n";

  const auto output = day01_result_01(input.c_str(), input.length());

  EXPECT_EQ(output, 0ull);
}

TEST(DAY01, example_01) {
  const std::string input = "1abc2\n"
                            "pqr3stu8vwx\n"
                            "a1b2c3d4e5f\n"
                            "treb7uchet";

  const auto output = day01_result_01(input.c_str(), input.length());

  EXPECT_EQ(output, 142ull);
}

TEST(DAY01, example_02) {
  const std::string input = "two1nine\n"
                            "eightwothree\n"
                            "abcone2threexyz\n"
                            "xtwone3four\n"
                            "4nineeightseven2\n"
                            "zoneight234\n"
                            "7pqrstsixteen";

  const auto output = day01_result_02(input.c_str(), input.length());

  EXPECT_EQ(output, 281ull);
}
