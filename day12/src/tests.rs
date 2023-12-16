#[cfg(test)]
mod tests {
    use crate::solution_01;

    #[test]
    fn example_01_01() {
        const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = solution_01(INPUT);
        assert_eq!(result, Ok(21));
    }
}
