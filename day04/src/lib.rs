use crate::card::Cards;

mod card;
mod tests;

pub fn solution_01(input: &str) -> u128 {
    let cards = input.parse::<Cards>().unwrap().0;

    cards
        .iter()
        .map(|c| c.get_wins())
        .filter(|count| *count > 0)
        .map(|count| 2u128.pow(count as u32 - 1))
        .fold(0, |acc, v| acc + v)
}
