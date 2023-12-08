pub mod hand_camel_joker_value;

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::hand::camel_joker_poker::hand_camel_joker_value::HandCamelJokerValue;
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

impl CardValue for Hand<HandCamelJokerValue> {
    fn get_char_as_card(ch: char) -> u8 {
        let mut has_card = HashMap::<char, u8>::new();
        has_card.insert('A', 15);
        has_card.insert('K', 13);
        has_card.insert('Q', 12);
        has_card.insert('J', 11);
        has_card.insert('T', 10);

        if ch.is_digit(10) {
            ch as u8 - 48
        } else {
            *has_card.get(&ch).unwrap()
        }
    }
}

impl Hand<HandCamelJokerValue> {
    fn value_five(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelJokerValue> {
        groups_matching!(groups, HandCamelJokerValue::Five, 5, self.line.clone())
    }

    fn value_four(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelJokerValue> {
        groups_matching!(groups, HandCamelJokerValue::Four, 4, self.line.clone())
    }

    fn value_three(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelJokerValue> {
        groups_matching!(groups, HandCamelJokerValue::Three, 3, self.line.clone())
    }

    fn value_two_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelJokerValue> {
        let pairs = self.value_pair(groups);
        let pair1 = pairs.get(0);
        let pair2 = pairs.get(1);
        match (pair1, pair2) {
            (Some(HandCamelJokerValue::Pair(str)), Some(HandCamelJokerValue::Pair(_))) => {
                vec![HandCamelJokerValue::TwoPair(str.clone()); 1]
            }
            _ => vec![],
        }
    }

    fn value_full(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelJokerValue> {
        let threes = self.value_three(groups);
        let three = threes.get(0);

        let pairs = self.value_pair(groups);
        let pair = pairs.get(0);
        match (three, pair) {
            (Some(HandCamelJokerValue::Three(str)), Some(HandCamelJokerValue::Pair(_))) => {
                vec![HandCamelJokerValue::Full(str.clone()); 1]
            }
            _ => vec![],
        }
    }

    fn value_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<HandCamelJokerValue> {
        groups_matching!(groups, HandCamelJokerValue::Pair, 2, self.line.clone())
    }

    pub fn internal_value(&self) -> Vec<HandCamelJokerValue> {
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
        let mut results = results.iter().flat_map(|v| v.clone()).collect::<Vec<_>>();
        self.cards
            .iter()
            .for_each(|_| results.push(HandCamelJokerValue::HighCard(self.line.clone())));

        results
    }

    pub fn value(&self) -> Vec<HandCamelJokerValue> {
        let jokers_positions = self
            .cards
            .iter()
            .enumerate()
            .filter(|&(_, v)| *v == 1)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        if jokers_positions.len() == 0 {
            return self.internal_value();
        }

        let unique_cards = self
            .collect_groups()
            .iter()
            .filter(|(v, _)| *v != 1)
            .map(|(v, _)| *v)
            .collect::<Vec<_>>();

        let line = self.line.as_str();
        let joker_hands = unique_cards
            .iter()
            .map(|&v| {
                let mut cards = self.cards.clone();
                jokers_positions.iter().for_each(|i| cards[*i] = v);
                cards.sort();
                cards.reverse();

                let hand = Hand::<HandCamelJokerValue>::from((cards, 0, line));
                hand
            })
            .collect::<Vec<_>>();

        let mut values = joker_hands
            .iter()
            .map(|h| h.internal_value())
            .collect::<Vec<_>>();

        if values.len() == 0 {
            return self.internal_value();
        }

        values.sort();

        values[0].clone()
    }
}

impl PartialOrd<Self> for Hand<HandCamelJokerValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<HandCamelJokerValue> {
    fn cmp(&self, other: &Self) -> Ordering {
        let my = self.value();
        let other = other.value();

        let results = my
            .iter()
            .zip(&other)
            .map(|(&ref a, &ref b)| a.cmp(&b))
            .filter(|order| *order != Ordering::Equal)
            .collect::<Vec<_>>();

        if let Some(order) = results.get(0) {
            *order
        } else {
            Ordering::Equal
        }
    }
}
