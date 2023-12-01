#include "day01/lib.h"
#include "ctype.h"
#include "stdlib.h"

enum DigitState {
  NONE,
  FIRST,
};

typedef struct {
  enum DigitState state;
  size_t firstIndex;
  size_t lastIndex;
} Digit;

static size_t parse_and_add(const char *input, Digit *digit, uint64_t *sum) {
  char number[3] = {input[digit->firstIndex], input[digit->lastIndex]};
  char *endPtr;
  uint64_t value = strtoull(number, &endPtr, 10);

  if (endPtr == number) {
    return -1;
  }

  *sum += value;
  return 0;
}

uint64_t day01_result_01(const char *input, size_t n) {
  uint64_t sum = 0;
  Digit digit = {NONE,};

  for (size_t i = 0; i < n; i++) {
    const char character = input[i];

    if (isdigit(character)) {
      digit.lastIndex = i;
      if (digit.state == NONE) {
        digit.firstIndex = i;
        digit.state = FIRST;
      }
    } else if (isspace(character)) {
      digit.state = NONE;
      if (parse_and_add(input, &digit, &sum)) {
        return 0;
      }
    }
  }

  if (digit.state == FIRST) {
    digit.state = NONE;
    if (parse_and_add(input, &digit, &sum)) {
      return 0;
    }
  }

  return sum;
}
