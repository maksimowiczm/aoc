#include "inttypes.h"
#include "stdio.h"

typedef struct {
  uint64_t red;
  uint64_t green;
  uint64_t blue;
} Game;

int parse_game(const char *input, size_t n, Game *output);

uint64_t process_line(const char *line, size_t n, uint64_t(*process_games)(uint64_t, Game *, size_t));

uint64_t solution(const char *input, size_t n, uint64_t (*process_games)(uint64_t, Game *, size_t));
