#include "inttypes.h"
#include "stdio.h"

typedef struct {
  uint64_t red;
  uint64_t green;
  uint64_t blue;
} Game;

int parse_game(const char *input, size_t n, Game *output);
