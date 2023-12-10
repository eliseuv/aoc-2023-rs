use crate::custom_error::AocError;
use nom::character::complete::{digit1, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::Parser;
use nom::{bytes::complete::tag, IResult};
use nom_supreme::ParserExt;

#[derive(Debug)]
struct Race {
    time: u64,
    dist_record: u64,
}

impl Race {
    // The margin of error of a race is defined as how many ways there is to beat the best distance
    fn margin_of_error(&self) -> u64 {
        let length = ((self.time.pow(2) - 4 * self.dist_record) as f64).sqrt();
        let length_floor = length.floor();
        let range_start = (self.time as f64 - length) / 2.0;

        match (length - length_floor) + (range_start - range_start.floor()) {
            test if test == 0.0 => length_floor as u64 - 1,
            test if test < 1.0 => length_floor as u64,
            _ => length_floor as u64 + 1,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Race> {
    separated_pair(
        tuple((tag("Time:"), space1)).precedes(separated_list1(space1, digit1)),
        line_ending,
        tuple((tag("Distance:"), space1)).precedes(separated_list1(space1, digit1)),
    )
    .parse(input)
    .map(|(input, (time_chunks, dist_record_chunks))| {
        let time: u64 = time_chunks
            .iter()
            .fold(String::new(), |mut acc, x| {
                acc.push_str(x);
                acc
            })
            .parse()
            .unwrap();
        let dist_record: u64 = dist_record_chunks
            .iter()
            .fold(String::new(), |mut acc, x| {
                acc.push_str(x);
                acc
            })
            .parse()
            .unwrap();

        (input, Race { time, dist_record })
    })
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, race) = parse_input(input).unwrap();

    Ok(race.margin_of_error())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
