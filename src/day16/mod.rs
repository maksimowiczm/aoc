use crate::day16::beam::BeamDirection;
use crate::solution::SolveDay;
use beam::Beam;
use cave::Cave;

mod beam;
mod cave;

struct Day16;

impl SolveDay for Day16 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let cave = input.parse::<Cave>().ok()?;
        let beam = Beam::from_cave(&cave);
        Some(beam.count_energized())
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let cave = input.parse::<Cave>().ok()?;

        (0..cave.0.len())
            .map(|i| {
                Beam::from_cave_with_start(&cave, (0u8, i as u8, BeamDirection::Right))
                    .count_energized()
            })
            .chain((0..cave.0.len()).map(|i| {
                Beam::from_cave_with_start(
                    &cave,
                    ((cave.0.len() - 1) as u8, i as u8, BeamDirection::Left),
                )
                .count_energized()
            }))
            .chain((0..cave.0.len()).map(|i| {
                Beam::from_cave_with_start(&cave, (i as u8, 0, BeamDirection::Down))
                    .count_energized()
            }))
            .chain((0..cave.0.len()).map(|i| {
                Beam::from_cave_with_start(
                    &cave,
                    (i as u8, (cave.0.len() - 1) as u8, BeamDirection::Up),
                )
                .count_energized()
            }))
            .max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let result = Day16::solve_part1(INPUT).unwrap();

        assert_eq!(result, 46);
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

        let result = Day16::solve_part2(INPUT).unwrap();

        assert_eq!(result, 51);
    }
}
