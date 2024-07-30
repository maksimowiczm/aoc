use crate::solution::SolveDay;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

pub struct Day17;

fn solution(
    city: City,
    mut queue: BinaryHeap<Cell>,
    target: impl Target,
    neighbor: impl NeighboursVisitor,
) -> u32 {
    assert!(!city.blocks.is_empty());
    assert!(!city.blocks[0].is_empty());
    assert!(!queue.is_empty());

    let mut visited = HashSet::new();

    while let Some(cell) = queue.pop() {
        if cell.is_target(&city, &target) {
            return cell.cost;
        }

        for neighbour in cell.neighbours(&city, &neighbor) {
            if visited.insert(neighbour.vertex()) {
                queue.push(neighbour);
            }
        }
    }

    unreachable!();
}

impl SolveDay for Day17 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let city = input.parse::<City>().unwrap();
        let mut queue = BinaryHeap::new();
        struct Part1;
        impl Target for Part1 {
            fn is_target(&self, cell: &Cell, city: &City) -> bool {
                cell.position.row == city.blocks.len() - 1
                    && cell.position.col == city.blocks[0].len() - 1
            }
        }
        impl NeighboursVisitor for Part1 {
            fn get_neighbours(&self, cell: &Cell, city: &City) -> Vec<Cell> {
                let mut neighbours = vec![];

                for direction in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    if cell.moves >= 3 && cell.direction == direction {
                        continue;
                    }

                    if cell.direction.opposite_of(&direction) {
                        continue;
                    }

                    let next_position = match cell.position.move_in_direction(&direction, city) {
                        Some(position) => position,
                        None => continue,
                    };
                    let moves = if cell.direction == direction {
                        cell.moves + 1
                    } else {
                        1
                    };

                    neighbours.push(Cell {
                        cost: cell.cost + city.blocks[next_position.row][next_position.col],
                        position: next_position,
                        direction,
                        moves,
                    });
                }

                neighbours
            }
        }

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

        Some(solution(city, queue, Part1, Part1))
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let city = input.parse::<City>().unwrap();
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
        struct Part2;
        impl Target for Part2 {
            fn is_target(&self, cell: &Cell, city: &City) -> bool {
                if cell.moves < 4 {
                    return false;
                }

                cell.position.row == city.blocks.len() - 1
                    && cell.position.col == city.blocks[0].len() - 1
            }
        }
        impl NeighboursVisitor for Part2 {
            fn get_neighbours(&self, cell: &Cell, city: &City) -> Vec<Cell> {
                let mut neighbours = vec![];

                for direction in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    if cell.moves >= 10 && cell.direction == direction {
                        continue;
                    }

                    if cell.direction.opposite_of(&direction) {
                        continue;
                    }

                    if cell.moves < 4 && cell.direction != direction {
                        continue;
                    }

                    let next_position = match cell.position.move_in_direction(&direction, city) {
                        Some(position) => position,
                        None => continue,
                    };
                    let moves = if cell.direction == direction {
                        cell.moves + 1
                    } else {
                        1
                    };

                    neighbours.push(Cell {
                        cost: cell.cost + city.blocks[next_position.row][next_position.col],
                        position: next_position,
                        direction,
                        moves,
                    });
                }

                neighbours
            }
        }

        Some(solution(city, queue, Part2, Part2))
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
struct Vertex {
    position: Position,
    direction: Direction,
    moves: usize,
}

trait Target {
    fn is_target(&self, cell: &Cell, city: &City) -> bool;
}

trait NeighboursVisitor {
    fn get_neighbours(&self, cell: &Cell, city: &City) -> Vec<Cell>;
}

struct Cell {
    cost: u32,
    position: Position,
    direction: Direction,
    moves: usize,
}

impl Cell {
    fn vertex(&self) -> Vertex {
        Vertex {
            position: self.position,
            direction: self.direction,
            moves: self.moves,
        }
    }

    fn neighbours(&self, city: &City, visitor: &impl NeighboursVisitor) -> Vec<Cell> {
        visitor.get_neighbours(self, city)
    }

    fn is_target(&self, city: &City, visitor: &impl Target) -> bool {
        visitor.is_target(self, city)
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

    #[test]
    fn example_02() {
        const INPUT: &str = "111111111111
999999999991
999999999991
999999999991
999999999991
";
        let result = Day17::solve_part2(INPUT).unwrap();
        assert_eq!(result, 71);
    }
}
