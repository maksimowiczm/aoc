#[cfg(test)]
mod tests {
    use crate::solution_01;

    #[test]
    fn example_01_01() {
        const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

        let result = solution_01(INPUT);

        assert_eq!(result, Ok(136));
    }
}
