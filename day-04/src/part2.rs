use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    todo!("count all cards, including copies")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!();
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
