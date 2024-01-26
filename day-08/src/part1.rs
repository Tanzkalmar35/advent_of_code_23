use nom::InputTakeAtPosition;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (instr, seq) = split_instr_and_seq(&input);
    let res = seq.iter().map(|line| extract_dir);
    for instr in instr.split("") {
        match instr {
            String::from("L") => {

            }
            String::from("R") => {

            }
            _ => {}
        }
    }
    todo!()
}

pub fn extract_dir(line: &str) -> Vec<&str> {
    let right_path = line.split_whitespace().nth(3).unwrap();
    let left_path = line.split_whitespace().nth(2).unwrap();
    vec![left_path, right_path]

}

fn split_instr_and_seq(input: &str) -> (Vec<String> ,Vec<String>) {
    let lines = input.lines();
    (lines[0], lines[2..])
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
