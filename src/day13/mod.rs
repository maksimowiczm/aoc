use crate::day13::lava_pattern::{LavaPattern, Mirror};
use crate::solution::SolveDay;
use std::collections::HashMap;
use std::str::FromStr;

mod lava_pattern;

struct Day13;

impl SolveDay for Day13 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let patterns = input
            .split("\n\n")
            .flat_map(LavaPattern::from_str)
            .collect::<Vec<_>>();

        let cols = patterns.iter().fold(0, |acc, pattern| {
            let positions = pattern.mirror_position().unwrap();
            if let Some(&(_, res)) = positions.iter().nth(0) {
                assert_eq!(positions.len(), 1);
                acc + res
            } else {
                acc
            }
        });
        let result = patterns
            .iter()
            .flat_map(LavaPattern::rotate_90)
            .fold(cols, |acc, pattern| {
                let positions = pattern.mirror_position().unwrap();
                if let Some(&(_, res)) = positions.iter().nth(0) {
                    assert_eq!(positions.len(), 1);
                    acc + res * 100
                } else {
                    acc
                }
            });

        Some(result as u64)
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let patterns = input
            .split("\n\n")
            .flat_map(LavaPattern::from_str)
            .collect::<Vec<_>>();

        let mut map = HashMap::new();

        let result_cols = patterns
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.fix_smudge()))
            .filter(|(_, p)| p.is_some())
            .map(|(i, p)| (i, p.unwrap()))
            .fold(0, |acc, (i, (_, res))| {
                assert!(!map.contains_key(&i));
                map.insert(i, res);
                acc + res
            });

        let result_rows = patterns
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.rotate_90().unwrap().fix_smudge()))
            .filter(|(_, p)| p.is_some())
            .map(|(i, p)| (i, p.unwrap()))
            .fold(0, |acc, (i, (_, res))| {
                assert!(!map.contains_key(&i));
                map.insert(i, res);
                acc + res * 100
            });

        Some(result_cols as u64 + result_rows as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_01_01() {
        const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let result = Day13::solve_part1(INPUT).unwrap();
        assert_eq!(result, 405);
    }

    #[test]
    fn example_01_02() {
        const INPUT: &str = "##..##..##.
######..###
.####.##.##
..........#
.####.##.##
.####....##
..##..##..#
";

        let result = Day13::solve_part1(INPUT).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn example_01_03() {
        const INPUT: &str = "..#.#.####.#.#.
##.####..####.#
##.#........#.#
..#....##......
#####......####
..#.#.####.#.#.
..#.#.#..#.#.#.
...##..##..##..
..#.#......#.#.
";

        let result = Day13::solve_part1(INPUT).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    #[ignore]
    fn example_02_01() {
        const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

        let result = Day13::solve_part2(INPUT).unwrap();
        assert_eq!(result, 400);
    }

    #[test]
    #[ignore]
    fn example_02_02() {
        const INPUT: &str = ".#####.##..
##...##.#..
##.##..####
#...##....#
.#....#####
.#....#####
#...##....#
##.#...####
##...##.#..
.#####.##..
#####..##..
#.###....#.
#.###....#.
#####..##..
.#####.##..
";

        let result = Day13::solve_part2(INPUT).unwrap();
        assert_eq!(result, 400);
    }
}
