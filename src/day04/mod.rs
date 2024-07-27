use crate::day04::card::Cards;
use crate::solution::SolveDay;

mod card;

struct Day04;

impl SolveDay for Day04 {
    type Part1 = u128;
    type Part2 = u128;
    fn solve_part1(input: &str) -> Option<Self::Part1> {
        let cards = input.parse::<Cards>().unwrap().0;

        let result = cards
            .iter()
            .map(|c| c.get_wins())
            .filter(|count| *count > 0)
            .map(|count| 2u128.pow(count as u32 - 1))
            .fold(0, |acc, v| acc + v);

        Some(result)
    }
    fn solve_part2(input: &str) -> Option<Self::Part2> {
        let cards = input.parse::<Cards>().unwrap().0;

        let won_proxies = cards
            .iter()
            .map(|c| c.get_wins())
            .enumerate()
            .map(|(i, wins)| (i + 1..i + 1 + wins).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut counts = won_proxies.iter().map(|c| c.len()).collect::<Vec<_>>();

        won_proxies
            .iter()
            .enumerate()
            .rev()
            .for_each(|(i, cards_won)| {
                cards_won.iter().for_each(|card| {
                    counts[i] += counts[*card];
                })
            });

        let result = counts.iter().fold(0, |acc, v| acc + v) as u128 + cards.len() as u128;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = Day04::solve_part1(input).unwrap();

        assert_eq!(result, 13);
    }

    #[test]
    fn example_02() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        let result = Day04::solve_part2(input).unwrap();

        assert_eq!(result, 30);
    }
}
