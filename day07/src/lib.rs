mod hand;
mod tests;

use crate::hand::{CardValue, Hand, HandBid};

pub fn solution<T>(input: &str) -> u64
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
