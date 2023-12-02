#include "day02/internal.h"
#include "stdlib.h"
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

uint64_t process_line(const char *line, size_t n, uint64_t(*process_games)(uint64_t, Game *, size_t)) {
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

  uint64_t result = process_games(game_id, games, games_count);
  free(games);
  return result;
}

uint64_t solution(const char *input, size_t n, uint64_t (*process_games)(uint64_t, Game *, size_t)) {
  uint64_t result = 0;
  size_t startLine = 0;
  for (size_t i = 0; i < n; i++) {
    const char character = input[i];
    if (character == '\n' || character == '\r') {
      result += process_line(input + startLine, i - startLine, process_games);
      startLine = i + 1;
    }
  }

  result += process_line(input + startLine, n - startLine, process_games);
  return result;
}