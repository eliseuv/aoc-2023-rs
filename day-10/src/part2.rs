use crate::custom_error::AocError;
use ndarray::Array2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn possible_next_tiles(&self) -> [Tile; 3] {
        match self {
            Direction::East => [Tile::Horizontal, Tile::TurnNW, Tile::TurnSW],
            Direction::North => [Tile::Vertical, Tile::TurnSW, Tile::TurnSE],
            Direction::South => [Tile::Vertical, Tile::TurnNE, Tile::TurnNW],
            Direction::West => [Tile::Horizontal, Tile::TurnNE, Tile::TurnSE],
        }
    }
}

fn get_nearest_neighbors<T>(
    lattice: &Array2<T>,
    &[y, x]: &[usize; 2],
) -> [(Direction, Option<T>); 4]
where
    T: Clone + Copy + Debug,
{
    let maybe_idx_north = y.checked_sub(1).map(|y_north| [y_north, x]);
    let maybe_idx_west = x.checked_sub(1).map(|x_west| [y, x_west]);
    let idx_south = [y + 1, x];
    let idx_east = [y, x + 1];

    [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .iter()
    .copied()
    .zip(
        [maybe_idx_north, maybe_idx_west]
            .iter()
            .map(|maybe_idx| maybe_idx.map(|idx| lattice[idx]))
            .chain(
                [idx_south, idx_east]
                    .iter()
                    .map(|idx| lattice.get(*idx).copied()),
            ),
    )
    .collect::<Vec<_>>()
    .try_into()
    .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,      // S
    Ground,     // .
    Vertical,   // |
    Horizontal, // -
    TurnNE,     // L
    TurnNW,     // J
    TurnSW,     // 7
    TurnSE,     // F
}

impl Tile {
    fn from_char(c: &char) -> Self {
        match c {
            'S' => Self::Start,
            '.' => Self::Ground,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::TurnNE,
            'J' => Self::TurnNW,
            '7' => Self::TurnSW,
            'F' => Self::TurnSE,
            _ => panic!("Unrecognized tile character!"),
        }
    }

    // Given that we have arrived at this tile with `current_direction`, in what direction will we turn to
    // `Start` position is special for it always preserve direction
    fn next_direction(&self, current_direction: &Direction) -> Direction {
        match current_direction {
            Direction::North => match self {
                Self::Start => Direction::North,
                Self::Vertical => Direction::North,
                Self::TurnSW => Direction::West,
                Self::TurnSE => Direction::East,
                _ => panic!("Impossible current direction!"),
            },
            Direction::South => match self {
                Self::Start => Direction::South,
                Self::Vertical => Direction::South,
                Self::TurnNW => Direction::West,
                Self::TurnNE => Direction::East,
                _ => panic!("Impossible current direction!"),
            },
            Direction::East => match self {
                Self::Start => Direction::East,
                Self::Horizontal => Direction::East,
                Self::TurnNW => Direction::North,
                Self::TurnSW => Direction::South,
                _ => panic!("Impossible current direction!"),
            },
            Direction::West => match self {
                Self::Start => Direction::West,
                Self::Horizontal => Direction::West,
                Self::TurnNE => Direction::North,
                Self::TurnSE => Direction::South,
                _ => panic!("Impossible current direction!"),
            },
        }
    }
}

#[derive(Debug)]
struct Maze {
    // 2D array of tiles
    tiles: Array2<Tile>,
    // Coordinates of the starting tile
    start_idx: [usize; 2],
    // The two possible directions we can start walking
    start_directions: [Direction; 2],
}

impl Maze {
    fn from_parsed_input(arr: Vec<Vec<char>>) -> Self {
        let nrows = arr.len();
        let ncols = arr[0].len();
        let data = arr.iter().flatten().map(Tile::from_char).collect();

        let tiles = Array2::from_shape_vec([nrows, ncols], data).unwrap();

        let start_idx: [usize; 2] = tiles
            .indexed_iter()
            .find(|(_, &tile)| tile == Tile::Start)
            .map(|(idx, _)| idx)
            .expect("There should be an starting tile!")
            .into();

        let start_directions: [Direction; 2] = get_nearest_neighbors(&tiles, &start_idx)
            .iter()
            .filter(|(direction, maybe_tile)| match maybe_tile {
                Some(tile) => direction.possible_next_tiles().contains(tile),
                None => false,
            })
            .map(|(direction, _)| *direction)
            .collect::<Vec<_>>()
            .try_into()
            .expect("There should be exactly two possible directions to start!");

        Self {
            tiles,
            start_idx,
            start_directions,
        }
    }
}

// The 2 possible opposite directions to traverse a loop
#[derive(Debug, Clone, Copy)]
enum LoopDirection {
    A,
    B,
}

#[derive(Debug)]
struct Walker<'a> {
    maze: &'a Maze,
    idx: [usize; 2],
    direction: Direction,
}

impl<'a> Walker<'a> {
    fn on_maze(maze: &'a Maze, loop_direction: LoopDirection) -> Self {
        let direction = match loop_direction {
            LoopDirection::A => maze.start_directions[0],
            LoopDirection::B => maze.start_directions[1],
        };
        Self {
            maze,
            idx: maze.start_idx,
            direction,
        }
    }

    fn current_tile(&self) -> &Tile {
        &self.maze.tiles[self.idx]
    }

    fn step(&mut self, direction: &Direction) {
        let [y, x] = self.idx;
        self.idx = match direction {
            Direction::North => [y - 1, x],
            Direction::West => [y, x - 1],
            Direction::South => [y + 1, x],
            Direction::East => [y, x + 1],
        }
    }
}

impl Iterator for Walker<'_> {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        // Decide next direction based on current tile
        let next_direction = self.current_tile().next_direction(&self.direction);
        // Turn
        self.direction = next_direction;
        // Move
        self.step(&next_direction);
        // The iteration finishes when the loop is completed
        match self.current_tile() {
            Tile::Start => None,
            _ => Some(self.idx.to_owned()),
        }
    }
}

fn parse_maze(input: &str) -> IResult<&str, Maze> {
    separated_list1(line_ending, many1(one_of("S.|-LJ7F")))(input)
        .map(|(input, arr)| (input, Maze::from_parsed_input(arr)))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (_, maze) = parse_maze(input).unwrap();

    // dbg!(Walker::new(&maze, LoopDirection::A)
    //     .enumerate()
    //     .collect_vec());

    let n_steps = Walker::on_maze(&maze, LoopDirection::A)
        .zip(Walker::on_maze(&maze, LoopDirection::B))
        .position(|(position_a, position_b)| position_a == position_b)
        .expect("The walkers must meet somewhere!");

    // We must add one because the original problem takes into account the starting postion
    Ok(n_steps + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    #[case(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        8
    )]
    fn test_cases(#[case] input: &str, #[case] expected: usize) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
