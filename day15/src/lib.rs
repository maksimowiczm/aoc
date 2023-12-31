use book::Book;

use crate::label::Label;
use crate::lens::LensOperation;
use std::str::FromStr;

mod aoc_tests;
mod book;
mod label;
mod lens;
mod tests;

fn solution_01(input: &str) -> Result<u64, ()> {
    let sequence = input
        .split(",")
        .flat_map(Label::from_str)
        .collect::<Vec<_>>()
        .iter()
        .map(Label::hash)
        .sum();

    Ok(sequence)
}

fn solution_02(input: &str) -> Result<u64, ()> {
    let sequence = input
        .split(",")
        .flat_map(LensOperation::from_str)
        .collect::<Vec<_>>();

    let mut book = Book::new();
    book.execute_sequence(&sequence);
    Ok(book.get_focusing_power())
}
