#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn example_01_01() {
        let result = solution(INPUT, 1);

        assert_eq!(result, Ok(374));
    }

    #[test]
    fn example_02_01() {
        let result = solution(INPUT, 10);

        assert_eq!(result, Ok(1030));
    }

    #[test]
    fn example_02_02() {
        let result = solution(INPUT, 100);

        assert_eq!(result, Ok(8410));
    }

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";


}
