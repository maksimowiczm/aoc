use crate::day11::space::Space;
use crate::solution::SolveDay;
use std::str::FromStr;

mod space;

struct Day11;

impl SolveDay for Day11 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        Some(solution(input, 1).unwrap())
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        Some(solution(input, 100).unwrap())
    }
}

fn solution(input: &str, scale: usize) -> Result<u64, ()> {
    Space::from_str(input).map(|space| space.distance_between_galaxies_pairs(scale))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01_01() {
        let result = Day11::solve_part1(INPUT).unwrap();

        assert_eq!(result, 374);
    }

    #[test]
    fn example_02_01() {
        let result = solution(INPUT, 10).unwrap();

        assert_eq!(result, 1030);
    }

    #[test]
    fn example_02_02() {
        let result = Day11::solve_part2(INPUT).unwrap();

        assert_eq!(result, 8410);
    }

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
}
