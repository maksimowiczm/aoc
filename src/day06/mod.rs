use crate::solution::SolveDay;

struct Day06;

impl SolveDay for Day06 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let lines = input.split("\n").collect::<Vec<_>>();
        let times = lines[0]
            .chars()
            .skip(5)
            .collect::<String>()
            .trim()
            .split(" ")
            .flat_map(|v| v.parse::<u64>())
            .collect::<Vec<_>>();

        let distance = lines[1]
            .chars()
            .skip(9)
            .collect::<String>()
            .trim()
            .split(" ")
            .flat_map(|v| v.parse::<u64>())
            .collect::<Vec<_>>();

        assert_eq!(distance.len(), times.len());

        let result = (0..times.len())
            .map(|i| Race(&times[i], &distance[i]))
            .fold(1, |acc, race| acc * race.possible_wins());

        Some(result)
    }
}

struct Race<'a>(&'a u64, &'a u64);

impl Race<'_> {
    fn possible_wins(&self) -> u64 {
        let time = *self.0;
        let distance = *self.1;

        (1..time).fold(0, |acc, velocity| {
            let time_left = time - velocity;
            if time_left * velocity > distance {
                acc + 1
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";

        let result = Day06::solve_part1(input).unwrap();

        assert_eq!(result, 288);
    }

    #[test]
    fn example_02() {
        let input = "Time:      71530
Distance:  940200
";

        let result = Day06::solve_part1(input).unwrap();

        assert_eq!(result, 71503);
    }
}
