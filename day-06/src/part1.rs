use crate::custom_error::AocError;
use nom::character::complete::{self, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::Parser;
use nom::{bytes::complete::tag, IResult};
use nom_supreme::ParserExt;
use std::iter::zip;

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

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    separated_pair(
        tuple((tag("Time:"), space1)).precedes(separated_list1(space1, complete::u64)),
        line_ending,
        tuple((tag("Distance:"), space1)).precedes(separated_list1(space1, complete::u64)),
    )
    .parse(input)
    .map(|(input, (times, best_dists))| {
        let races = zip(times, best_dists)
            .map(|(time, best_dist)| Race {
                time,
                dist_record: best_dist,
            })
            .collect();
        (input, races)
    })
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, races) = parse_input(input).unwrap();

    Ok(races.iter().map(|race| race.margin_of_error()).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
