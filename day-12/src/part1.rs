use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, one_of, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
}

impl State {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Operational),
            '#' => Some(Self::Damaged),
            '?' => None,
            _ => panic!("Unrecognized character!"),
        }
    }

    fn matches(&self, s: &Option<State>) -> bool {
        match s {
            None => true,
            Some(x) => self == x,
        }
    }
}

// TODO: Can `Rc<[T]>` be used instead of `Vec<T>`?
#[derive(Debug)]
struct DamagedRecord {
    state: Vec<Option<State>>,
    grouping: Vec<usize>,
}

#[derive(Debug)]
enum GroupState {
    Operation(usize),
    Damaged(usize),
}

#[derive(Debug)]
struct Record {
    grouping: Vec<usize>,
    fill: Vec<usize>,
}

impl Record {
    fn new(damaged_record: &DamagedRecord) -> Self {
        let grouping = damaged_record.grouping.clone();
        let n = grouping.len() + 1;
        let mut fill = vec![1; n];
        fill[0] = 0;
        fill[n - 1] = damaged_record.state.len() - grouping.iter().sum::<usize>() - n + 2;

        Self { grouping, fill }
    }

    fn len(&self) -> usize {
        self.grouping.iter().sum::<usize>() + self.fill.iter().sum::<usize>()
    }

    fn state(&self) -> Vec<State> {
        self.grouping
            .iter()
            .zip(self.fill.iter())
            .flat_map(|(g, f)| {
                vec![State::Operational; *f]
                    .into_iter()
                    .chain(vec![State::Damaged; *g])
            })
            .chain(vec![State::Operational; *self.fill.last().unwrap()])
            .collect()
    }
}

fn parse_record(input: &str) -> IResult<&str, DamagedRecord> {
    separated_pair(
        many1(one_of(".#?")),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)
    .map(|(input, (springs_chars, grouping))| {
        (
            input,
            DamagedRecord {
                state: springs_chars.into_iter().map(State::from_char).collect(),
                grouping: grouping.into_iter().map(|x| x as usize).collect(),
            },
        )
    })
}

#[tracing::instrument]
pub fn process_line(line: &str) -> miette::Result<usize, AocError> {
    let (_, damaged_record) = dbg!(parse_record(line).unwrap());
    let record = Record::new(&damaged_record);
    assert_eq!(record.len(), damaged_record.state.len());

    Ok(0)
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
