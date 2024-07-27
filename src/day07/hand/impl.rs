use crate::day07::hand::{Hand, HandBid};
use itertools::Itertools;

impl<T> HandBid for Hand<T> {
    fn get_bid(&self) -> u64 {
        self.bid
    }
}

impl<T> Hand<T> {
    pub fn collect_groups(&self) -> Vec<(u8, Vec<u8>)> {
        self.cards
            .iter()
            .group_by(|c| **c)
            .into_iter()
            .map(|(ge0, group)| (ge0, group.cloned().collect::<Vec<_>>()))
            .collect()
    }
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
