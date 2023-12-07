pub(crate) mod hand_camel_value;

use std::cmp::Ordering;

use crate::hand::camel_poker::hand_camel_value::HandCamelValue;
use crate::hand::{Hand, HandValue};

macro_rules! groups_matching {
    ($groups:tt, $result:expr, $group_length:expr, $str:expr) => {{
        $groups
            .iter()
            .filter(|(_, cards)| cards.len() == $group_length)
            .map(|(_, _)| $result($str))
            .collect::<Vec<_>>()
    }};
}

impl HandValue for Hand<HandCamelValue<'_>> {
    fn get_bid(&self) -> u64 {
        self.bid
    }
}

impl Hand<HandCamelValue<'_>> {
    fn value_five(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelValue> {
        groups_matching!(groups, HandCamelValue::Five, 5, &self.line)
    }

    fn value_four(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelValue> {
        groups_matching!(groups, HandCamelValue::Four, 4, &self.line)
    }

    fn value_three(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelValue> {
        groups_matching!(groups, HandCamelValue::Three, 3, &self.line)
    }

    fn value_two_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelValue> {
        let pair1 = self.value_pair(groups).get(0).copied();
        let pair2 = self.value_pair(groups).get(1).copied();
        match (pair1, pair2) {
            (Some(HandCamelValue::Pair(str)), Some(HandCamelValue::Pair(_))) => {
                vec![HandCamelValue::TwoPair(str); 1]
            }
            _ => vec![],
        }
    }

    fn value_full(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelValue> {
        let three = self.value_three(groups).get(0).copied();
        let pair = self.value_pair(groups).get(0).copied();
        match (three, pair) {
            (Some(HandCamelValue::Three(str)), Some(HandCamelValue::Pair(_))) => {
                vec![HandCamelValue::Full(str); 1]
            }
            _ => vec![],
        }
    }

    fn value_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelValue> {
        groups_matching!(groups, HandCamelValue::Pair, 2, &self.line)
    }

    pub fn value(&self) -> Vec<HandCamelValue> {
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
            .for_each(|_| results.push(HandCamelValue::HighCard(&self.line)));

        results
    }
}

impl PartialOrd<Self> for Hand<HandCamelValue<'_>> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<HandCamelValue<'_>> {
    fn cmp(&self, other: &Self) -> Ordering {
        let my = self.value();
        let other = other.value();

        let results = my
            .iter()
            .zip(&other)
            .map(|(&a, &b)| a.cmp(&b))
            .filter(|order| *order != Ordering::Equal)
            .collect::<Vec<_>>();

        if let Some(order) = results.get(0) {
            *order
        } else {
            Ordering::Equal
        }
    }
}
