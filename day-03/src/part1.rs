use crate::custom_error::AocError;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Range;

#[derive(Debug)]
struct NumberPosition {
    number: u32,
    line_num: usize,
    position: Range<usize>,
}

#[derive(Debug)]
struct CharGrid<'a> {
    ncols: usize,
    lines: Vec<&'a str>,
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

    pub fn find_all_numbers(&self) -> Vec<NumberPosition> {
        static RE_NUM: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"\d+").expect("This regex creation should never fail!"));

        self.lines
            .iter()
            .enumerate()
            .flat_map(|(i, &line)| {
                RE_NUM.find_iter(line).map(move |m| NumberPosition {
                    number: m
                        .as_str()
                        .parse()
                        .expect("Parsing this integer should never fail!"),
                    line_num: i,
                    position: m.range(),
                })
            })
            .collect()
    }

    pub fn find_part_numbers(&self) -> Vec<u32> {
        self.find_all_numbers()
            .into_iter()
            .filter(|np| {
                let own_line = self.lines[np.line_num];
                // Neighboring chars in same line
                let prev_idx = match np.position.start {
                    0 => None,
                    _ => Some(np.position.start - 1),
                };
                let next_idx = if np.position.end == self.ncols {
                    None
                } else {
                    Some(np.position.end)
                };

                // Neighboring lines
                let prev_line = match np.line_num {
                    0 => None,
                    _ => Some(self.lines[np.line_num - 1]),
                };
                let next_line = if np.line_num == self.lines.len() - 1 {
                    None
                } else {
                    Some(self.lines[np.line_num + 1])
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
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let grid = CharGrid::new(input);
    // Find part numbers
    let part_numbers = grid.find_part_numbers();

    Ok(part_numbers.iter().sum())
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
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
