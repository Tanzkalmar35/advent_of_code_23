use itertools::Itertools;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: u32 = input.lines().flat_map(|line| process_line(line)).sum();

    Ok(output.to_string())
}

fn process_line(line: &str) -> Option<u32> {
    let mut card_score: u32 = 0;
    let (winning_numbers, actual_numbers) = line.split(":").nth(1)?.split("|").next_tuple()?;

    for winning_number in winning_numbers.split_whitespace() {
        for actual_number in actual_numbers.split_whitespace() {
            if str_is_numeric(actual_number) && str_is_numeric(winning_number) {
                if actual_number == winning_number {
                    if card_score == 0 {
                        card_score = 1;
                    } else if card_score > 0 {
                        card_score = card_score * 2;
                    }
                }
            }
        }
    }
    Some(card_score)
}

fn str_is_numeric(str: &str) -> bool {
    str.chars().next().unwrap().is_numeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let string = process(input)?;
        assert_eq!("13", string);
        Ok(())
    }
}
