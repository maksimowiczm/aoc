use crate::day05::category::Category;
use crate::day05::map::{CategoryMap, Map};
use crate::solution::SolveDay;
use itertools::Itertools;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Add, Sub};
use std::str::FromStr;

mod category;
mod map;

struct Day05<T> {
    phantom_data: PhantomData<T>,
}

impl<T> SolveDay for Day05<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + FromStr + Ord + Clone + Display,
{
    type Part1 = T;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
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
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
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

        (0..u64::MAX).find_or_last(|location| {
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
    }
}

#[allow(dead_code)]
fn solution_02_bf(input: &str) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let result: u8 = Day05::solve_part1(INPUT_EXAMPLE).unwrap();

        assert_eq!(result, 35);
    }

    #[test]
    fn example_02() {
        let result = Day05::<u64>::solve_part2(INPUT_EXAMPLE).unwrap();

        assert_eq!(result, 46);
    }

    #[test]
    fn example_02_bf() {
        let result = solution_02_bf(INPUT_EXAMPLE);

        assert_eq!(result, 46);
    }

    const INPUT_EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
}
