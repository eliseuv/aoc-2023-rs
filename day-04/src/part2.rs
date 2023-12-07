use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, space0, space1},
    multi::fold_many1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};
use std::collections::HashSet;

#[derive(Debug)]
struct ScratchCard {
    _id: u32,
    winning: HashSet<u32>,
    scratched: HashSet<u32>,
}

impl ScratchCard {
    /// Find which of the scratched numbers are winnig numbers
    pub fn winning_numbers_count(&self) -> usize {
        self.scratched.intersection(&self.winning).count()
    }

    /// Calcualte score of the card
    pub fn score(&self) -> u32 {
        match self.winning_numbers_count().checked_sub(1) {
            Some(n) => 2u32.pow(n as u32),
            None => 0,
        }
    }
}

/// Parser that takes a list of integers separated by spaces and returns set
fn parse_number_set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

fn parse_card_id(input: &str) -> IResult<&str, u32> {
    delimited(
        tuple((tag("Card"), space1)),
        complete::u32,
        tuple((tag(":"), space1)),
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, ScratchCard> {
    let (input, id) = parse_card_id(input)?;

    separated_pair(
        parse_number_set,
        tuple((tag("|"), space1)),
        parse_number_set,
    )
    .map(|(winning, scratched)| ScratchCard {
        _id: id,
        winning,
        scratched,
    })
    .parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    // Parse all cards from input
    let cards: Vec<ScratchCard> = input
        .lines()
        .map(|line| {
            let (_, card) = parse_card(line).unwrap();
            card
        })
        .collect();

    let cards_count: usize = cards
        .iter()
        .enumerate()
        // We start with one of each card
        .fold(vec![1usize; cards.len()], |mut acc, (idx, card)| {
            let current_card_count = acc[idx];
            for i in (idx + 1)..(idx + 1 + card.winning_numbers_count()) {
                if let Some(count) = acc.get_mut(i) {
                    *count += current_card_count;
                }
            }
            acc
        })
        .iter()
        .sum();

    Ok(cards_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(input)?);
        Ok(())
    }
}
