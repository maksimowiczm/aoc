use crate::solution::SolveDay;
use clap::{Parser, Subcommand};
use std::fmt::Display;

#[allow(clippy::all)]
mod day03;
#[allow(clippy::all)]
mod day04;
#[allow(clippy::all)]
mod day05;
#[allow(clippy::all)]
mod day06;
#[allow(clippy::all)]
mod day07;
#[allow(clippy::all)]
mod day08;
#[allow(clippy::all)]
mod day09;
#[allow(clippy::all)]
mod day10;
#[allow(clippy::all)]
mod day11;
#[allow(clippy::all)]
mod day12;
#[allow(clippy::all)]
mod day13;
#[allow(clippy::all)]
mod day14;
#[allow(clippy::all)]
mod day15;
#[allow(clippy::all)]
mod day16;
mod day17;
mod day18;
mod day19;
mod solution;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    day: Day,

    #[clap(long)]
    path: String,
}

#[derive(Subcommand)]
enum Part {
    Part1,
    Part2,
}

#[derive(Subcommand)]
enum Day {
    Day17 {
        #[clap(subcommand)]
        part: Part,
    },
    Day18 {
        #[clap(subcommand)]
        part: Part,
    },
    Day19 {
        #[clap(subcommand)]
        part: Part,
    },
}

impl Day {
    fn run(&self, path: &str) -> Box<dyn Display> {
        let input = std::fs::read_to_string(path).unwrap();
        match self {
            Day::Day17 { part: Part::Part1 } => {
                Box::new(day17::Day17::solve_part1(&input).unwrap())
            }
            Day::Day17 { part: Part::Part2 } => {
                Box::new(day17::Day17::solve_part2(&input).unwrap())
            }
            Day::Day18 { part: Part::Part1 } => {
                Box::new(day18::Day18::solve_part1(&input).unwrap())
            }
            Day::Day18 { part: Part::Part2 } => {
                Box::new(day18::Day18::solve_part2(&input).unwrap())
            }
            Day::Day19 { part: Part::Part1 } => {
                Box::new(day19::Day19::solve_part1(&input).unwrap())
            }
            Day::Day19 { part: Part::Part2 } => {
                Box::new(day19::Day19::solve_part2(&input).unwrap())
            }
        }
    }
}

fn main() {
    let args: Args = Args::parse();

    let result = args.day.run(&args.path);
    println!("{}", result);
}
