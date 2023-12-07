mod hand;
mod tests;

use crate::hand::camel_poker::hand_camel_value::HandCamelValue;
use crate::hand::{Hand, HandValue};
use std::collections::HashMap;

pub fn solution_01(input: &str) -> u64 {
    let mut has_card = HashMap::<char, u8>::new();
    has_card.insert('A', 15);
    has_card.insert('K', 13);
    has_card.insert('Q', 12);
    has_card.insert('J', 11);
    has_card.insert('T', 10);

    let mut hands: Vec<Hand<HandCamelValue>> = input
        .split("\n")
        .map(|line| {
            let hand = line.trim().split(" ").collect::<Vec<_>>();
            assert_eq!(hand.len(), 2);
            let mut cards = hand[0]
                .chars()
                .map(|v| Hand::<HandCamelValue>::get_char_as_card(v))
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
