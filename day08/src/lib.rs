use crate::node::Node;
use std::{collections::HashMap, ops::ControlFlow};

mod node;
mod tests;

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

pub fn solution_01(input: &str) -> u64 {
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

    match times {
        ControlFlow::Break(Some((_, _, result))) => result as u64,
        _ => 0,
    }
}
