use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input
        .lines()// Gives us each line individually
        .map(|line| {
            // line.chars() effectively splits the line into single chars
            let mut iterator = line.chars().filter_map(|char| {
                // Try to cast the chars to a number of the decimal system
                char.to_digit(10) // Only numbers will be casted
            });
            let first_num = iterator.next().expect("This should be a number.");
            match iterator.last() {
                Some(num) => format!("{first_num}{num}"),
                None => format!("{first_num}{first_num}"), // If we only have one number
            }
                .parse::<u32>()
                .expect("This should be a valid number.")
        }).sum::<u32>(); // summing all lines
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
