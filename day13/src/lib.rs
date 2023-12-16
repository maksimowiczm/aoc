use std::collections::HashMap;
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

    let mut map = HashMap::new();

    let result_cols = patterns
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.fix_smudge()))
        .filter(|(_, p)| p.is_some())
        .map(|(i, p)| (i, p.unwrap()))
        .fold(0, |acc, (i, (_, res))| {
            assert!(!map.contains_key(&i));
            map.insert(i, res);
            acc + res
        });

    let result_rows = patterns
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.rotate_90().unwrap().fix_smudge()))
        .filter(|(_, p)| p.is_some())
        .map(|(i, p)| (i, p.unwrap()))
        .fold(0, |acc, (i, (_, res))| {
            assert!(!map.contains_key(&i));
            map.insert(i, res);
            acc + res * 100
        });

    Ok(result_cols as u64 + result_rows as u64)
}
