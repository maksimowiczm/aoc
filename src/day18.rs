use std::str::FromStr;
use std::vec::IntoIter;

use crate::day18::MoveParseError::*;
use crate::solution::SolveDay;

pub struct Day18;

impl SolveDay for Day18 {
    type Part1 = i32;
    type Part2 = i64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let moves: IntoIter<Move> = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()
            .expect("moves")
            .into_iter();

        let vertices = moves
            .scan(Position { x: 0f64, y: 0f64 }, |current, mv| {
                match mv {
                    Move::Up(distance) => current.y += distance as f64,
                    Move::Down(distance) => current.y -= distance as f64,
                    Move::Left(distance) => current.x -= distance as f64,
                    Move::Right(distance) => current.x += distance as f64,
                }
                Some(*current)
            })
            .collect::<Vec<_>>();

        let polygon = Polygon::from_vertices(vertices);

        // this is hack, https://www.reddit.com/r/adventofcode/comments/18l8mao/2023_day_18_intuition_for_why_spoiler_alone/
        Some(polygon.area() as i32 + polygon.perimeter() as i32 / 2 + 1)
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let moves: IntoIter<HexMove> = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()
            .expect("moves")
            .into_iter();

        let vertices = moves
            .scan(Position { x: 0f64, y: 0f64 }, |current, mv| {
                match mv {
                    HexMove::Up(distance) => current.y += distance as f64,
                    HexMove::Down(distance) => current.y -= distance as f64,
                    HexMove::Left(distance) => current.x -= distance as f64,
                    HexMove::Right(distance) => current.x += distance as f64,
                }
                Some(*current)
            })
            .collect::<Vec<_>>();

        let polygon = Polygon::from_vertices(vertices);

        // this is hack, https://www.reddit.com/r/adventofcode/comments/18l8mao/2023_day_18_intuition_for_why_spoiler_alone/
        Some(polygon.area() as i64 + polygon.perimeter() as i64 / 2 + 1)
    }
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: f64,
    y: f64,
}

struct Polygon {
    vertices: Vec<Position>,
}

impl Polygon {
    fn from_vertices(vertices: Vec<Position>) -> Self {
        Self { vertices }
    }

    fn area(&self) -> f64 {
        assert!(self.vertices.len() >= 3);
        // assume these don't have self-intersections
        let mut iter = self.vertices.iter().peekable();
        let initial = self.vertices.first().unwrap();
        let mut area = 0.0;
        while let Some(vertex) = iter.next() {
            let next = iter.peek().unwrap_or(&initial);
            area += vertex.x * next.y - vertex.y * next.x;
        }

        area.abs() / 2.0
    }

    fn perimeter(&self) -> f64 {
        assert!(self.vertices.len() >= 3);
        let mut iter = self.vertices.iter().peekable();
        let initial = self.vertices.first().unwrap();
        let mut perimeter = 0.0;
        while let Some(vertex) = iter.next() {
            let next = iter.peek().unwrap_or(&initial);
            perimeter += (vertex.x - next.x).hypot(vertex.y - next.y);
        }

        perimeter
    }
}

enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

enum HexMove {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

#[allow(dead_code)]
#[derive(Debug)]
enum MoveParseError {
    EmptyDirection,
    InvalidDirection,
    EmptyDistance,
    InvalidDistance(std::num::ParseIntError),
    EmptyColor,
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let direction = match split.next().ok_or(EmptyDirection)? {
            "U" => Move::Up,
            "D" => Move::Down,
            "L" => Move::Left,
            "R" => Move::Right,
            _ => return Err(InvalidDirection),
        };

        let distance = split
            .next()
            .ok_or(EmptyDistance)?
            .parse()
            .map_err(InvalidDistance)?;

        _ = split.next().ok_or(EmptyColor)?;

        Ok(direction(distance))
    }
}

impl FromStr for HexMove {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ').skip(2);

        let hex = split.next().ok_or(EmptyColor)?;

        let mut iter = hex.chars().skip(2);

        let distance = i64::from_str_radix(&iter.clone().take(5).collect::<String>(), 16)
            .map_err(InvalidDistance)?;

        let result = match iter.nth(5) {
            Some('0') => HexMove::Right(distance),
            Some('1') => HexMove::Down(distance),
            Some('2') => HexMove::Left(distance),
            Some('3') => HexMove::Up(distance),
            _ => return Err(InvalidDirection),
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn example_01() {
        let result = Day18::solve_part1(INPUT).unwrap();
        assert_eq!(result, 62);
    }

    #[test]
    fn example_02() {
        let result = Day18::solve_part2(INPUT).unwrap();
        assert_eq!(result, 952408144115);
    }
}
