use crate::map::{Category, CategoryMap, Map};
use itertools::Itertools;

mod map;
mod tests;

pub fn solution_01(input: &str) -> u32 {
    let lines = input.split("\n").collect::<Vec<_>>();

    let seeds = lines[0]
        .chars()
        .skip(6)
        .collect::<String>()
        .split(" ")
        .map(|seed| seed.parse::<u32>())
        .flatten()
        .collect::<Vec<_>>();

    let categories = lines
        .iter()
        .map(|line| line.parse::<Category>())
        .flatten()
        .collect::<Vec<_>>();

    let maps = lines
        .iter()
        .map(|l| l.parse::<Map>())
        .group_by(|v| v.is_ok())
        .into_iter()
        .filter(|(success, _)| *success)
        .map(|(_, v)| v.flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert_eq!(categories.len(), maps.len());

    let category_maps = (0..categories.len())
        .map(|i| CategoryMap {
            from: categories[i].from.clone(),
            to: categories[i].to.clone(),
            maps: &maps[i],
        })
        .collect::<Vec<_>>();

    lines.len() as u32
}
