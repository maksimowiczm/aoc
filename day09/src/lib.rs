use crate::sequence::Sequence;
use std::str;
use str::FromStr;

mod sequence;
mod tests;

fn parse_input<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
{
    input
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(" ")
                .flat_map(|v| v.parse::<T>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn solution_01(input: &str) -> i64 {
    let lines = parse_input::<i64>(input);
    let predictions = lines
        .iter()
        .map(|line| Sequence::from(line.clone()).predict_forward())
        .collect::<Vec<_>>();

    predictions.iter().fold(0, |acc, v| acc + v)
}

pub fn solution_02(input: &str) -> i64 {
    let lines = parse_input::<i64>(input);
    let predictions = lines
        .iter()
        .map(|line| Sequence::from(line.clone()).predict_backward())
        .collect::<Vec<_>>();

    predictions.iter().fold(0, |acc, v| acc + v)
}
