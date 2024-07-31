mod models;
mod parser;

use crate::day19::models::{Category, Part, ReviewResult, Rule, Workflow};
use crate::day19::parser::{Lexer, Parser};
use crate::solution::SolveDay;
use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::RangeInclusive;

pub struct Day19;

impl SolveDay for Day19 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        Some(part1(input.chars().peekable()))
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        Some(part2(input.chars().peekable()))
    }
}

fn part1(input: Peekable<impl Iterator<Item = char>>) -> u64 {
    let lexer = Lexer::new(input);
    let parser = Parser::new(lexer.into_iter().peekable());
    let system = parser.parse().unwrap();
    let initial_workflow = system.workflows.get("in").unwrap();

    system
        .parts
        .iter()
        .filter(|part| {
            let mut current_workflow = initial_workflow;
            loop {
                match current_workflow.review(part) {
                    ReviewResult::Accept => return true,
                    ReviewResult::Reject => return false,
                    ReviewResult::Pass(next_workflow) => {
                        current_workflow = system.workflows.get(&next_workflow).unwrap();
                    }
                }
            }
        })
        .fold(0, |acc, part| {
            acc + part.coolness + part.musicality + part.aerodynamic + part.shininess
        })
}

impl Workflow {
    fn review(&self, part: &Part) -> ReviewResult {
        for rule in self.rules.iter() {
            if rule.apply(part) {
                return match rule {
                    Rule::LessThan { then, .. } | Rule::GreaterThan { then, .. } => then.clone(),
                };
            }
        }

        self.otherwise.clone()
    }
}

impl Rule {
    fn apply(&self, part: &Part) -> bool {
        match self {
            Rule::LessThan {
                category, value, ..
            } => {
                (match category {
                    Category::Coolness => part.coolness,
                    Category::Musicality => part.musicality,
                    Category::Aerodynamic => part.aerodynamic,
                    Category::Shininess => part.shininess,
                } < *value)
            }
            Rule::GreaterThan {
                category, value, ..
            } => {
                (match category {
                    Category::Coolness => part.coolness,
                    Category::Musicality => part.musicality,
                    Category::Aerodynamic => part.aerodynamic,
                    Category::Shininess => part.shininess,
                } > *value)
            }
        }
    }
}

fn part2(input: Peekable<impl Iterator<Item = char>>) -> u64 {
    let lexer = Lexer::new(input);
    let parser = Parser::new(lexer.into_iter().peekable());
    let system = parser.parse().unwrap();
    let initial_workflow = system.workflows.get("in").unwrap();

    let range = PartRange {
        coolness: 1..=4000,
        musicality: 1..=4000,
        aerodynamic: 1..=4000,
        shininess: 1..=4000,
    };

    initial_workflow
        .apply(range, &system.workflows)
        .iter()
        .fold(0, |acc, range| acc + range.count())
}

#[derive(Clone)]
struct PartRange {
    coolness: RangeInclusive<u64>,
    musicality: RangeInclusive<u64>,
    aerodynamic: RangeInclusive<u64>,
    shininess: RangeInclusive<u64>,
}

impl PartRange {
    fn count(&self) -> u64 {
        (self.coolness.end() - self.coolness.start() + 1)
            * (self.musicality.end() - self.musicality.start() + 1)
            * (self.aerodynamic.end() - self.aerodynamic.start() + 1)
            * (self.shininess.end() - self.shininess.start() + 1)
    }

    fn apply_rule(self, rule: &Rule) -> (PartRange, PartRange) {
        match rule {
            Rule::LessThan {
                category, value, ..
            } => {
                let start = *category.start(&self);
                let end = *category.end(&self);
                let left = category.new_range(self.clone(), start, *value - 1);
                let right = category.new_range(self, *value, end);
                (left, right)
            }
            Rule::GreaterThan {
                category, value, ..
            } => {
                let start = *category.start(&self);
                let end = *category.end(&self);
                let left = category.new_range(self.clone(), *value + 1, end);
                let right = category.new_range(self, start, *value);
                (left, right)
            }
        }
    }
}

impl Workflow {
    fn apply(&self, range: PartRange, workflows: &HashMap<String, Workflow>) -> Vec<PartRange> {
        let mut ranges = vec![];
        let mut current_range = range;
        for rule in self.rules.iter() {
            let (left, right) = current_range.apply_rule(rule);
            current_range = right;

            match rule.then() {
                ReviewResult::Accept => {
                    ranges.push(left);
                }
                ReviewResult::Reject => {}
                ReviewResult::Pass(next) => {
                    let next_workflow = workflows.get(next).unwrap();
                    ranges.extend(next_workflow.apply(left, workflows));
                }
            }
        }

        match &self.otherwise {
            ReviewResult::Accept => {
                ranges.push(current_range);
            }
            ReviewResult::Reject => {}
            ReviewResult::Pass(next) => {
                let next_workflow = workflows.get(next).unwrap();
                ranges.extend(next_workflow.apply(current_range, workflows));
            }
        }

        ranges
    }
}
impl Category {
    fn start<'range>(&self, range: &'range PartRange) -> &'range u64 {
        match self {
            Category::Coolness => range.coolness.start(),
            Category::Musicality => range.musicality.start(),
            Category::Aerodynamic => range.aerodynamic.start(),
            Category::Shininess => range.shininess.start(),
        }
    }
    fn end<'range>(&self, range: &'range PartRange) -> &'range u64 {
        match self {
            Category::Coolness => range.coolness.end(),
            Category::Musicality => range.musicality.end(),
            Category::Aerodynamic => range.aerodynamic.end(),
            Category::Shininess => range.shininess.end(),
        }
    }
    fn new_range(&self, range: PartRange, start: u64, end: u64) -> PartRange {
        match self {
            Category::Coolness => PartRange {
                coolness: start..=end,
                ..range
            },
            Category::Musicality => PartRange {
                musicality: start..=end,
                ..range
            },
            Category::Aerodynamic => PartRange {
                aerodynamic: start..=end,
                ..range
            },
            Category::Shininess => PartRange {
                shininess: start..=end,
                ..range
            },
        }
    }
}

impl Rule {
    fn then(&self) -> &ReviewResult {
        match self {
            Rule::LessThan { then, .. } | Rule::GreaterThan { then, .. } => then,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn example_01() {
        let result = part1(INPUT.chars().peekable());
        assert_eq!(result, 19114);
    }

    #[test]
    fn example_02() {
        let result = part2(INPUT.chars().peekable());
        assert_eq!(result, 167409079868000);
    }
}
