use crate::day07::hand::camel_joker::camel_joker_hand_value::CamelJokerHandValue;
use crate::day07::hand::camel_poker::camel_hand_value::CamelHandValue;
use crate::day07::hand::{CardValue, Hand, HandBid};
use crate::solution::SolveDay;

mod hand;

struct Day07;

impl SolveDay for Day07 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part1(input: &str) -> Option<Self::Part1> {
        Some(solution::<CamelHandValue>(input))
    }

    fn solve_part2(input: &str) -> Option<Self::Part2> {
        Some(solution::<CamelJokerHandValue>(input))
    }
}

fn solution<T>(input: &str) -> u64
where
    Hand<T>: Ord + HandBid,
    T: CardValue,
{
    let mut hands: Vec<Hand<T>> = input
        .split("\n")
        .map(|line| {
            let hand = line.trim().split(" ").collect::<Vec<_>>();
            assert_eq!(hand.len(), 2);
            let mut cards = hand[0]
                .chars()
                .map(|v| T::get_char_as_card(v))
                .collect::<Vec<_>>();
            cards.sort();
            Hand::from((
                cards.iter().rev().cloned().collect(),
                hand[1].parse::<u64>().unwrap(),
                line,
            ))
        })
        .collect::<Vec<_>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u64 + 1) * hand.get_bid())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = Day07::solve_part1(input).unwrap();

        assert_eq!(result, 6440);
    }

    #[test]
    fn my_example_01() {
        let input = "AA432 12\nKK99T 69";

        let result = Day07::solve_part1(input).unwrap();

        assert_eq!(result, 12 + 2 * 69);
    }

    #[test]
    fn my_example_02() {
        let input = "AAAAA 10\nAA8AA 20\n23332 30\nTTT98 40\n23432 50\nA23A4 60\n23456 70";

        let result = Day07::solve_part1(input).unwrap();

        assert_eq!(
            result,
            7 * 10 + 6 * 20 + 5 * 30 + 4 * 40 + 3 * 50 + 2 * 60 + 70
        );
    }

    #[test]
    fn my_example_03() {
        let input = "KK1AA 10\nKAKA2 20";

        let result = Day07::solve_part1(input).unwrap();

        assert_eq!(result, 1 * 10 + 2 * 20);
    }

    #[test]
    fn my_example_04() {
        let input = "33332 12\n2AAAA 69";

        let result = Day07::solve_part1(input).unwrap();

        assert_eq!(result, 12 * 2 + 69 * 1);
    }

    #[test]
    fn my_example_05() {
        let input = "KK677 28
KTJJT 220";

        let result = Day07::solve_part1(input).unwrap();

        assert_eq!(result, 28 * 2 + 220);
    }

    #[test]
    fn example_02() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = Day07::solve_part2(input).unwrap();

        assert_eq!(result, 5905);
    }
}
