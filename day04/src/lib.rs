use crate::card::Card;
use std::ops::ControlFlow;

mod card;
mod tests;

fn get_cards(input: &str) -> Vec<Card> {
    input
        .split("\n")
        .map(|line| {
            if let ControlFlow::Break(card_start) =
                line.chars().enumerate().try_for_each(|(i, ch)| {
                    if ch == ':' {
                        ControlFlow::Break(i)
                    } else {
                        ControlFlow::Continue(())
                    }
                })
            {
                Some(line.chars().skip(card_start + 1).collect::<String>())
            } else {
                None
            }
        })
        .flatten()
        .map(|str| str.parse::<Card>())
        .flatten()
        .collect()
}

pub fn solution_01(input: &str) -> u128 {
    let cards = get_cards(input);

    cards
        .iter()
        .map(|c| c.get_wins())
        .filter(|count| *count > 0)
        .map(|count| 2u128.pow(count as u32 - 1))
        .fold(0, |acc, v| acc + v)
}
