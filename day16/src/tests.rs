#[cfg(test)]
mod tests {
    use crate::solution_01;

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
}
