#include "day02/solution.h"
#include "day02/internal.h"

static uint64_t solution_01(uint64_t game_id, Game *games, size_t games_count) {
  Game max = {12, 13, 14};

  for (size_t i = 0; i < games_count; i++) {
    Game game = games[i];

    if (game.red > max.red) {
      return 0;
    } else if (game.green > max.green) {
      return 0;
    } else if (game.blue > max.blue) {
      return 0;
    }
  }

  return game_id;
}

uint64_t day02_solution_01(const char *input, size_t n) {
  return solution(input, n, solution_01);
}
