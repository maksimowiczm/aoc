use itertools::Itertools;
use std::collections::HashMap;
use std::marker::PhantomData;

pub mod camel_poker;
pub mod poker;

#[allow(dead_code)]
pub struct Hand<T> {
    phantom: PhantomData<T>,
    cards: Vec<u8>,
    bid: u64,
    line: String,
}

impl<T> Hand<T> {
    pub fn get_char_as_card(ch: char) -> u8 {
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

    pub fn collect_groups(&self) -> Vec<(u8, Vec<u8>)> {
        self.cards
            .iter()
            .group_by(|c| **c)
            .into_iter()
            .map(|(ge0, group)| (ge0, group.cloned().collect::<Vec<_>>()))
            .collect()
    }
}

pub trait HandValue {
    fn get_bid(&self) -> u64;
}

impl<T> From<(Vec<u8>, u64, &str)> for Hand<T> {
    fn from((cards, bid, string): (Vec<u8>, u64, &str)) -> Self {
        Hand {
            phantom: Default::default(),
            cards,
            bid,
            line: string.to_owned(),
        }
    }
}

impl<T> PartialEq<Self> for Hand<T> {
    fn eq(&self, other: &Self) -> bool {
        let matching = self
            .cards
            .iter()
            .zip(&other.cards)
            .filter(|&(a, b)| a == b)
            .count();

        self.cards.len() == matching
    }
}

impl<T> Eq for Hand<T> {}
