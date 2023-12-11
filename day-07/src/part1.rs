use std::collections::HashMap;
use itertools::Itertools;
use nom::Parser;
use crate::custom_error::AocError;

enum Cards {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

#[derive(Debug, PartialEq)]
enum Types {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    None = 0,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    hand_type: u32,
    bid: u32,
}

impl Hand {
    pub fn from_hand_with_bid(hand: String, bid: u32) -> Self {
        Self { hand, hand_type: Types::None as u32, bid }
    }

    pub fn get_type(mut self) -> u32 {
        if self.is_five_of_a_kind() {
            self.hand_type = Types::FiveOfAKind as u32;
        } else if self.is_four_of_a_kind() {
            self.hand_type = Types::FourOfAKind as u32;
        } else if self.is_full_house() {
            self.hand_type = Types::FullHouse as u32;
        } else if self.is_three_of_a_kind() {
            self.hand_type = Types::ThreeOfAKind as u32;
        } else if self.is_two_pair() {
            self.hand_type = Types::TwoPair as u32;
        } else if self.is_one_pair() {
            self.hand_type = Types::OnePair as u32;
        } else if self.is_high_card() {
            self.hand_type = Types::HighCard as u32;
        } else {
            self.hand_type = Types::None as u32;
        }
        self.hand_type
    }

    /// Checks whether the hand is a HighCard (Value 1)
    pub fn is_high_card(&self) -> bool {
        let mut char_counts = HashMap::new();
        for char in self.hand.chars() {
            *char_counts.entry(char).or_insert(0) += 1;
        }
        for &count in char_counts.values() {
            if count >= 2 {
                return false
            }
        }
        true
    }

    /// Checks whether the hand is a OnePair (Value 2)
    fn is_one_pair(&self) -> bool {
        self.is_n_of_a_kind(1, 2)
    }

    /// Checks whether the hand is a TwoPair (Value 3)
    fn is_two_pair(&self) -> bool {
        self.is_n_of_a_kind(2, 2)
    }

    /// Checks whether the hand contains three of a kind (Value 4)
    fn is_three_of_a_kind(&self) -> bool {
        self.is_n_of_a_kind(1, 3)
    }

    /// Checks if the hand is a full house (Value 5)
    fn is_full_house(&self) -> bool {
        let mut char_counts = HashMap::new();
        for char in self.hand.chars() {
            *char_counts.entry(char).or_insert(0) += 1;
        }
        let i = char_counts.len() == 2;
        let x = char_counts.values().nth(1).unwrap() == &3i32;
        i && x
    }

    /// Checks whether the hand contains four of a kind (Value 6)
    fn is_four_of_a_kind(&self) -> bool {
        self.is_n_of_a_kind(1, 4)
    }

    /// Checks whether the hand contains five of a kind (Value 7)
    fn is_five_of_a_kind(&self) -> bool {
        self.is_n_of_a_kind(1, 5)
    }

    /// Checks if the hand contains a given number of pairs
    fn is_n_of_a_kind(&self, n_count: u32, n: u32) -> bool {
        let mut char_counts = HashMap::new();
        for char in self.hand.chars() {
            *char_counts.entry(char).or_insert(0) += 1;
        }
        let mut n_of_a_kind_count = 0;
        for &same_char_count in char_counts.values() {
            if same_char_count == n {
                n_of_a_kind_count += 1;
            } else if same_char_count > n {
                return false
            }
        }
        n_of_a_kind_count == n_count
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut res_map = vec![];
    for line in input.lines() {
        let hand = line.split_whitespace().nth(0).unwrap();
        let bid = line.split_whitespace().nth(1).unwrap().parse::<u32>().expect("should be a number");
        let hand_type = Hand::from_hand_with_bid(String::from(hand), bid).get_type();
        res_map.push(hand_type)
    }
    res_map.sort();
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_card() -> miette::Result<()> {
        let input = String::from("23456");
        assert_eq!(Types::HighCard, Hand::from_hand_with_bid(input, 0).get_type());
        Ok(())
    }

    #[test]
    fn test_one_pair() -> miette::Result<()> {
        let input = String::from("A23A4");
        assert_eq!(Types::OnePair, Hand::from_hand_with_bid(input, 0).get_type());
        Ok(())
    }

    #[test]
    fn test_two_pair() -> miette::Result<()> {
        let input = String::from("23432");
        assert_eq!(Types::TwoPair, Hand::from_hand_with_bid(input, 0).get_type());
        Ok(())
    }

    #[test]
    fn test_three_of_a_kind() -> miette::Result<()> {
        let input = String::from("TTT98");
        assert_eq!(Types::ThreeOfAKind, Hand::from_hand_with_bid(input, 0).get_type());
        Ok(())
    }

    #[test]
    fn test_full_house() -> miette::Result<()> {
        let input = String::from("23332");
        assert_eq!(Types::FullHouse, Hand::from_hand_with_bid(input, 0).get_type());
        Ok(())
    }

    #[test]
    fn test_four_of_a_kind() -> miette::Result<()> {
        let input = String::from("AA8AA");
        assert_eq!(Types::FourOfAKind, Hand::from_hand_with_bid(input, 0).get_type());
        Ok(())
    }

    #[test]
    fn five_of_a_kind() -> miette::Result<()> {
        let input = String::from("AAAAA");
        assert_eq!(Types::FiveOfAKind, Hand::from_hand_with_bid(input, 0).get_type());
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
