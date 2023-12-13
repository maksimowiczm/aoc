#[cfg(test)]
mod tests {
    use crate::{solution_01, solution_02};

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

    #[test]
    fn example_02_01() {
        const INPUT: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

        let result = solution_02(INPUT);

        assert_eq!(result, 4);
    }

    #[test]
    fn example_02_02() {
        const INPUT: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

        let result = solution_02(INPUT);

        assert_eq!(result, 10);
    }

    #[test]
    fn example_02_03() {
        const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

        let result = solution_02(INPUT);

        assert_eq!(result, 1);
    }
}
