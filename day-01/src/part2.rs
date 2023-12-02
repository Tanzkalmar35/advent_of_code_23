use std::ops::Add;
use tracing_subscriber::fmt::format;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output =
        input.lines().map(process_line).sum::<u32>();
    Ok(output.to_string())
}

fn process_line(line: &str) -> u32 {
    let mut resulting: Vec<char> = vec![];
    for (index, c) in line.chars().enumerate() {
        let red = &line[index..];
        let result = if red.starts_with("one") || red.starts_with("1") {
            '1'
        } else if red.starts_with("two") {
            '2'
        } else if red.starts_with("three") {
            '3'
        } else if red.starts_with("four")  {
            '4'
        } else if red.starts_with("five") {
            '5'
        } else if red.starts_with("six") {
            '6'
        } else if red.starts_with("seven") {
            '7'
        } else if red.starts_with("eight") {
            '8'
        } else if red.starts_with("nine") {
            '9'
        } else if red.chars().next().unwrap().is_numeric() {
            red.chars().next().unwrap()
        } else {
            '0'
        };

        if result != '0' {
            resulting.push(result);
        }
    }
    let first = resulting[0];
    let last = resulting[resulting.len() - 1];

    format!("{}{}", first, last).parse().unwrap()
}

fn begins_with(line: &str, str: &str) -> bool {
    line.starts_with(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
