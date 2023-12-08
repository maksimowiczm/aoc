#[cfg(test)]
mod tests {
    use crate::solution_01;

    #[test]
    fn example_01_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        let result = solution_01(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn example_01_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        let result = solution_01(input);

        assert_eq!(result, 6);
    }
}
