use crate::solution::SolveDay;
use dish::Dish;

mod dish;

struct Day14;

impl SolveDay for Day14 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let dish = input.parse::<Dish>().ok()?;

        dish.tilt_north_load().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01_01() {
        const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

        let result = Day14::solve_part1(INPUT).unwrap();

        assert_eq!(result, 136);
    }
}
