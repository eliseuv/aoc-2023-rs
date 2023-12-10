use crate::custom_error::AocError;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::ParserExt;
use std::ops::Range;

// #[derive(Debug)]
// enum Category {
//     Seed(u64),
//     Soil(u64),
//     Fertilizer(u64),
//     Water(u64),
//     Light(u64),
//     Temperature(u64),
//     Humidity(u64),
//     Location(u64),
// }

#[derive(Debug)]
struct RangeMapping {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl RangeMapping {
    fn source_range(&self) -> Range<u64> {
        self.source_start..(self.source_start + self.length)
    }

    fn map(&self, num: u64) -> u64 {
        self.destination_start + num - self.source_start
    }
}

#[derive(Debug)]
struct Map {
    mappings: Vec<RangeMapping>,
}

impl Map {
    fn lookup(&self, num: u64) -> u64 {
        if let Some(range_mapping) = self
            .mappings
            .iter()
            .find(|range_mapping| range_mapping.source_range().contains(&num))
        {
            range_mapping.map(num)
        } else {
            num
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn lookup(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed| self.maps.iter().fold(*seed, |acc, map| map.lookup(acc)))
            .collect()
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    tuple((tag("seeds:"), space1))
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)
}

fn parse_range_mapping(input: &str) -> IResult<&str, RangeMapping> {
    tuple((
        complete::u64,
        complete::u64.preceded_by(space1),
        complete::u64.preceded_by(space1),
    ))(input)
    .map(|(input, (destination_start, source_start, length))| {
        (
            input,
            RangeMapping {
                destination_start,
                source_start,
                length,
            },
        )
    })
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(parse_range_mapping)))
        .parse(input)
        .map(|(input, mappings)| (input, Map { mappings }))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    tuple((parse_seeds, many1(parse_map)))
        .parse(input)
        .map(|(input, (seeds, maps))| (input, Almanac { seeds, maps }))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, almanac) = parse_almanac(input).unwrap();

    Ok(*almanac
        .lookup()
        .iter()
        .min()
        .expect("List of final locations should not be empty!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(35, process(input)?);
        Ok(())
    }
}
