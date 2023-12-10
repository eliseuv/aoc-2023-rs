use nom::{
    bytes::complete::take,
    character::complete::{self, one_of, space1},
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy)]
enum Card {
    N2 = 2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    dbg!(separated_pair(
        take()
        tuple((
            one_of("23456789TJQKA"),
            one_of("23456789TJQKA"),
            one_of("23456789TJQKA"),
            one_of("23456789TJQKA"),
            one_of("23456789TJQKA"),
        )),
        space1,
        complete::u32
    )(input).map(|(input, (suits, bid))| {
        (input, Hand { cards: suits.map, bid })
    })?);

    Ok((
        "",
        Hand {
            cards: [Card::A; 5],
            bid: 0,
        },
    ))
}

#[tracing::instrument]
pub fn process_line(line: &str) -> miette::Result<u32, AocError> {
    let (_, hand) = parse_hand(line).unwrap();

    Ok(0)
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
    #[case("32T3K", 765)]
    #[case("T55J5", 684)]
    #[case("KK677", 28)]
    #[case("KTJJT", 220)]
    #[case("QQQJA", 483)]
    fn test_lines(#[case] line: &str, #[case] expected: u32) -> miette::Result<()> {
        assert_eq!(expected, process_line(line)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(0, process(input)?);
        Ok(())
    }
}
