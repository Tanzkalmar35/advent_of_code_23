use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let output =
        input.lines().map(process_line).sum::<u32>();

    Ok(output.to_string())
}

fn process_line(line: &str) -> u32 {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || Some({
        let reduced_line = &line[index..];

        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
             //let result = reduced_line.chars().next();
            //result.unwrap()
            '0'
        };
        index += 1;
        result
    }));
    let mut it = line_iter.filter_map(|char| char.to_digit(10));
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
        .parse::<u32>()
        .expect("should be a valid number")
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
