use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process_line(line: &str) -> miette::Result<u32, AocError> {
    todo!("Part 1");
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    todo!("Part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", 0)]
    fn test_lines(#[case] line: &str, #[case] expected: u32) -> miette::Result<()> {
        todo!("haven't built test yet");
        assert_eq!(expected, process_line(line)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!((), process(input)?);
        Ok(())
    }
}
