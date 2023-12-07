#[cfg(test)]
mod tests {
    use crate::solution_01;

    #[test]
    fn example_01() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = solution_01(input);

        assert_eq!(result, 6440);
    }

    #[test]
    fn my_example_01() {
        let input = "AA432 12\nKK99T 69";

        let result = solution_01(input);

        assert_eq!(result, 12 + 2 * 69);
    }

    #[test]
    fn my_example_02() {
        let input = "AAAAA 10\nAA8AA 20\n23332 30\nTTT98 40\n23432 50\nA23A4 60\n23456 70";

        let result = solution_01(input);

        assert_eq!(
            result,
            7 * 10 + 6 * 20 + 5 * 30 + 4 * 40 + 3 * 50 + 2 * 60 + 70
        );
    }

    #[test]
    fn my_example_03() {
        let input = "KK1AA 10\nKAKA2 20";

        let result = solution_01(input);

        assert_eq!(result, 1 * 10 + 2 * 20);
    }
}
