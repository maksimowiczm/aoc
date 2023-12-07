#[cfg(test)]
mod tests {
    use crate::hand::poker::hand_poker_value::HandPokerValue;
    use crate::hand::Hand;

    #[test]
    fn hand_value_five() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![1, 1, 1, 1, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Five(1));
    }

    #[test]
    fn hand_value_four_01() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![2, 2, 2, 2, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Four(2));
    }

    #[test]
    fn hand_value_four_02() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![2, 1, 1, 1, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Four(1));
    }

    #[test]
    fn hand_value_full_01() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![2, 2, 2, 1, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Full(2, 1));
    }

    #[test]
    fn hand_value_full_02() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![2, 2, 1, 1, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Full(1, 2));
    }

    #[test]
    fn hand_value_three_01() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![3, 3, 3, 2, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Three(3));
    }

    #[test]
    fn hand_value_three_02() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![3, 2, 2, 2, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Three(2));
    }

    #[test]
    fn hand_value_three_03() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![3, 2, 1, 1, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Three(1));
    }

    #[test]
    fn hand_value_pair_01() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![4, 4, 3, 2, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Pair(4));
    }

    #[test]
    fn hand_value_two_pair_01() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![4, 4, 3, 3, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::TwoPair(4, 3));
    }

    #[test]
    fn hand_value_pair_03() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![4, 3, 2, 1, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::Pair(1));
    }

    #[test]
    fn hand_value_high_card_01() {
        let hand: Hand<HandPokerValue> = Hand::from((vec![5, 4, 3, 2, 1], 0,  ""));
        assert_eq!(hand.value()[0], HandPokerValue::HighCard(5));
    }
}
