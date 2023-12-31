use crate::custom_error::AocError;
use itertools::Itertools;
use ndarray::{Array2, ArrayView2};
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Space,
    Galaxy,
}

impl Element {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!("Unrecognized character!"),
        }
    }
}

#[derive(Debug)]
struct Image {
    array: Array2<Element>,
    empty_lanes: [Vec<usize>; 2],
    galaxies: Vec<[usize; 2]>,
}

impl Image {
    fn new(matrix: Array2<Element>) -> Self {
        let (empty_rows, empty_cols) = find_empty_space(matrix.view());
        let galaxies = matrix
            .view()
            .indexed_iter()
            .filter(|(_idx, x)| *x == &Element::Galaxy)
            .map(|(idx, _x)| idx.into())
            .collect_vec();

        Self {
            array: matrix,
            empty_lanes: [empty_rows, empty_cols],
            galaxies,
        }
    }

    fn distance(&self, a: &[usize; 2], b: &[usize; 2], inflation: usize) -> usize {
        a.iter()
            .zip(b)
            .zip(&self.empty_lanes)
            .map(|((&x_a, &x_b), idx_empty)| {
                let coord_range = if x_a <= x_b { x_a..x_b } else { x_b..x_a };
                let coord_distance = coord_range.len();
                let inflated_space = idx_empty.iter().filter(|i| coord_range.contains(i)).count();
                coord_distance + inflated_space * (inflation - 1)
            })
            .sum()
    }
}

fn find_empty_space(matrix: ArrayView2<Element>) -> (Vec<usize>, Vec<usize>) {
    let rows_idx = matrix
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(_idx, row)| !row.into_iter().any(|x| *x != Element::Space))
        .map(|(idx, _row)| idx)
        .collect_vec();
    let cols_idx = matrix
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(_idx, col)| !col.into_iter().any(|x| *x != Element::Space))
        .map(|(idx, _col)| idx)
        .collect_vec();
    (rows_idx, cols_idx)
}

fn parse_image(input: &str) -> IResult<&str, Image> {
    separated_list1(line_ending, many1(one_of(".#")))(input).map(|(input, char_data)| {
        debug_assert!(char_data.iter().map(|char_vec| char_vec.len()).all_equal());
        let nested_vec = char_data
            .into_iter()
            .map(|char_vec| char_vec.into_iter().map(Element::from_char).collect_vec())
            .collect_vec();
        let nrows = nested_vec.len();
        let ncols = nested_vec[0].len();
        let matrix =
            Array2::from_shape_vec((nrows, ncols), nested_vec.into_iter().flatten().collect())
                .unwrap();
        (input, Image::new(matrix))
    })
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, image) = parse_image(input).expect("Test input should parse!");
    // Iterate on every pair and calcuate the distance, then sum
    let result = image
        .galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| image.galaxies[(i + 1)..].iter().map(move |b| (a, b)))
        .map(|(a, b)| image.distance(a, b, 1000000))
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(1030, process(input)?);
        Ok(())
    }
}
