#[cfg(test)]
mod tests {
    use crate::solution_01;

    #[test]
    fn example_01() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";

        let result = solution_01(input);

        assert_eq!(result, 288);
    }
}
