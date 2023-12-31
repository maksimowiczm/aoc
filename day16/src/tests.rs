#[cfg(test)]
mod tests {
    use crate::{solution_01, solution_02};

    #[test]
    fn example_01_01() {
        const INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";

        let result = solution_01(INPUT);

        assert_eq!(result, Ok(46));
    }

    #[test]
    fn example_02_01() {
        const INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";

        let result = solution_02(INPUT);

        assert_eq!(result, Ok(51));
    }
}
