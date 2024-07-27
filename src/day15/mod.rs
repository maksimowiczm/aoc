use book::Book;

use crate::day15::label::Label;
use crate::day15::lens::LensOperation;
use crate::solution::SolveDay;
use std::str::FromStr;

mod book;
mod label;
mod lens;

struct Day15;

impl SolveDay for Day15 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let sequence = input
            .split(",")
            .flat_map(Label::from_str)
            .collect::<Vec<_>>()
            .iter()
            .map(Label::hash)
            .sum();

        Some(sequence)
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let sequence = input
            .split(",")
            .flat_map(LensOperation::from_str)
            .collect::<Vec<_>>();

        let mut book = Book::new();
        book.execute_sequence(&sequence);
        Some(book.get_focusing_power())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01_01() {
        const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = Day15::solve_part1(INPUT).unwrap();

        assert_eq!(result, 1320);
    }

    #[test]
    fn example_02_01() {
        const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = Day15::solve_part2(INPUT).unwrap();

        assert_eq!(result, 145);
    }
}
