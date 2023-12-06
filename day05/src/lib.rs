use crate::category::Category;
use crate::map::{CategoryMap, Map};
use itertools::Itertools;
use std::ops::{Add, Sub};
use std::str::FromStr;

mod category;
mod map;
mod tests;

pub fn solution_01<T>(input: &str) -> T
    where
        T: Add<Output=T> + Sub<Output=T> + PartialOrd + FromStr + Ord + Clone,
{
    let lines = input.split("\n").collect::<Vec<_>>();

    let seeds = lines[0]
        .chars()
        .skip(6)
        .collect::<String>()
        .split(" ")
        .flat_map(|seed| seed.parse::<T>())
        .collect::<Vec<_>>();

    let categories = lines
        .iter()
        .flat_map(|line| line.parse::<Category>())
        .collect::<Vec<_>>();

    let maps = lines
        .iter()
        .map(|l| l.parse::<Map<T>>())
        .group_by(|v| v.is_ok())
        .into_iter()
        .filter(|(success, _)| *success)
        .map(|(_, v)| v.flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert_eq!(categories.len(), maps.len());

    let category_maps = (0..categories.len())
        .map(|i| CategoryMap::from(&maps[i]))
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|seed| {
            category_maps
                .iter()
                .fold(seed.clone(), |acc, category_map| {
                    category_map.convert_to_destination(acc)
                })
        })
        .min()
        .unwrap()
}

pub fn solution_02(input: &str) -> usize {
    let lines = input.split("\n").collect::<Vec<_>>();

    let seeds = lines[0]
        .chars()
        .skip(6)
        .collect::<String>()
        .split(" ")
        .flat_map(|seed| seed.parse::<usize>())
        .collect::<Vec<_>>();

    let seeds_ranges = (0..seeds.len())
        .map(|v| v * 2)
        .filter(|v| *v < seeds.len())
        .map(|i| (seeds[i], seeds[i] + seeds[i + 1]))
        .collect::<Vec<_>>();

    let categories = lines
        .iter()
        .flat_map(|line| line.parse::<Category>())
        .collect::<Vec<_>>();

    let maps = lines
        .iter()
        .map(|l| l.parse::<Map<usize>>())
        .group_by(|v| v.is_ok())
        .into_iter()
        .filter(|(success, _)| *success)
        .map(|(_, v)| v.flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert_eq!(categories.len(), maps.len());

    let category_maps = (0..categories.len())
        .map(|i| CategoryMap::from(&maps[i]))
        .collect::<Vec<_>>();

    let mut iter = 0..usize::MAX;
    while let Some(v) = iter.next() {
        let out = category_maps
            .iter()
            .rev()
            .fold(v, |acc, category_map| category_map.convert_to_source(acc));

        if seeds_ranges.iter().fold(false, |acc, (start, end)| {
            acc || (*start <= out && out <= *end)
        }) {
            return v;
        }
    }

    panic!()
}
