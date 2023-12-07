use std::cmp::Ordering;

#[derive(Debug, Eq, PartialOrd, PartialEq, Copy, Clone)]
pub enum HandPokerValue {
    Five(u8),
    Four(u8),
    Full(u8, u8),
    Three(u8),
    TwoPair(u8, u8),
    Pair(u8),
    HighCard(u8),
}

impl HandPokerValue {
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
            HandPokerValue::Five(v) => Some(v),
            _ => None,
        }
    }

    fn four_value(&self) -> Option<&u8> {
        match self {
            HandPokerValue::Four(v) => Some(v),
            _ => None,
        }
    }

    fn full_values(&self) -> Option<(&u8, &u8)> {
        match self {
            HandPokerValue::Full(v1, v2) => Some((v1, v2)),
            _ => None,
        }
    }

    fn three_value(&self) -> Option<&u8> {
        match self {
            HandPokerValue::Three(v) => Some(v),
            _ => None,
        }
    }

    fn two_pair_value(&self) -> Option<(&u8, &u8)> {
        match self {
            HandPokerValue::TwoPair(v1, v2) => Some((v1, v2)),
            _ => None,
        }
    }

    fn pair_value(&self) -> Option<&u8> {
        match self {
            HandPokerValue::Pair(v) => Some(v),
            _ => None,
        }
    }

    fn high_card_value(&self) -> Option<&u8> {
        match self {
            HandPokerValue::HighCard(v) => Some(v),
            _ => None,
        }
    }
}

impl Ord for HandPokerValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            HandPokerValue::Five(v) => HandPokerValue::compare_values(Some(v), other.five_value()),
            HandPokerValue::Four(v) => match other {
                HandPokerValue::Five(_) => Ordering::Less,
                _ => HandPokerValue::compare_values(Some(v), other.four_value()),
            },
            HandPokerValue::Full(v1, v2) => match other {
                HandPokerValue::Five(_) | HandPokerValue::Four(_) => Ordering::Less,
                _ => HandPokerValue::compare_values_tuple(Some((v1, v2)), other.full_values()),
            },
            HandPokerValue::Three(v) => match other {
                HandPokerValue::Five(_) | HandPokerValue::Four(_) | HandPokerValue::Full(_, _) => {
                    Ordering::Less
                }
                _ => HandPokerValue::compare_values(Some(v), other.three_value()),
            },
            HandPokerValue::TwoPair(v1, v2) => match other {
                HandPokerValue::Five(_)
                | HandPokerValue::Four(_)
                | HandPokerValue::Full(_, _)
                | HandPokerValue::Three(_) => Ordering::Less,
                _ => HandPokerValue::compare_values_tuple(Some((v1, v2)), other.two_pair_value()),
            },
            HandPokerValue::Pair(v) => match other {
                HandPokerValue::Five(_)
                | HandPokerValue::Four(_)
                | HandPokerValue::Full(_, _)
                | HandPokerValue::Three(_)
                | HandPokerValue::TwoPair(_, _) => Ordering::Less,
                _ => HandPokerValue::compare_values(Some(v), other.pair_value()),
            },
            HandPokerValue::HighCard(v) => match other {
                HandPokerValue::Five(_)
                | HandPokerValue::Four(_)
                | HandPokerValue::Full(_, _)
                | HandPokerValue::Three(_)
                | HandPokerValue::TwoPair(_, _)
                | HandPokerValue::Pair(_) => Ordering::Less,
                _ => HandPokerValue::compare_values(Some(v), other.high_card_value()),
            },
        }
    }
}
