use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, one_of},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

#[derive(Debug)]
#[repr(u8)]
enum Instruction {
    Left,
    Right,
}

type Node<'a> = &'a str;
type Network<'a> = HashMap<Node<'a>, [Node<'a>; 2]>;

#[derive(Debug)]
struct Map<'a> {
    instructions: Vec<Instruction>,
    network: Network<'a>,
}

#[derive(Debug)]
struct MapNavigator<'a> {
    map: &'a Map<'a>,
    counter: usize,
    node: &'a str,
}

impl<'a> MapNavigator<'a> {
    fn new(map: &'a Map<'a>, node: Node<'a>) -> Self {
        Self {
            map,
            counter: 0,
            node,
        }
    }
}

impl<'a> Iterator for MapNavigator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.ends_with("Z") {
            None
        } else {
            // Go Left/Right
            let step = &self.map.instructions[self.counter];
            // Update current node
            let idx = match step {
                Instruction::Left => 0,
                Instruction::Right => 1,
            };
            self.node = self.map.network[self.node][idx];
            // Update counter and wrap if necessary
            self.counter += 1;
            if self.counter == self.map.instructions.len() {
                self.counter = 0;
            }
            // Return new state
            Some(self.node)
        }
    }
}

// TODO: Find a better algortihm for GCD and LCM
fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(nums: &[usize]) -> usize {
    match nums {
        [n] => *n,
        [n, ns @ ..] => {
            let m = lcm(ns);
            (n * m) / gcd(*n, m)
        }
        [] => panic!("Least Common Multiplier of empty vector!"),
    }
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(one_of("LR"))(input).map(|(input, inst_chars)| {
        (
            input,
            inst_chars
                .into_iter()
                .map(|c| match c {
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => panic!(),
                })
                .collect::<Vec<_>>(),
        )
    })
}

fn parse_node_info(input: &str) -> IResult<&str, (Node, (Node, Node))> {
    tuple((
        alphanumeric1,
        delimited(
            tag(" = ("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    ))(input)
    // .map(|(input, (node, (node_left, node_right)))| {
    //     (
    //         input,
    //         (
    //             node.as_bytes(),
    //             (node_left.as_bytes(), node_right.as_bytes()),
    //         ),
    //     )
    // })
}

fn parse_network(input: &str) -> IResult<&str, Network> {
    separated_list1(line_ending, parse_node_info)
        .preceded_by(many0(line_ending))
        .parse(input)
        .map(|(input, nodes)| {
            (
                input,
                nodes.into_iter().fold(
                    HashMap::new(),
                    |mut acc, (node, (node_left, node_rigth))| {
                        acc.insert(node, [node_left, node_rigth]);
                        acc
                    },
                ),
            )
        })
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    separated_pair(parse_instructions, many1(line_ending), parse_network)(input).map(
        |(input, (instructions, network))| {
            (
                input,
                Map {
                    instructions,
                    network,
                },
            )
        },
    )
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, map) = parse_map(input).expect("Sample input should parse!");

    let starting_nodes: Vec<_> = map
        .network
        .keys()
        .filter(|node| node.ends_with("A"))
        .copied()
        .collect();

    let steps: Vec<_> = starting_nodes
        .par_iter()
        .map(|node| MapNavigator::new(&map, node).count())
        .collect();

    Ok(lcm(&steps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
