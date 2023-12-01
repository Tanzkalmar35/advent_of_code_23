use std::ptr::null;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|char| {
                char.to_digit(10)
            });
            let first_num = it.next().expect("This should be a number.");
            match it.last() {
                Some(num) => format!("{first_num}{num}"),
                None => format!("{first_num}{first_num}"),
            }
                .parse::<u32>()
                .expect("This should be a valid number.")
        }).sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
