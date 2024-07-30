use crate::solution::SolveDay;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

pub struct Day17;

impl SolveDay for Day17 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let city = input.parse::<City>().unwrap();

        assert!(!city.blocks.is_empty());
        assert!(!city.blocks[0].is_empty());

        let mut queue = BinaryHeap::new();
        queue.push(Cell {
            cost: city.blocks[0][1],
            position: Position { row: 0, col: 1 },
            direction: Direction::Right,
            moves: 1,
        });
        queue.push(Cell {
            cost: city.blocks[1][0],
            position: Position { row: 1, col: 0 },
            direction: Direction::Down,
            moves: 1,
        });
        let mut visited = HashSet::new();

        while let Some(cell) = queue.pop() {
            if cell.position.row == city.blocks.len() - 1
                && cell.position.col == city.blocks[0].len() - 1
            {
                return Some(cell.cost);
            }

            for neighbour in cell.neighbours(&city) {
                if visited.insert(neighbour.stateless()) {
                    queue.push(neighbour);
                }
            }
        }

        unreachable!();
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn move_in_direction(&self, direction: &Direction, city: &City) -> Option<Position> {
        let result = match direction {
            Direction::Up => Position {
                row: self.row.checked_sub(1)?,
                col: self.col,
            },
            Direction::Down => Position {
                row: self.row.checked_add(1)?,
                col: self.col,
            },
            Direction::Left => Position {
                row: self.row,
                col: self.col.checked_sub(1)?,
            },
            Direction::Right => Position {
                row: self.row,
                col: self.col.checked_add(1)?,
            },
        };

        if result.row >= city.blocks.len() || result.col >= city.blocks[0].len() {
            return None;
        }

        Some(result)
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite_of(&self, direction: &Direction) -> bool {
        match self {
            Direction::Up => direction == &Direction::Down,
            Direction::Down => direction == &Direction::Up,
            Direction::Left => direction == &Direction::Right,
            Direction::Right => direction == &Direction::Left,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Stateless {
    position: Position,
    direction: Direction,
    moves: usize,
}

struct Cell {
    cost: u32,
    position: Position,
    direction: Direction,
    moves: usize,
}

impl Cell {
    fn stateless(&self) -> Stateless {
        Stateless {
            position: self.position,
            direction: self.direction,
            moves: self.moves,
        }
    }

    fn neighbours(&self, city: &City) -> Vec<Cell> {
        let mut neighbours = vec![];

        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if self.moves >= 3 && self.direction == direction {
                continue;
            }

            if self.direction.opposite_of(&direction) {
                continue;
            }

            let next_position = match self.position.move_in_direction(&direction, city) {
                Some(position) => position,
                None => continue,
            };
            let moves = if self.direction == direction {
                self.moves + 1
            } else {
                1
            };

            neighbours.push(Cell {
                cost: self.cost + city.blocks[next_position.row][next_position.col],
                position: next_position,
                direction,
                moves,
            });
        }

        neighbours
    }
}

impl Eq for Cell {}

impl PartialEq<Self> for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl PartialOrd<Self> for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.cost.cmp(&other.cost)
        // max-heap (reverse order)
        other.cost.cmp(&self.cost)
    }
}

#[derive(Debug, Clone)]
struct City {
    blocks: Vec<Vec<u32>>,
}

impl FromStr for City {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let blocks = input
            .trim()
            .split('\n')
            .map(|row| {
                row.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(City { blocks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let result = Day17::solve_part1(INPUT).unwrap();

        assert_eq!(result, 102);
    }

    #[test]
    fn my_example_01() {
        const INPUT: &str = "
154
552
465";

        let result = Day17::solve_part1(INPUT).unwrap();

        assert_eq!(result, 16);
    }
}
