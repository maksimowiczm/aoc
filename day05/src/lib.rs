use crate::category::Category;
use crate::map::{CategoryMap, Map};
use itertools::Itertools;
use std::ops::{Add, Sub};
use std::str::FromStr;

mod aoc_tests;
mod category;
mod map;
mod tests;

pub fn solution_01<T>(input: &str) -> T
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Ord + Clone,
{
    let lines = input.split("\n").collect::<Vec<_>>();
    let seeds = parse_seeds::<T>(lines[0]);
    let categories = parse_categories::<T>(&lines);
    let maps = parse_maps::<T>(&lines);
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

pub fn solution_02(input: &str) -> u64 {
    let lines = input.split("\n").collect::<Vec<_>>();
    let seeds = parse_seeds::<u64>(lines[0]);
    let seeds_ranges = (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i] + seeds[i + 1]))
        .collect::<Vec<_>>();
    let categories = parse_categories::<u64>(&lines);
    let maps = parse_maps::<u64>(&lines);
    let category_maps = (0..categories.len())
        .map(|i| CategoryMap::from(&maps[i]))
        .collect::<Vec<_>>();

    (0..u64::MAX)
        .find_or_last(|location| {
            let seed = category_maps
                .iter()
                .rev()
                .fold(*location, |acc, category_map| {
                    category_map.convert_to_source(acc)
                });

            seeds_ranges
                .iter()
                .any(|&(start, end)| start <= seed && seed <= end)
        })
        .unwrap()
}

pub fn solution_02_bf(input: &str) -> u64 {
    let lines = input.split("\n").collect::<Vec<_>>();
    let seeds = parse_seeds::<u64>(lines[0]);
    let seeds_ranges = (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i] + seeds[i + 1]))
        .collect::<Vec<_>>();
    let categories = parse_categories::<u64>(&lines);
    let maps = parse_maps::<u64>(&lines);
    let category_maps = (0..categories.len())
        .map(|i| CategoryMap::from(&maps[i]))
        .collect::<Vec<_>>();

    let mut mini = u64::MAX;

    seeds_ranges.iter().for_each(|&(start, end)| {
        (start..end).for_each(|i| {
            let seed = category_maps.iter().fold(i, |acc, category_map| {
                category_map.convert_to_destination(acc)
            });

            if seed < mini {
                mini = seed;
            }
        });

        println!("Range done found {}", mini);
    });

    mini
}

fn parse_seeds<T>(input: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .chars()
        .skip(6)
        .collect::<String>()
        .split(" ")
        .flat_map(|seed| seed.parse::<T>())
        .collect()
}

fn parse_categories<T>(lines: &Vec<&str>) -> Vec<Category> {
    lines
        .iter()
        .flat_map(|line| line.parse::<Category>())
        .collect::<Vec<_>>()
}

fn parse_maps<T>(lines: &Vec<&str>) -> Vec<Vec<Map<T>>>
where
    T: Clone + FromStr,
{
    lines
        .iter()
        .map(|l| l.parse::<Map<T>>())
        .group_by(|v| v.is_ok())
        .into_iter()
        .filter(|(success, _)| *success)
        .map(|(_, v)| v.flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
