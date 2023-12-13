use crate::custom_error::AocError;
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

fn parse_list(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, parse_list)(input)
}

#[derive(Debug)]
struct SensorReading {
    data: Vec<Vec<i32>>,
}

impl SensorReading {
    fn new(readings: &[i32]) -> Self {
        Self {
            data: vec![readings.to_owned()],
        }
    }

    fn get_diffs(&mut self) {
        while !self.data.last().unwrap().iter().all_equal() {
            self.data.push(diff(self.data.last().unwrap()));
        }
    }

    fn extrapolate_forward(&mut self) -> i32 {
        if !self.data.last().unwrap().iter().all_equal() {
            self.get_diffs();
        }
        let n = self.data.len();
        for k in (0..n - 1).rev() {
            let x = self.data[k].last().unwrap().to_owned();
            let dx = self.data[k + 1].last().unwrap().to_owned();
            self.data[k].push(x + dx)
        }

        *self.data.first().unwrap().last().unwrap()
    }

    fn extrapolate_backward(&mut self) -> i32 {
        if !self.data.last().unwrap().iter().all_equal() {
            self.get_diffs();
        }
        let n = self.data.len();
        for k in (0..n - 1).rev() {
            let x = self.data[k].first().unwrap().to_owned();
            let dx = self.data[k + 1].first().unwrap().to_owned();
            self.data[k].insert(0, x - dx)
        }

        *self.data.first().unwrap().first().unwrap()
    }
}

fn diff(list: &[i32]) -> Vec<i32> {
    list.array_windows::<2>().map(|[a, b]| b - a).collect()
}

#[tracing::instrument]
pub fn process_line(line: &str) -> miette::Result<i32, AocError> {
    let (_, list) = dbg!(parse_list(line).unwrap());

    let mut sensor = SensorReading::new(&list);
    let result = sensor.extrapolate_backward();

    Ok(result)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    let (_, lists) = parse_input(input).unwrap();

    let result = lists
        .iter()
        .map(|list| {
            let mut sensor = SensorReading::new(list);
            sensor.extrapolate_backward()
        })
        .sum::<i32>();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("10 13 16 21 30 45", 5)]
    fn test_lines(#[case] line: &str, #[case] expected: i32) -> miette::Result<()> {
        assert_eq!(expected, process_line(line)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, process(input)?);
        Ok(())
    }
}
