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

pub fn solution_02(input: &str) -> usize {
    let patterns = input
        .split("\n\n")
        .flat_map(LavaPattern::from_str)
        .collect::<Vec<_>>();

    let mut sum = 0;

    let mut result_cols = patterns
        .iter()
        .flat_map(|p| {
            if let Some(res) = p.fix_smudge() {
                Some((p, res))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    result_cols.sort();

    result_cols.iter().for_each(|(s, p)| {
        let old = s.mirror_position().unwrap();
        let mirrors = p.mirror_position().unwrap();
        let new = mirrors
            .iter()
            .filter(|m| !old.iter().any(|old_m| *old_m == **m))
            .collect::<Vec<_>>();
        let mirror = new.iter().nth(0).unwrap();
        sum += mirror.1;
        println!("{:?}", mirror);
        // println!("FROM:\n{s}\nto:\n{p}");
    });

    let mut result_rows = patterns
        .iter()
        .map(|p| (p, p.rotate_90().unwrap()))
        .flat_map(|(s, p)| {
            if let Some(res) = p.fix_smudge() {
                Some((
                    s,
                    res.rotate_90()
                        .unwrap()
                        .rotate_90()
                        .unwrap()
                        .rotate_90()
                        .unwrap(),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    result_rows.sort();

    result_rows.iter().for_each(|(s, p)| {
        let old = s.rotate_90().unwrap().mirror_position().unwrap();
        let mirrors = p.rotate_90().unwrap().mirror_position().unwrap();
        let new = mirrors
            .iter()
            .filter(|m| !old.iter().any(|old_m| *old_m == **m))
            .collect::<Vec<_>>();
        let mirror = new.iter().nth(0).unwrap();
        sum += mirror.1 * 100;
        println!("ROW {:?}", mirror);
        // println!("FROM:\n{s}\nto:\n{p}");
    });

    sum
}
