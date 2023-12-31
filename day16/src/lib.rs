use beam::Beam;
use cave::Cave;

mod beam;
mod cave;
mod tests;
mod aoc_tests;

fn solution_01(input: &str) -> Result<u64, ()> {
    let cave = input.parse::<Cave>()?;
    let beam = Beam::from_cave(&cave);
    Ok(beam.count_energized())
}
