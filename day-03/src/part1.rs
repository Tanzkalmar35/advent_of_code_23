use std::str::Lines;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = process_line(input);
    Ok(result.to_string())
}

// input:
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
fn process_line(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut result = 0;
    for (line_idx, line) in lines.clone().enumerate() { // loop through lines
        for (char_idx, char) in line.chars().enumerate() { // loop through chars of line
            if char.is_numeric() {
                let first_digit_adjacent = is_adjacent_to_symbol(&mut lines, line_idx, char_idx);
                if line.chars().nth(char_idx + 1).unwrap().is_numeric() { // if the number is at least 2 digits
                    let second_digit_adjacent = is_adjacent_to_symbol(&mut lines, line_idx, char_idx + 1);
                    if line.chars().nth(char_idx + 2).unwrap().is_numeric() { // if the number is 3 digits
                        let third_digit_adjacent = is_adjacent_to_symbol(&mut lines, line_idx, char_idx + 2);
                        if third_digit_adjacent {
                            result +=
                                format!("{}{}{}",
                                        line.chars().nth(char_idx).unwrap(),
                                        line.chars().nth(char_idx + 1).unwrap(),
                                        line.chars().nth(char_idx + 2).unwrap())
                                    .parse::<u32>().unwrap();
                        }
                    } else {
                        if second_digit_adjacent {
                            result +=
                                format!("{}{}",
                                        line.chars().nth(char_idx).unwrap(),
                                        line.chars().nth(char_idx + 1).unwrap())
                                    .parse::<u32>().unwrap();
                        }
                    }
                } else {
                    if first_digit_adjacent {
                        result +=
                            format!("{}",
                                    line.chars().nth(char_idx).unwrap())
                                .parse::<u32>().unwrap();
                    }
                }
            }
        }
    }
    result
}

fn is_adjacent_to_symbol(lines: &mut Lines, line_idx: usize, char_idx: usize) -> bool {
    is_adjacent_above(lines, line_idx, char_idx) ||
        is_adjacent_next_to(lines, line_idx, char_idx) ||
        is_adjacent_below(lines, line_idx, char_idx)
}

fn is_adjacent_above(lines: &mut Lines, line_idx: usize, char_idx: usize) -> bool {
    let mut is_adjacent: bool = false;
    let mut line_above: &str = "";
    if line_idx != 0 {
        line_above = lines.nth(line_idx - 1).unwrap();
    }
    if !line_above.eq("") {
        for _i in 0..3 {
            if char_idx != 0 {
                is_adjacent = is_symbol(line_above.chars().nth(char_idx - 1).unwrap()) ||
                    is_symbol(line_above.chars().nth(char_idx).unwrap()) ||
                    is_symbol(line_above.chars().nth(char_idx + 1).unwrap());
            }
        }
    }
    is_adjacent
}

fn is_adjacent_next_to(lines: &mut Lines, line_idx: usize, char_idx: usize) -> bool {
    let mut is_adjacent: bool = false;
    let mut line = "";
    if line_idx > 0 {
        line = lines.nth(line_idx).unwrap();
    }
    if char_idx != 0 {
        is_adjacent = is_symbol(line.chars().nth(char_idx - 1).unwrap()) ||
            is_symbol(line.chars().nth(char_idx + 1).unwrap());
    }
    is_adjacent
}

fn is_adjacent_below(lines: &mut Lines, line_idx: usize, char_idx: usize) -> bool {
    let mut is_adjacent: bool = false;
    let mut line_below: &str = "";
    let line_count = lines.count() + 1;
    if line_idx < line_count {
        //line_below = lines.nth(line_idx + 1).unwrap(); // Fails at .unwrap()
        if let Some(line) = lines.nth(line_idx + 1) {
            let lin = line;
            line_below = line;
        } else {
            return false; // or handle the error in some other way
        }
    }
    if !line_below.eq("") {
        for _i in 0..3 {
            if char_idx != 0 {
                is_adjacent = is_symbol(line_below.chars().nth(char_idx - 1).unwrap()) ||
                    is_symbol(line_below.chars().nth(char_idx).unwrap()) ||
                    is_symbol(line_below.chars().nth(char_idx + 1).unwrap());
            }
        }
    }
    is_adjacent
}

fn is_symbol(char: char) -> bool {
    !char.is_numeric() && !char.eq(&'.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
