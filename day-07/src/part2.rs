use itertools::Itertools;
use nom::character::complete::{self, line_ending, one_of, space1};
use nom::multi::{many1, separated_list1};
use nom::{sequence::separated_pair, IResult};
use std::cmp::{Ordering, Reverse};
use std::iter::zip;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
enum CardLabel {
    J, // Joker is the weakest individual card label
    N2 = 2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl CardLabel {
    fn from_char(c: &char) -> Self {
        match c {
            '2' => CardLabel::N2,
            '3' => CardLabel::N3,
            '4' => CardLabel::N4,
            '5' => CardLabel::N5,
            '6' => CardLabel::N6,
            '7' => CardLabel::N7,
            '8' => CardLabel::N8,
            '9' => CardLabel::N9,
            'T' => CardLabel::T,
            'J' => CardLabel::J,
            'Q' => CardLabel::Q,
            'K' => CardLabel::K,
            'A' => CardLabel::A,
            _ => panic!("Unrecognized card character!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [CardLabel; 5],
    bid: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let joker_count = self
            .cards
            .iter()
            .filter(|label| *label == &CardLabel::J)
            .count();

        let mut regular_counts: Vec<usize> = CardLabel::iter()
            .filter(|label| label != &CardLabel::J)
            .map(|card_label| {
                self.cards
                    .iter()
                    .filter(|card| **card == card_label)
                    .count()
            })
            .filter(|n| *n > 0)
            .sorted_unstable_by_key(|w| Reverse(*w))
            .collect();

        let final_counts = match joker_count {
            0 => regular_counts,
            n => match regular_counts[..] {
                [] => vec![n],
                _ => {
                    regular_counts[0] += n;
                    regular_counts
                }
            },
        };

        match final_counts[..] {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Unrecognized hand count!"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.get_type().partial_cmp(&other.get_type()) {
            Some(Ordering::Equal) => Some(
                zip(&self.cards, &other.cards)
                    .find(|(self_card, other_card)| self_card != other_card)
                    .map(|(self_card, other_card)| self_card.cmp(other_card))
                    .unwrap_or(Ordering::Equal),
            ),
            x => x,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => zip(&self.cards, &other.cards)
                .find(|(self_card, other_card)| self_card != other_card)
                .map(|(self_card, other_card)| self_card.cmp(other_card))
                .unwrap_or(Ordering::Equal),

            x => x,
        }
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    separated_pair(many1(one_of("23456789TJQKA")), space1, complete::u32)(input).map(
        |(input, (cards, bid))| {
            (
                input,
                Hand {
                    cards: cards
                        .iter()
                        .map(CardLabel::from_char)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                    bid,
                },
            )
        },
    )
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}

#[tracing::instrument]
pub fn process_line(line: &str) -> miette::Result<Hand, AocError> {
    let (_, hand) = parse_hand(line).unwrap();

    Ok(hand)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_, hands) = parse_hands(input).expect("Sample input parsing should work!");

    let result = hands
        .iter()
        .sorted_unstable()
        .enumerate()
        .map(|(n, hand)| (n + 1) as u32 * hand.bid)
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("32T3K 765", Hand{ cards: [CardLabel::N3, CardLabel::N2, CardLabel::T, CardLabel::N3, CardLabel::K], bid: 765 })]
    #[case("T55J5 684", Hand{ cards: [CardLabel::T, CardLabel::N5, CardLabel::N5, CardLabel::J, CardLabel::N5], bid: 684 })]
    #[case("KK677 28", Hand{ cards: [CardLabel::K, CardLabel::K, CardLabel::N6, CardLabel::N7, CardLabel::N7], bid: 28 })]
    #[case("KTJJT 220", Hand{ cards: [CardLabel::K, CardLabel::T, CardLabel::J, CardLabel::J, CardLabel::T], bid: 220 })]
    #[case("QQQJA 483", Hand{ cards: [CardLabel::Q, CardLabel::Q, CardLabel::Q, CardLabel::J, CardLabel::A], bid: 483 })]
    fn test_lines(#[case] line: &str, #[case] expected: Hand) -> miette::Result<()> {
        assert_eq!(expected, process_line(line)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(5905, process(input)?);
        Ok(())
    }
}
