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

pub fn solution_02(input: &str) -> u128 {
    let cards = input.parse::<Cards>().unwrap().0;

    let won_proxies = cards
        .iter()
        .map(|c| c.get_wins())
        .enumerate()
        .map(|(i, wins)| (i + 1..i + 1 + wins).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counts = won_proxies.iter().map(|c| c.len()).collect::<Vec<_>>();

    won_proxies
        .iter()
        .enumerate()
        .rev()
        .for_each(|(i, cards_won)| {
            cards_won.iter().for_each(|card| {
                counts[i] += counts[*card];
            })
        });

    counts.iter().fold(0, |acc, v| acc + v) as u128 + cards.len() as u128
}
