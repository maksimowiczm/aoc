use std::marker::PhantomData;

pub mod camel_joker;
pub mod camel_poker;
mod r#impl;
pub mod poker;
mod shared;

#[allow(dead_code)]
pub struct Hand<T> {
    phantom: PhantomData<T>,
    cards: Vec<u8>,
    bid: u64,
    line: String,
}

pub trait HandValue<'a, T> {
    fn value(&'a self) -> Vec<T>;
}

pub trait HandBid {
    fn get_bid(&self) -> u64;
}

pub trait CardValue {
    fn get_char_as_card(ch: char) -> u8;
}
