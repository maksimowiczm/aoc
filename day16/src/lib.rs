use crate::beam::BeamDirection;
use beam::Beam;
use cave::Cave;

mod aoc_tests;
mod beam;
mod cave;
mod tests;

fn solution_01(input: &str) -> Result<u64, ()> {
    let cave = input.parse::<Cave>()?;
    let beam = Beam::from_cave(&cave);
    Ok(beam.count_energized())
}

fn solution_02(input: &str) -> Result<u64, ()> {
    let cave = input.parse::<Cave>()?;

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
            Beam::from_cave_with_start(&cave, (i as u8, 0, BeamDirection::Down)).count_energized()
        }))
        .chain((0..cave.0.len()).map(|i| {
            Beam::from_cave_with_start(
                &cave,
                (i as u8, (cave.0.len() - 1) as u8, BeamDirection::Up),
            )
            .count_energized()
        }))
        .max()
        .ok_or(())
}
