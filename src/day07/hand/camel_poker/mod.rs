pub(crate) mod camel_hand_value;

use crate::camel_groups_matching;
use crate::day07::hand::camel_poker::camel_hand_value::CamelHandValue;
use crate::day07::hand::{Hand, HandValue};
use std::cmp::Ordering;

impl<'a> HandValue<'a, CamelHandValue<'a>> for Hand<CamelHandValue<'a>> {
    fn value(&self) -> Vec<CamelHandValue> {
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
            .for_each(|_| results.push(CamelHandValue::HighCard(&self.line)));

        results
    }
}

impl Hand<CamelHandValue<'_>> {
    fn value_five(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelHandValue> {
        camel_groups_matching!(groups, CamelHandValue::Five, 5, &self.line)
    }

    fn value_four(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelHandValue> {
        camel_groups_matching!(groups, CamelHandValue::Four, 4, &self.line)
    }

    fn value_three(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelHandValue> {
        camel_groups_matching!(groups, CamelHandValue::Three, 3, &self.line)
    }

    fn value_two_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelHandValue> {
        let pair1 = self.value_pair(groups).get(0).copied();
        let pair2 = self.value_pair(groups).get(1).copied();
        match (pair1, pair2) {
            (Some(CamelHandValue::Pair(str)), Some(CamelHandValue::Pair(_))) => {
                vec![CamelHandValue::TwoPair(str); 1]
            }
            _ => vec![],
        }
    }

    fn value_full(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelHandValue> {
        let three = self.value_three(groups).get(0).copied();
        let pair = self.value_pair(groups).get(0).copied();
        match (three, pair) {
            (Some(CamelHandValue::Three(str)), Some(CamelHandValue::Pair(_))) => {
                vec![CamelHandValue::Full(str); 1]
            }
            _ => vec![],
        }
    }

    fn value_pair(&self, groups: &Vec<(u8, Vec<u8>)>) -> Vec<CamelHandValue> {
        camel_groups_matching!(groups, CamelHandValue::Pair, 2, &self.line)
    }
}

impl PartialOrd<Self> for Hand<CamelHandValue<'_>> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<CamelHandValue<'_>> {
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
