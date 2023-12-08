use std::cmp::Ordering;
use std::collections::HashMap;

use crate::hand::camel_joker_poker::hand_camel_joker_value::HandCamelJokerValue::{
    Five, Four, Full, HighCard, Pair, Three, TwoPair,
};
use crate::hand::CardValue;

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone)]
pub enum HandCamelJokerValue {
    Five(String),
    Four(String),
    Full(String),
    Three(String),
    TwoPair(String),
    Pair(String),
    HighCard(String),
}

impl CardValue for HandCamelJokerValue {
    fn get_char_as_card(ch: char) -> u8 {
        let mut has_card = HashMap::<char, u8>::new();
        has_card.insert('A', 15);
        has_card.insert('K', 13);
        has_card.insert('Q', 12);
        has_card.insert('J', 1);
        has_card.insert('T', 10);

        if ch.is_digit(10) {
            ch as u8 - 48
        } else {
            *has_card.get(&ch).unwrap()
        }
    }
}

macro_rules! value_function {
    ($fn_name:tt, $wrapper:ident) => {
        fn $fn_name(&self) -> Option<&str> {
            match self {
                $wrapper(s) => Some(s),
                _ => None,
            }
        }
    };
}

impl HandCamelJokerValue {
    value_function!(five_value, Five);
    value_function!(four_value, Four);
    value_function!(full_values, Full);
    value_function!(three_value, Three);
    value_function!(two_pair_value, TwoPair);
    value_function!(pair_value, Pair);
    value_function!(high_card_value, HighCard);

    fn compare_values(a: Option<&str>, b: Option<&str>) -> Ordering {
        match (a, b) {
            (Some(a), Some(b)) => {
                let orders = a
                    .chars()
                    .take(5)
                    .zip(b.chars().take(5))
                    .flat_map(|(a, b)| {
                        let a_value = HandCamelJokerValue::get_char_as_card(a);
                        let b_value = HandCamelJokerValue::get_char_as_card(b);
                        let order = a_value.cmp(&b_value);
                        if order == Ordering::Equal {
                            None
                        } else {
                            Some(order)
                        }
                    })
                    .collect::<Vec<_>>();

                *orders.get(0).unwrap_or(&Ordering::Equal)
            }
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
        }
    }
}

impl Ord for HandCamelJokerValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Five(s) => HandCamelJokerValue::compare_values(Some(s), other.five_value()),
            Four(s) => match other {
                Five(_) => Ordering::Less,
                _ => HandCamelJokerValue::compare_values(Some(s), other.four_value()),
            },
            Full(s) => match other {
                Five(_) | Four(_) => Ordering::Less,
                _ => HandCamelJokerValue::compare_values(Some(s), other.full_values()),
            },
            Three(s) => match other {
                Five(_) | Four(_) | Full(_) => Ordering::Less,
                _ => HandCamelJokerValue::compare_values(Some(s), other.three_value()),
            },
            TwoPair(s) => match other {
                Five(_) | Four(_) | Full(_) | Three(_) => Ordering::Less,
                _ => HandCamelJokerValue::compare_values(Some(s), other.two_pair_value()),
            },
            Pair(s) => match other {
                Five(_) | Four(_) | Full(_) | Three(_) | TwoPair(_) => Ordering::Less,
                _ => HandCamelJokerValue::compare_values(Some(s), other.pair_value()),
            },
            HighCard(s) => match other {
                Five(_) | Four(_) | Full(_) | Three(_) | TwoPair(_) | Pair(_) => Ordering::Less,
                _ => HandCamelJokerValue::compare_values(Some(s), other.high_card_value()),
            },
        }
    }
}
