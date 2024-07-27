pub mod camel_joker_hand_value;

use crate::camel_groups_matching;
use crate::day07::hand::camel_joker::camel_joker_hand_value::CamelJokerHandValue;
use crate::day07::hand::{Hand, HandValue};
use std::cmp::Ordering;

impl Hand<CamelJokerHandValue> {
    fn value_five(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelJokerHandValue> {
        camel_groups_matching!(groups, CamelJokerHandValue::Five, 5, self.line.clone())
    }

    fn value_four(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelJokerHandValue> {
        camel_groups_matching!(groups, CamelJokerHandValue::Four, 4, self.line.clone())
    }

    fn value_three(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelJokerHandValue> {
        camel_groups_matching!(groups, CamelJokerHandValue::Three, 3, self.line.clone())
    }

    fn value_two_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelJokerHandValue> {
        let pairs = self.value_pair(groups);
        let pair1 = pairs.get(0);
        let pair2 = pairs.get(1);
        match (pair1, pair2) {
            (Some(CamelJokerHandValue::Pair(str)), Some(CamelJokerHandValue::Pair(_))) => {
                vec![CamelJokerHandValue::TwoPair(str.clone()); 1]
            }
            _ => vec![],
        }
    }

    fn value_full(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelJokerHandValue> {
        let threes = self.value_three(groups);
        let three = threes.get(0);

        let pairs = self.value_pair(groups);
        let pair = pairs.get(0);
        match (three, pair) {
            (Some(CamelJokerHandValue::Three(str)), Some(CamelJokerHandValue::Pair(_))) => {
                vec![CamelJokerHandValue::Full(str.clone()); 1]
            }
            _ => vec![],
        }
    }

    fn value_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelJokerHandValue> {
        camel_groups_matching!(groups, CamelJokerHandValue::Pair, 2, self.line.clone())
    }

    fn internal_value(&self) -> Vec<CamelJokerHandValue> {
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
            .for_each(|_| results.push(CamelJokerHandValue::HighCard(self.line.clone())));

        results
    }
}

impl HandValue<'_, CamelJokerHandValue> for Hand<CamelJokerHandValue> {
    fn value(&self) -> Vec<CamelJokerHandValue> {
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

                let hand = Hand::<CamelJokerHandValue>::from((cards, 0, line));
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

impl PartialOrd<Self> for Hand<CamelJokerHandValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<CamelJokerHandValue> {
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
