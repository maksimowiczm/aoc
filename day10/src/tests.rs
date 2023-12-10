#[cfg(test)]
mod tests {
    use crate::solution_01;

    #[test]
    fn example_01_01() {
        const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";
        let result = solution_01(INPUT);

        assert_eq!(result, 4);
    }

    #[test]
    fn example_01_02() {
        const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
        let result = solution_01(INPUT);

        assert_eq!(result, 8);
    }
}
