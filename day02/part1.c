#include "day02/solution.h"
#include "day02/game.h"

#include "stdio.h"
#include "stdlib.h"
#include "string.h"

static uint64_t process_line(const char *line, size_t n, Game max) {
  uint64_t game_id = 0;
  sscanf(line + 4, "%llu", &game_id);

  size_t start = 0;
  for (size_t i = 5; i < n; i++) {
    if (line[i] == ':') {
      start = i;
      break;
    }
  }

  size_t games_count = 1;
  for (size_t i = start; i < n; i++) {
    if (line[i] == ';') {
      games_count++;
    }
  }

  Game *games = malloc(sizeof(Game) * games_count);
  memset(games, 0, sizeof(Game) * games_count);
  if (parse_game(line + start + 1, n - start, games)) {
    return 0;
  }

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
  Game max = {12, 13, 14};
  uint64_t result = 0;
  size_t startLine = 0;
  for (size_t i = 0; i < n; i++) {
    const char character = input[i];
    if (character == '\n' || character == '\r') {
      result += process_line(input + startLine, i - startLine, max);
      startLine = i + 1;
    }
  }

  result += process_line(input + startLine, n - startLine, max);

  return result;
}
