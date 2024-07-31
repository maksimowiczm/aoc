mod models;
mod parser;

use crate::day19::models::{Category, Part, ReviewResult, Rule, Workflow};
use crate::day19::parser::{Lexer, Parser};
use crate::solution::SolveDay;
use std::iter::Peekable;

pub struct Day19;

impl SolveDay for Day19 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        Some(part1(input.chars().peekable()))
    }
}

fn part1(input: Peekable<impl Iterator<Item = char>>) -> u64 {
    let lexer = Lexer::new(input);
    let tokens = lexer.collect::<Vec<_>>();
    let parser = Parser::new(tokens.into_iter().peekable());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
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

        let result = part1(INPUT.chars().peekable());
        assert_eq!(result, 19114);
    }
}
