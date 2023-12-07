use crate::custom_error::AocError;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Range;

#[derive(Debug)]
struct CharGrid<'a> {
    ncols: usize,
    lines: Vec<&'a str>,
}

#[derive(Debug)]
struct GridPosition {
    line_num: usize,
    range: Range<usize>,
}

impl GridPosition {
    pub fn is_adjacent_to(&self, other: &Self) -> bool {
        match (self.line_num as isize) - (other.line_num as isize) {
            // Same line
            -1..=1 => other.range.start <= self.range.end && self.range.start <= other.range.end,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct NumberWithPosition {
    number: u32,
    position: GridPosition,
}

impl<'a> CharGrid<'a> {
    pub fn new(text: &'a str) -> Self {
        let lines: Vec<&'a str> = text.lines().collect();
        assert!(lines.iter().map(|line| line.len()).all_equal());

        Self {
            ncols: lines[0].len(),
            lines,
        }
    }

    pub fn find_all_numbers(&self) -> Vec<NumberWithPosition> {
        static RE_NUM: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"\d+").expect("This regex creation should never fail!"));

        self.lines
            .iter()
            .enumerate()
            .flat_map(|(i, &line)| {
                RE_NUM.find_iter(line).map(move |m| NumberWithPosition {
                    number: m
                        .as_str()
                        .parse()
                        .expect("Parsing this integer should never fail!"),
                    position: GridPosition {
                        line_num: i,
                        range: m.range(),
                    },
                })
            })
            .collect()
    }

    pub fn find_part_numbers(&self) -> Vec<u32> {
        self.find_all_numbers()
            .into_iter()
            .filter(|num_pos| {
                let own_line = self.lines[num_pos.position.line_num];
                // Neighboring chars in same line
                let prev_idx = match num_pos.position.range.start {
                    0 => None,
                    _ => Some(num_pos.position.range.start - 1),
                };
                let next_idx = if num_pos.position.range.end == self.ncols {
                    None
                } else {
                    Some(num_pos.position.range.end)
                };

                // Neighboring lines
                let prev_line = match num_pos.position.line_num {
                    0 => None,
                    _ => Some(self.lines[num_pos.position.line_num - 1]),
                };
                let next_line = if num_pos.position.line_num == self.lines.len() - 1 {
                    None
                } else {
                    Some(self.lines[num_pos.position.line_num + 1])
                };

                // Symbol regex
                // TODO: Scan grid for symbols
                static RE_SYMB: Lazy<Regex> = Lazy::new(|| {
                    Regex::new(r"(?:\+|\*|\#|\$|\%|\&|\/|\-|\=|\@)")
                        .expect("This regex creation should never fail!")
                });

                [prev_idx, next_idx]
                    .into_iter()
                    .flatten()
                    .any(|idx| RE_SYMB.is_match(&own_line[idx..=idx]))
                    || [prev_line, next_line].into_iter().flatten().any(|line| {
                        let prev_char_idx = if let Some(idx) = prev_idx { idx } else { 0 };
                        let next_char_idx = if let Some(idx) = next_idx {
                            idx
                        } else {
                            self.ncols - 1
                        };
                        RE_SYMB.is_match(&line[prev_char_idx..=next_char_idx])
                    })
            })
            .map(|np| np.number)
            .collect()
    }

    pub fn find_all_gears(&self) -> Vec<GridPosition> {
        static RE_GEAR: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"\*").expect("This regex creation should never fail!"));

        self.lines
            .iter()
            .enumerate()
            .flat_map(|(line_num, &line)| {
                RE_GEAR.find_iter(line).map(move |m| GridPosition {
                    line_num,
                    range: m.range(),
                })
            })
            .collect()
    }

    pub fn get_adjacent_numbers(&self, gear_pos: &GridPosition) -> Vec<u32> {
        self.find_all_numbers()
            .into_iter()
            .filter(|num_pos| num_pos.position.is_adjacent_to(gear_pos))
            .map(|num_pos| num_pos.number)
            .collect()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let grid = CharGrid::new(input);
    // Find part numbers
    let result = grid
        .find_all_gears()
        .into_iter()
        // Get the numbers adjacents to each grid
        .map(|gear_pos| grid.get_adjacent_numbers(&gear_pos))
        // Consider only gears adjacent to exactly 2 numbers
        .filter(|numbers_list| numbers_list.len() == 2)
        // Calculate gear ratios by multiplying these numbers
        .map(|numbers_list| numbers_list.iter().product::<u32>())
        // Sum all gear ratios
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}
