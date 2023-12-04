use std::str::FromStr;

pub struct ParseCardErr {}

pub struct Card {
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

impl Card {
    pub fn get_wins(&self) -> usize {
        let mut c_winning = self.winning_numbers.clone();
        c_winning.sort();

        self.playing_numbers
            .iter()
            .map(|num| c_winning.binary_search(num))
            .flatten()
            .count()
    }
}

impl FromStr for Card {
    type Err = ParseCardErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited = s.trim().split("|").collect::<Vec<_>>();
        if splited.len() > 2 {
            return Err(ParseCardErr {});
        }

        let winning_numbers = splited[0]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u32>())
            .flatten()
            .collect::<Vec<_>>();

        let playing_numbers = splited[1]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u32>())
            .flatten()
            .collect::<Vec<_>>();

        Ok(Card {
            winning_numbers,
            playing_numbers,
        })
    }
}
