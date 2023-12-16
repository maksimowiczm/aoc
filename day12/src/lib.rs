use std::str::FromStr;

use crate::springs::{Arrangements, FromFolds, Springs};

mod aoc_tests;
mod springs;
mod tests;

pub fn solution_01(input: &str) -> Result<u64, ()> {
    let springs = Springs::from_str(input)?;
    Ok(springs.arrangements_bogo_count())
}

pub fn solution_02(input: &str) -> Result<u64, ()> {
    let springs = Springs::from_folds(input, 5)?;
    Ok(springs.arrangements_bogo_count())
}
