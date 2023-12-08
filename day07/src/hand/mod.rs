use std::marker::PhantomData;

pub mod camel_joker_poker;
pub mod camel_poker;
mod r#impl;
pub mod poker;

pub struct Hand<T> {
    phantom: PhantomData<T>,
    cards: Vec<u8>,
    bid: u64,
    line: String,
}

pub trait HandBid {
    fn get_bid(&self) -> u64;
}

pub trait CardValue {
    fn get_char_as_card(ch: char) -> u8;
}
