#include "day02/game.h"

#include "string.h"

int parse_game(const char *input, size_t n, Game *output) {
  size_t set = 0;
  for (size_t i = 0; i < n; i++) {
    if (input[i] == ';') {
      set++;
    }

    uint64_t count = 0;
    char color[6] = {0};
    sscanf(input + i, "%llu %5s", &count, color);
    if (count != 0) {
      if (strncmp(color, "red", 3) == 0) {
        if (output[set].red == 0) {
          output[set].red = count;
        }
      } else if (strncmp(color, "green", 5) == 0) {
        if (output[set].green == 0) {
          output[set].green = count;
        }
      } else if (strncmp(color, "blue", 4) == 0) {
        if (output[set].blue == 0) {
          output[set].blue = count;
        }
      }
    }
  }

  return 0;
}
