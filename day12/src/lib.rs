use springs::Arrangements;
use std::str::FromStr;

use crate::springs::Springs;

mod aoc_tests;
mod springs;
mod tests;

pub fn solution_01(input: &str) -> Result<u64, ()> {
    let springs = Springs::from_str(input)?;
    Ok(springs.arrangements_count())
}
