use std::cmp::Ordering;

#[derive(Debug, Eq, PartialOrd, PartialEq, Copy, Clone)]
pub enum PokerHandValue {
    Five(u8),
    Four(u8),
    Full(u8, u8),
    Three(u8),
    TwoPair(u8, u8),
    Pair(u8),
    HighCard(u8),
}

impl PokerHandValue {
    fn compare_values(a: Option<&u8>, b: Option<&u8>) -> Ordering {
        match (a, b) {
            (Some(a), Some(b)) => a.cmp(b),
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
        }
    }

    fn compare_values_tuple(a: Option<(&u8, &u8)>, b: Option<(&u8, &u8)>) -> Ordering {
        match (a, b) {
            (Some((a1, a2)), Some((b1, b2))) => {
                let first = a1.cmp(b1);
                if first != Ordering::Equal {
                    first
                } else {
                    a2.cmp(b2)
                }
            }
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
        }
    }

    fn five_value(&self) -> Option<&u8> {
        match self {
            PokerHandValue::Five(v) => Some(v),
            _ => None,
        }
    }

    fn four_value(&self) -> Option<&u8> {
        match self {
            PokerHandValue::Four(v) => Some(v),
            _ => None,
        }
    }

    fn full_values(&self) -> Option<(&u8, &u8)> {
        match self {
            PokerHandValue::Full(v1, v2) => Some((v1, v2)),
            _ => None,
        }
    }

    fn three_value(&self) -> Option<&u8> {
        match self {
            PokerHandValue::Three(v) => Some(v),
            _ => None,
        }
    }

    fn two_pair_value(&self) -> Option<(&u8, &u8)> {
        match self {
            PokerHandValue::TwoPair(v1, v2) => Some((v1, v2)),
            _ => None,
        }
    }

    fn pair_value(&self) -> Option<&u8> {
        match self {
            PokerHandValue::Pair(v) => Some(v),
            _ => None,
        }
    }

    fn high_card_value(&self) -> Option<&u8> {
        match self {
            PokerHandValue::HighCard(v) => Some(v),
            _ => None,
        }
    }
}

impl Ord for PokerHandValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PokerHandValue::Five(v) => PokerHandValue::compare_values(Some(v), other.five_value()),
            PokerHandValue::Four(v) => match other {
                PokerHandValue::Five(_) => Ordering::Less,
                _ => PokerHandValue::compare_values(Some(v), other.four_value()),
            },
            PokerHandValue::Full(v1, v2) => match other {
                PokerHandValue::Five(_) | PokerHandValue::Four(_) => Ordering::Less,
                _ => PokerHandValue::compare_values_tuple(Some((v1, v2)), other.full_values()),
            },
            PokerHandValue::Three(v) => match other {
                PokerHandValue::Five(_) | PokerHandValue::Four(_) | PokerHandValue::Full(_, _) => {
                    Ordering::Less
                }
                _ => PokerHandValue::compare_values(Some(v), other.three_value()),
            },
            PokerHandValue::TwoPair(v1, v2) => match other {
                PokerHandValue::Five(_)
                | PokerHandValue::Four(_)
                | PokerHandValue::Full(_, _)
                | PokerHandValue::Three(_) => Ordering::Less,
                _ => PokerHandValue::compare_values_tuple(Some((v1, v2)), other.two_pair_value()),
            },
            PokerHandValue::Pair(v) => match other {
                PokerHandValue::Five(_)
                | PokerHandValue::Four(_)
                | PokerHandValue::Full(_, _)
                | PokerHandValue::Three(_)
                | PokerHandValue::TwoPair(_, _) => Ordering::Less,
                _ => PokerHandValue::compare_values(Some(v), other.pair_value()),
            },
            PokerHandValue::HighCard(v) => match other {
                PokerHandValue::Five(_)
                | PokerHandValue::Four(_)
                | PokerHandValue::Full(_, _)
                | PokerHandValue::Three(_)
                | PokerHandValue::TwoPair(_, _)
                | PokerHandValue::Pair(_) => Ordering::Less,
                _ => PokerHandValue::compare_values(Some(v), other.high_card_value()),
            },
        }
    }
}
