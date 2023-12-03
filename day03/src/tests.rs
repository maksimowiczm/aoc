#[cfg(test)]
mod tests {
    use crate::{solution_01, solution_02};

    #[test]
    fn example_01() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = solution_01(input).unwrap();

        assert_eq!(result, 4361);
    }

    #[test]
    fn my_example_01() {
        let input = "*123....15#...";

        let result = solution_01(input).unwrap();

        assert_eq!(result, 123 + 15);
    }

    #[test]
    fn example_02() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = solution_02(input).unwrap();

        assert_eq!(result, 467835);
    }
}
