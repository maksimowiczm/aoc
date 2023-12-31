use crate::label::Label;
use std::str::FromStr;

mod aoc_tests;
mod label;
mod tests;

fn solution_01(input: &str) -> Result<u64, ()> {
    let sequence = input
        .split(",")
        .flat_map(Label::from_str)
        .collect::<Vec<_>>()
        .iter()
        .map(Label::hash)
        .sum();

    Ok(sequence)
}
