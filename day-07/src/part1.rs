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

#[derive(Debug)]
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
    hand_type: Types,
    bid: u32,
}

impl Hand {
    pub fn with_bid(hand: String, bid: u32) -> Self {
        Self { hand, hand_type: Types::None, bid }
    }

    pub fn get_type(mut self) -> Types {
        if self.is_high_card() {
            self.hand_type = Types::HighCard;
        } else if self.is_one_pair() {
            self.hand_type = Types::OnePair;
        } else if self.is_two_pair() {
            self.hand_type = Types::TwoPair;
        } else if self.is_three_of_a_kind() {
            self.hand_type = Types::ThreeOfAKind;
        } else if self.is_full_house() {
            self.hand_type = Types::FullHouse;
        } else if self.is_four_of_a_kind() {
            self.hand_type = Types::FourOfAKind;
        } else if self.is_five_of_a_kind() {
            self.hand_type = Types::FiveOfAKind;
        } else {
            self.hand_type = Types::None;
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
        self.is_n_of_a_kind(1, 3) ||
            self.is_n_of_a_kind(1, 2)
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
    let it = input.lines().map(|line| {
        let hand = line.split_whitespace().nth(0).unwrap();
        let bid = line.split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
        let from_hand = Hand::with_bid(String::from(hand), bid);
        // TODO: place the hand types in a vec, sort them and calc values, then sum
    });
    for line in input.lines() {

    }
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
