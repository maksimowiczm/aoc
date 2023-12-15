use crate::space::Space;
use std::str::FromStr;

mod aoc_tests;
mod space;
mod tests;

pub fn solution(input: &str, scale: usize) -> Result<u64, ()> {
    let space = Space::from_str(input)?;
    Ok(space.distance_between_galaxies_pairs(scale))
}
