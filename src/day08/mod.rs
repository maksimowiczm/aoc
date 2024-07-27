use crate::day08::node::Node;
use crate::solution::SolveDay;
use num_integer::Integer;
use std::{collections::HashMap, ops::ControlFlow};

mod node;

struct Day08;

impl SolveDay for Day08 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let lines = input.split("\n").collect::<Vec<_>>();
        let directions = lines[0].trim();

        let parsed = lines
            .iter()
            .skip(2)
            .map(|line| line.replace(" = (", " ").replace(",", "").replace(")", ""))
            .collect::<Vec<_>>();
        let nodes = create_nodes(parsed.iter().map(String::as_str).collect());

        let times = directions.chars().cycle().try_fold(
            Some(("AAA", &nodes["AAA"], 0usize)),
            |state, direction| match state {
                Some((key, node, i)) => match key {
                    "ZZZ" => ControlFlow::Break(Some(("ZZZ", &nodes["ZZZ"], i))),
                    _ => {
                        let next = node.next(&direction);
                        ControlFlow::Continue(Some((next, &nodes[next], i + 1)))
                    }
                },
                _ => ControlFlow::Break(None),
            },
        );

        let result = match times {
            ControlFlow::Break(Some((_, _, result))) => result as u64,
            _ => 0,
        };

        Some(result)
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let lines = input.split("\n").collect::<Vec<_>>();
        let directions = lines[0].trim();

        let parsed = lines
            .iter()
            .skip(2)
            .map(|line| line.replace(" = (", " ").replace(",", "").replace(")", ""))
            .collect::<Vec<_>>();
        let nodes = create_nodes(parsed.iter().map(String::as_str).collect());
        let start_keys = nodes
            .keys()
            .copied()
            .into_iter()
            .filter(|key| match key.chars().last() {
                Some('A') => true,
                _ => false,
            })
            .collect::<Vec<_>>();

        let iters_to_z = start_keys
            .iter()
            .map(|key| map_zs(key, &nodes, directions, 1))
            .collect::<Vec<_>>();

        let result = iters_to_z.iter().fold(1, |acc, v| acc.lcm(v));

        Some(result)
    }
}

fn create_nodes(input: Vec<&str>) -> HashMap<&str, Node> {
    let mut nodes = HashMap::new();

    input
        .iter()
        .flat_map(|line| {
            let values = line.split(" ").collect::<Vec<_>>();
            let key = *values.get(0)?;
            let left = *values.get(1)?;
            let right = *values.get(2)?;
            Some((key, Node::from((right, left))))
        })
        .for_each(|(key, node)| {
            nodes.insert(key, node);
        });

    nodes
}

#[allow(dead_code)]
fn solution_02_bf(input: &str) -> u64 {
    let lines = input.split("\n").collect::<Vec<_>>();
    let directions = lines[0].trim();

    let parsed = lines
        .iter()
        .skip(2)
        .map(|line| line.replace(" = (", " ").replace(",", "").replace(")", ""))
        .collect::<Vec<_>>();
    let nodes = create_nodes(parsed.iter().map(String::as_str).collect());
    let start_keys = nodes
        .keys()
        .copied()
        .into_iter()
        .filter(|key| match key.chars().last() {
            Some('A') => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    let start_nodes = start_keys
        .iter()
        .map(|key| &nodes[*key])
        .collect::<Vec<_>>();

    let times = directions.chars().cycle().try_fold(
        Some((start_keys.clone(), start_nodes.clone(), 0usize)),
        |state, direction| match state {
            Some((keys, these_nodes, i)) => {
                let ends = keys
                    .iter()
                    .flat_map(|key| key.chars().last())
                    .filter(|&ch| ch == 'Z')
                    .count();

                if ends == keys.len() {
                    ControlFlow::Break(Some((keys, these_nodes, i)))
                } else {
                    let next_keys = these_nodes
                        .iter()
                        .map(|node| node.next(&direction))
                        .collect::<Vec<_>>();
                    let next_nodes = next_keys.iter().map(|key| &nodes[key]).collect::<Vec<_>>();
                    ControlFlow::Continue(Some((next_keys, next_nodes, i + 1)))
                }
            }
            _ => ControlFlow::Break(None),
        },
    );

    match times {
        ControlFlow::Break(Some((_, _, result))) => result as u64,
        _ => 0,
    }
}

pub fn map_zs(start: &str, nodes: &HashMap<&str, Node>, directions: &str, n: usize) -> u64 {
    let mut nth = 0;
    let times = directions.chars().cycle().try_fold(
        Some((start, &nodes[start], 0usize)),
        |state, direction| match state {
            Some((key, node, i)) => {
                let next = node.next(&direction);
                if key.chars().last().unwrap() == 'Z' {
                    nth += 1;
                    if nth == n {
                        ControlFlow::Break(Some((key, node, i)))
                    } else {
                        ControlFlow::Continue(Some((next, &nodes[next], i + 1)))
                    }
                } else {
                    ControlFlow::Continue(Some((next, &nodes[next], i + 1)))
                }
            }
            _ => ControlFlow::Break(None),
        },
    );

    match times {
        ControlFlow::Break(Some((_, _, result))) => result as u64,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        let result = Day08::solve_part1(input).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn example_01_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        let result = Day08::solve_part1(input).unwrap();

        assert_eq!(result, 6);
    }

    #[test]
    fn example_02_1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        let result = Day08::solve_part2(input).unwrap();

        assert_eq!(result, 6);
    }
}
