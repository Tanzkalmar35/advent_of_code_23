use nom::InputTakeAtPosition;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let vec = split_instr_and_seq(&input);
    let res = vec.iter().map(|line| process_line);
    todo!()
}

pub fn process_line(line: &str) {

}

fn split_instr_and_seq(input: &str) -> Vec<Vec<&str>> {
    let result: Vec<Vec<&str>> = input.lines().fold(vec![], |mut acc, line| {
        if line.trim().is_empty() {
            acc.push(vec![]);
        } else {
            let last: &mut Vec<&str> = acc.last_mut().unwrap();
            last.push(line);
        }
        acc
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
