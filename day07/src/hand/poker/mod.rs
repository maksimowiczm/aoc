mod hand_poker_value;
mod tests;

use std::cmp::Ordering;

use crate::hand::poker::hand_poker_value::HandPokerValue;
use crate::hand::{Hand, HandValue};

macro_rules! groups_matching {
    ($groups:tt, $result:expr, $group_length:expr) => {{
        $groups
            .iter()
            .filter(|(_, cards)| cards.len() == $group_length)
            .map(|(v, _)| $result(*v))
            .collect::<Vec<_>>()
    }};
}

impl HandValue for Hand<HandPokerValue> {
    fn get_bid(&self) -> u64 {
        self.bid
    }
}

impl Hand<HandPokerValue> {
    fn value_five(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandPokerValue> {
        groups_matching!(groups, HandPokerValue::Five, 5)
    }

    fn value_four(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandPokerValue> {
        groups_matching!(groups, HandPokerValue::Four, 4)
    }

    fn value_three(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandPokerValue> {
        groups_matching!(groups, HandPokerValue::Three, 3)
    }

    fn value_two_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandPokerValue> {
        let pair1 = self.value_pair(groups).get(0).copied();
        let pair2 = self.value_pair(groups).get(1).copied();
        match (pair1, pair2) {
            (Some(HandPokerValue::Pair(p1)), Some(HandPokerValue::Pair(p2))) => {
                assert!(p1 > p2);
                assert_ne!(p1, p2);
                vec![HandPokerValue::TwoPair(p1, p2); 1]
            }
            _ => vec![],
        }
    }

    fn value_full(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandPokerValue> {
        let three = self.value_three(groups).get(0).copied();
        let pair = self.value_pair(groups).get(0).copied();
        match (three, pair) {
            (Some(HandPokerValue::Three(three_item)), Some(HandPokerValue::Pair(pair_item))) => {
                assert_ne!(three_item, pair_item);
                vec![HandPokerValue::Full(three_item, pair_item); 1]
            }
            _ => vec![],
        }
    }

    fn value_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandPokerValue> {
        groups_matching!(groups, HandPokerValue::Pair, 2)
    }

    pub fn value(&self) -> Vec<HandPokerValue> {
        let groups = self.collect_groups();

        let mut results = vec![];

        results.push(self.value_five(&groups));
        results.push(self.value_four(&groups));
        results.push(self.value_full(&groups));
        results.push(self.value_three(&groups));
        let two_pairs = self.value_two_pair(&groups);
        if two_pairs.len() != 0 {
            results.push(two_pairs);
        } else {
            results.push(self.value_pair(&groups));
        }
        let mut results = results.iter().flatten().copied().collect::<Vec<_>>();
        self.cards
            .iter()
            .for_each(|card| results.push(HandPokerValue::HighCard(*card)));

        results
    }
}

impl PartialOrd<Self> for Hand<HandPokerValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<HandPokerValue> {
    fn cmp(&self, other: &Self) -> Ordering {
        let my = self.value();
        let other = other.value();

        let results = my
            .iter()
            .zip(&other)
            .map(|(a, b)| a.cmp(b))
            .filter(|order| *order != Ordering::Equal)
            .collect::<Vec<_>>();

        if let Some(order) = results.get(0) {
            *order
        } else {
            Ordering::Equal
        }
    }
}
