use std::collections::HashMap;

use crate::day10::coord_pipe::CoordPipe;
use crate::day10::next_pipe::NextPipe;
use crate::day10::pipe_board::PipesBoard;
use crate::solution::SolveDay;
use itertools::Itertools;

mod coord_pipe;
mod next_pipe;
mod pipe_board;

struct Day10;

impl SolveDay for Day10 {
    type Part1 = i64;
    type Part2 = i64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let (pipes, _, _) = parse_input(input);
        if let Some(start) = find_start(&pipes) {
            let (x, y) = *start.get_location();
            let x = x as usize;
            let y = y as usize;
            let pipes_around = [
                get_pipe(&pipes, Some(x), y.checked_sub(1)),
                get_pipe(&pipes, Some(x), y.checked_add(1)),
                get_pipe(&pipes, x.checked_sub(1), Some(y)),
                get_pipe(&pipes, x.checked_add(1), Some(y)),
            ];

            let distances = pipes_around
                .iter()
                .flatten()
                .map(|connected_to_start_pipe| {
                    let mut prev_pipe = start;
                    let mut distance = 1;
                    let mut current_pipe = *connected_to_start_pipe;
                    let mut distances = HashMap::<(i64, i64), i64>::new();

                    while let Some(next_pipe_location) = current_pipe.next(prev_pipe.get_location())
                    {
                        if current_pipe.get_symbol() == 'S' {
                            break;
                        }
                        distances.insert(*current_pipe.get_location(), distance);

                        prev_pipe = current_pipe;
                        let (x, y) = next_pipe_location;
                        distance += 1;
                        match get_pipe(&pipes, Some(x as usize), Some(y as usize)) {
                            Some(pipe) => current_pipe = pipe,
                            _ => break,
                        }
                    }

                    distances
                })
                .collect::<Vec<_>>();

            let mut true_distances = HashMap::<(i64, i64), i64>::new();
            distances.iter().for_each(|distances| {
                distances.iter().for_each(|(key, distance)| {
                    true_distances
                        .entry(*key)
                        .and_modify(|existing_distance| {
                            if *existing_distance < *distance {
                                *existing_distance = *distance;
                            }
                        })
                        .or_insert(*distance);
                });
            });

            true_distances.values().into_iter().min().copied()
        } else {
            Some(0)
        }
    }

    // todo
    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let (pipes, height, width) = parse_input(input);
        if let Some(start) = find_start(&pipes) {
            let (x, y) = *start.get_location();
            let x = x as usize;
            let y = y as usize;
            let pipes_around = [
                get_pipe(&pipes, Some(x), y.checked_sub(1)),
                get_pipe(&pipes, Some(x), y.checked_add(1)),
                get_pipe(&pipes, x.checked_sub(1), Some(y)),
                get_pipe(&pipes, x.checked_add(1), Some(y)),
            ];
            let next = pipes_around.iter().flatten().collect::<Vec<_>>();
            let next = next.get(0).unwrap();

            let mut prev_pipe = start;
            let mut current_pipe = **next;
            let mut pipes_board = PipesBoard::from((height, width));
            pipes_board.push(*start.get_location());

            while let Some(next_pipe_location) = current_pipe.next(prev_pipe.get_location()) {
                if current_pipe.get_symbol() == 'S' {
                    break;
                }
                pipes_board.push(*current_pipe.get_location());

                prev_pipe = current_pipe;
                let (x, y) = next_pipe_location;
                match get_pipe(&pipes, Some(x as usize), Some(y as usize)) {
                    Some(pipe) => current_pipe = pipe,
                    _ => break,
                }
            }

            Some(pipes_board.count_inside_blocks())
        } else {
            Some(0)
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<CoordPipe<i64>>>, usize, usize) {
    let lines = input.split("\n").collect::<Vec<_>>();

    let pipes = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| CoordPipe::from((ch, (x as i64, y as i64))))
                .collect::<Vec<_>>()
        })
        .filter(|vec| vec.len() != 0)
        .collect::<Vec<_>>();

    let width = if let Some(line) = lines.get(0) {
        line.len()
    } else {
        0
    };
    let height = pipes.len();
    (pipes, height, width)
}

fn find_start<T>(pipes: &Vec<Vec<CoordPipe<T>>>) -> Option<&CoordPipe<T>> {
    pipes
        .iter()
        .flatten()
        .find_or_last(|&pipe| pipe.get_symbol() == 'S')
}

fn get_pipe<T>(
    pipes: &Vec<Vec<CoordPipe<T>>>,
    x: Option<usize>,
    y: Option<usize>,
) -> Option<&CoordPipe<T>> {
    if let (Some(x), Some(y)) = (x, y) {
        match pipes.get(y)?.get(x) {
            Some(pipe) => match pipe.get_symbol() {
                '.' => None,
                _ => Some(pipe),
            },
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01_01() {
        const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";
        let result = Day10::solve_part1(INPUT).unwrap();

        assert_eq!(result, 4);
    }

    #[test]
    fn example_01_02() {
        const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
        let result = Day10::solve_part1(INPUT).unwrap();

        assert_eq!(result, 8);
    }

    #[test]
    #[ignore]
    fn example_02_01() {
        const INPUT: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

        let result = Day10::solve_part2(INPUT).unwrap();

        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn example_02_02() {
        const INPUT: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

        let result = Day10::solve_part2(INPUT).unwrap();

        assert_eq!(result, 10);
    }

    #[test]
    #[ignore]
    fn example_02_03() {
        const INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

        let result = Day10::solve_part2(INPUT).unwrap();

        assert_eq!(result, 1);
    }
}
