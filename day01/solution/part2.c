#include "day01/lib.h"
#include "ctype.h"
#include "stdlib.h"
#include "string.h"

typedef struct {
  const char *word;
  char numeral;
  size_t length;
} Digit;

static const Digit digits[] = {
    {"one", '1', 3},
    {"two", '2', 3},
    {"three", '3', 5},
    {"four", '4', 4},
    {"five", '5', 4},
    {"six", '6', 3},
    {"seven", '7', 5},
    {"eight", '8', 5},
    {"nine", '9', 4},
    {NULL, 0, 0}  // sentinel to mark the end
};

static char parse_digit(const char *input, size_t n) {
  for (const Digit *digit = digits; digit->word != NULL; digit++) {
    if (n > digit->length && strncmp(input, digit->word, digit->length) == 0) {
      return digit->numeral;
    }
  }

  return 0;
}

static ssize_t parse_digits(const char *input, size_t n, char *output, size_t n_output) {
  memset(output, 0, n_output);
  ssize_t pos = 0;
  for (size_t i = 0; i < n; i++) {
    if (pos >= n_output) {
      return -1;
    }

    const char character = input[i];
    if (isdigit(character) || isspace(character)) {
      output[pos++] = character;
      continue;
    }

    char digit = parse_digit(input + i, n - i);
    if (digit != 0) {
      output[pos++] = digit;
    }
  }

  return pos;
}

uint64_t day01_result_02(const char *input, size_t n) {
  char *buffer = malloc(n);
  size_t size = parse_digits(input, n, buffer, n);
  if (size == -1) {
    free(buffer);
    return 0;
  }

  uint64_t result = day01_result_01(buffer, size);
  free(buffer);
  return result;
}