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
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Ord + Clone,
{
    let lines = input.split("\n").collect::<Vec<_>>();

    let seeds = lines[0]
        .chars()
        .skip(6)
        .collect::<String>()
        .split(" ")
        .map(|seed| seed.parse::<T>())
        .flatten()
        .collect::<Vec<_>>();

    let categories = lines
        .iter()
        .map(|line| line.parse::<Category>())
        .flatten()
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
        .map(|i| {
            CategoryMap::from((
                categories[i].from.clone(),
                categories[i].to.clone(),
                &maps[i],
            ))
        })
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|seed| {
            category_maps
                .iter()
                .fold(seed.clone(), |acc, category_map| category_map.convert(acc))
        })
        .min()
        .unwrap()
}
