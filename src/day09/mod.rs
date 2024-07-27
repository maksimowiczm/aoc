use crate::day09::sequence::Sequence;
use crate::solution::SolveDay;
use std::str;
use str::FromStr;

mod sequence;

struct Day09;

impl SolveDay for Day09 {
    type Part1 = i64;
    type Part2 = i64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let lines = parse_input::<i64>(input);
        let predictions = lines
            .iter()
            .map(|line| Sequence::from(line.clone()).predict_forward())
            .collect::<Vec<_>>();

        Some(predictions.iter().fold(0, |acc, v| acc + v))
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let lines = parse_input::<i64>(input);
        let predictions = lines
            .iter()
            .map(|line| Sequence::from(line.clone()).predict_backward())
            .collect::<Vec<_>>();

        Some(predictions.iter().fold(0, |acc, v| acc + v))
    }
}

fn parse_input<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
{
    input
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(" ")
                .flat_map(|v| v.parse::<T>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01_01() {
        const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let result = Day09::solve_part1(INPUT).unwrap();

        assert_eq!(result, 114);
    }

    #[test]
    fn example_02_01() {
        const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let result = Day09::solve_part2(INPUT).unwrap();

        assert_eq!(result, 2);
    }
}
