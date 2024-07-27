use std::fmt::Display;

#[allow(dead_code)]
pub trait SolveDay {
    type Part1: Display;
    type Part2: Display;

    #[allow(unused_variables)]
    fn solve_part1(input: &str) -> Option<Self::Part1> {
        None
    }

    #[allow(unused_variables)]
    fn solve_part2(input: &str) -> Option<Self::Part2> {
        None
    }
}
