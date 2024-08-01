use crate::solution::SolveDay;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day21;

impl SolveDay for Day21 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        Some(part1(input, 64))
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        Some(part2(input))
    }
}

fn part1(input: &str, step_count: usize) -> usize {
    let garden = parse(input);

    let mut current = HashSet::new();
    current.insert(garden.start);

    for _ in 0..step_count {
        let mut local = HashSet::new();

        for pos in current.into_iter() {
            for neighbour in pos.get_neighbours(&garden) {
                local.insert(neighbour);
            }
        }

        current = local;
    }

    current.len()
}

fn part2(input: &str) -> usize {
    println!(
        "https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21"
    );
    println!("hardcoded for input");

    let garden = parse(input);

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(garden.start);

    // bfs
    while let Some(current) = queue.pop_front() {
        if visited.contains_key(&(current.col, current.row)) {
            continue;
        }

        visited.insert((current.col, current.row), current.step);

        for neighbour in current.get_neighbours(&garden) {
            queue.push_back(neighbour);
        }
    }

    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    const N: usize = 202300;

    // count squares
    let odd = (N + 1).pow(2);
    let even = N.pow(2);

    // count corners
    let corners_odd = N + 1;
    let corners_even = N;

    // count tiles
    let tiles_odd = visited.values().filter(|v| *v % 2 == 1).count();
    let tiles_even = visited.values().filter(|v| *v % 2 == 0).count();

    let tiles_corners_odd = visited.values().filter(|v| *v % 2 == 1 && **v > 65).count();
    let tiles_corners_even = visited.values().filter(|v| *v % 2 == 0 && **v > 65).count();

    odd * tiles_odd + even * tiles_even - corners_odd * tiles_corners_odd
        + corners_even * tiles_corners_even
}

fn parse(input: &str) -> Garden {
    let mut start = None;
    let garden = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Cell::Plot,
                    '#' => Cell::Rock,
                    'S' => {
                        start = Some(Position { row, col, step: 0 });
                        Cell::Plot
                    }
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();

    Garden {
        garden,
        start: start.unwrap(),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Position {
    row: usize,
    col: usize,
    step: usize,
}

#[derive(Copy, Clone)]
enum Cell {
    Rock,
    Plot,
}

struct Garden {
    garden: Vec<Vec<Cell>>,
    start: Position,
}

impl Garden {
    fn cell_type(&self, pos: Position) -> Option<Cell> {
        self.garden
            .get(pos.row)
            .and_then(|row| row.get(pos.col).copied())
    }
}

impl Position {
    fn get_neighbours(&self, garden: &Garden) -> Vec<Position> {
        [
            (Some(self.row), self.col.checked_add(1)),
            (Some(self.row), self.col.checked_sub(1)),
            (self.row.checked_add(1), Some(self.col)),
            (self.row.checked_sub(1), Some(self.col)),
        ]
        .into_iter()
        .flat_map(|(row, col)| {
            row.and_then(|row| {
                col.map(|col| Position {
                    row,
                    col,
                    step: self.step + 1,
                })
            })
        })
        .filter(|pos| matches!(garden.cell_type(*pos), Some(Cell::Plot)))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn example_01() {
        assert_eq!(part1(INPUT, 6), 16);
    }
}
