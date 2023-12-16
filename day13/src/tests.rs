#[cfg(test)]
mod tests {
    use crate::{solution_01, solution_02};

    #[test]
    fn example_01_01() {
        const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let result = solution_01(INPUT);
        assert_eq!(result, Ok(405));
    }

    #[test]
    fn example_01_02() {
        const INPUT: &str = "##..##..##.
######..###
.####.##.##
..........#
.####.##.##
.####....##
..##..##..#
";

        let result = solution_01(INPUT);
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn example_01_03() {
        const INPUT: &str = "..#.#.####.#.#.
##.####..####.#
##.#........#.#
..#....##......
#####......####
..#.#.####.#.#.
..#.#.#..#.#.#.
...##..##..##..
..#.#......#.#.
";

        let result = solution_01(INPUT);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn example_02_01() {
        const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

        let result = solution_02(INPUT);
        assert_eq!(result, Ok(400));
    }

    #[test]
    fn example_02_02() {
        const INPUT: &str = ".#####.##..
##...##.#..
##.##..####
#...##....#
.#....#####
.#....#####
#...##....#
##.#...####
##...##.#..
.#####.##..
#####..##..
#.###....#.
#.###....#.
#####..##..
.#####.##..
";

        let result = solution_02(INPUT);
        assert_eq!(result, Ok(400));
    }
}
