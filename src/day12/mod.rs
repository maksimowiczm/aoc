use crate::day12::springs::{Arrangements, FromFolds, Springs};
use crate::solution::SolveDay;
use std::str::FromStr;

mod springs;

struct Day12;

impl SolveDay for Day12 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        Some(Springs::from_str(input).ok()?.arrangements_bogo_count())
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        Some(
            Springs::from_folds(input, 5)
                .ok()?
                .arrangements_bogo_count(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01_01() {
        const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = Day12::solve_part1(INPUT).unwrap();
        assert_eq!(result, 21);
    }

    #[ignore]
    #[test]
    fn example_02_01() {
        const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = Day12::solve_part2(INPUT).unwrap();
        assert_eq!(result, 525152);
    }
}
