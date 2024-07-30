use crate::solution::SolveDay;
use clap::{Parser, Subcommand};
use std::fmt::Display;

mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
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
        }
    }
}

fn main() {
    let args: Args = Args::parse();

    let result = args.day.run(&args.path);
    println!("{}", result);
}
