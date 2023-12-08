#[cfg(test)]
mod tests {
    use crate::hand::poker::poker_hand_value::PokerHandValue;
    use crate::hand::Hand;

    #[test]
    fn hand_value_five() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![1, 1, 1, 1, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Five(1));
    }

    #[test]
    fn hand_value_four_01() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![2, 2, 2, 2, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Four(2));
    }

    #[test]
    fn hand_value_four_02() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![2, 1, 1, 1, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Four(1));
    }

    #[test]
    fn hand_value_full_01() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![2, 2, 2, 1, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Full(2, 1));
    }

    #[test]
    fn hand_value_full_02() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![2, 2, 1, 1, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Full(1, 2));
    }

    #[test]
    fn hand_value_three_01() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![3, 3, 3, 2, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Three(3));
    }

    #[test]
    fn hand_value_three_02() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![3, 2, 2, 2, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Three(2));
    }

    #[test]
    fn hand_value_three_03() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![3, 2, 1, 1, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Three(1));
    }

    #[test]
    fn hand_value_pair_01() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![4, 4, 3, 2, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Pair(4));
    }

    #[test]
    fn hand_value_two_pair_01() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![4, 4, 3, 3, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::TwoPair(4, 3));
    }

    #[test]
    fn hand_value_pair_03() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![4, 3, 2, 1, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::Pair(1));
    }

    #[test]
    fn hand_value_high_card_01() {
        let hand: Hand<PokerHandValue> = Hand::from((vec![5, 4, 3, 2, 1], 0, ""));
        assert_eq!(hand.value()[0], PokerHandValue::HighCard(5));
    }
}
