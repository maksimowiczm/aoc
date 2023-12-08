pub(crate) mod hand_camel_value;

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::hand::camel_poker::hand_camel_value::HandCamelValue;
use crate::hand::{CardValue, Hand};

macro_rules! groups_matching {
    ($groups:tt, $result:expr, $group_length:expr, $str:expr) => {{
        $groups
            .iter()
            .filter(|(_, cards)| cards.len() == $group_length)
            .map(|(_, _)| $result($str))
            .collect::<Vec<_>>()
    }};
}

impl CardValue for Hand<HandCamelValue<'_>> {
    fn get_char_as_card(ch: char) -> u8 {
        let mut has_card = HashMap::<char, u8>::new();
        has_card.insert('A', 15);
        has_card.insert('K', 13);
        has_card.insert('Q', 12);
        has_card.insert('J', 1);
        has_card.insert('T', 10);

        if ch.is_digit(10) {
            ch as u8 - 48
        } else {
            *has_card.get(&ch).unwrap()
        }
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
