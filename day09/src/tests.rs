#[cfg(test)]
mod tests {
    use crate::{solution_01, solution_02};

    #[test]
    fn example_01_01() {
        const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let result = solution_01(INPUT);

        assert_eq!(result, 114);
    }

    #[test]
    fn example_02_01() {
        const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let result = solution_02(INPUT);

        assert_eq!(result, 2);
    }
}
