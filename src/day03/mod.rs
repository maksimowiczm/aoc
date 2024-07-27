use crate::day03::board::Board;
use crate::solution::SolveDay;

mod board;

struct Day03;

impl SolveDay for Day03 {
    type Part1 = u128;
    type Part2 = u128;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let board: Board = input.into();

        Some(
            board
                .valid_parts()
                .iter()
                .fold(0, |acc, v| acc + *v as u128),
        )
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let board: Board = input.into();

        Some(
            board
                .valid_gears()
                .iter()
                .fold(0, |acc, v| acc + *v as u128),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = Day03::solve_part1(input).unwrap();

        assert_eq!(result, 4361);
    }

    #[test]
    fn my_example_01() {
        let input = "*123....15#...";

        let result = Day03::solve_part1(input).unwrap();

        assert_eq!(result, 123 + 15);
    }

    #[test]
    fn example_02() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = Day03::solve_part2(input).unwrap();

        assert_eq!(result, 467835);
    }
}
