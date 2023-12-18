use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process_line(line: &str) -> miette::Result<usize, AocError> {
    todo!("Part 1");
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    todo!("Part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_lines(#[case] line: &str, #[case] expected: usize) -> miette::Result<()> {
        assert_eq!(expected, process_line(line)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(21, process(input)?);
        Ok(())
    }
}
