#include "day02/solution.h"
#include "day02/internal.h"

static uint64_t solution_02(uint64_t game_id, Game *games, size_t games_count) {
  Game min = {0};

  for (size_t i = 0; i < games_count; i++) {
    Game game = games[i];
    if (game.red > min.red) {
      min.red = game.red;
    }
    if (game.green > min.green) {
      min.green = game.green;
    }
    if (game.blue > min.blue) {
      min.blue = game.blue;
    }
  }

  return min.red * min.green * min.blue;
}

uint64_t day02_solution_02(const char *input, size_t n) {
  return solution(input, n, solution_02);
}
