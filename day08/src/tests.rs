#[cfg(test)]
mod tests {
    use crate::{solution_01, solution_02};

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

    #[test]
    fn example_02_1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        let result = solution_02(input);

        assert_eq!(result, 6);
    }
}
