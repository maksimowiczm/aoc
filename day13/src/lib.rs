use crate::lava_pattern::{LavaPattern, Mirror};
use std::str::FromStr;

mod aoc_tests;
mod lava_pattern;
mod tests;

pub fn solution_01(input: &str) -> Result<u64, ()> {
    let patterns = input
        .split("\n\n")
        .flat_map(LavaPattern::from_str)
        .collect::<Vec<_>>();

    let cols = patterns.iter().fold(0, |acc, pattern| {
        let positions = pattern.mirror_position().unwrap();
        if let Some(&(_, res)) = positions.iter().nth(0) {
            assert_eq!(positions.len(), 1);
            acc + res
        } else {
            acc
        }
    });
    let result = patterns
        .iter()
        .flat_map(LavaPattern::rotate_90)
        .fold(cols, |acc, pattern| {
            let positions = pattern.mirror_position().unwrap();
            if let Some(&(_, res)) = positions.iter().nth(0) {
                assert_eq!(positions.len(), 1);
                acc + res * 100
            } else {
                acc
            }
        });

    Ok(result as u64)
}

pub fn solution_02(input: &str) -> Result<u64, ()> {
    let patterns = input
        .split("\n\n")
        .flat_map(LavaPattern::from_str)
        .collect::<Vec<_>>();

    let result = patterns
        .iter()
        .map(LavaPattern::fix_smudge)
        .fold(0, |acc, pattern| {
            let ((_, res), as_row) = pattern.unwrap();
            let multiplier = if as_row { 100 } else { 1 };
            acc + res * multiplier
        });

    Ok(result as u64)
}
