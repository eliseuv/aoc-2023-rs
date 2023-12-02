use crate::custom_error::AocError;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

/// Set of colored cubes
#[derive(Debug, Clone, PartialEq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    //
    pub fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    // The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    tirages: Vec<CubeSet>,
}

impl Game {
    // Fewest number of cubes of each color that are necessary for the game to be possible
    pub fn fewest_number_of_cubes(&self) -> CubeSet {
        self.tirages
            // TODO: Is this clone necessary
            .clone()
            .into_iter()
            .reduce(|acc, h| acc.max(&h))
            .unwrap()
            .clone()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let result = input
        // Read input line by line
        .lines()
        // Parse game from each line
        // TODO: Error propagation
        .map(|line| parse_game(line).unwrap())
        // Get the power of the fewest number of cubes necessary
        .map(|game| game.fewest_number_of_cubes().power())
        // Sum them
        .sum();

    Ok(result)
}

#[derive(Debug)]
enum ParseGameError {
    IDNotFound,
}

fn parse_game(line: &str) -> Result<Game, ParseGameError> {
    static RE_GAME: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^Game (\d+): ").expect("This regex construction should never fail!")
    });

    // Parse Game id
    let cap = RE_GAME.captures(line).ok_or(ParseGameError::IDNotFound)?;

    let id = cap[1]
        .parse::<u32>()
        .expect("Once a capture is found, parsing to integer should never fail!");

    // String index where the game can started to be parsed
    let first_index = cap
        .get(0)
        .expect("The capture has already been tested to contain a match!")
        .end();

    // Lazily construct regexes
    static RE_CUBESET_DELIM: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"; ").expect("This regex construction should never fail!"));

    // Parse cube sets
    let tirages: Vec<CubeSet> = [first_index]
        .into_iter()
        .chain(
            RE_CUBESET_DELIM
                .find_iter(&line[first_index..])
                .map(|m| first_index + m.end()),
        )
        .chain([line.len()])
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| &line[a..b])
        // TODO: Return `Result` from cube set parser and propagate error to game parser
        .map(parse_cube_set)
        .collect();

    Ok(Game { id, tirages })
}

fn parse_cube_set(cube_set_str: &str) -> CubeSet {
    // Lazily construct regexes
    static RE_COLORS: [Lazy<Regex>; 3] = [
        Lazy::new(|| Regex::new(r"(\d+) red").expect("This regex construction should never fail!")),
        Lazy::new(|| {
            Regex::new(r"(\d+) green").expect("This regex construction should never fail!")
        }),
        Lazy::new(|| {
            Regex::new(r"(\d+) blue").expect("This regex construction should never fail!")
        }),
    ];

    let counts: Vec<u32> = RE_COLORS
        .iter()
        .map(|re| {
            if let Some(cap) = re.captures(cube_set_str) {
                cap[1]
                    .parse::<u32>()
                    .expect("Once a capture is found, parsing to integer should never fail!")
            } else {
                0
            }
        })
        .collect();

    CubeSet {
        red: counts[0],
        green: counts[1],
        blue: counts[2],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", CubeSet{red: 4, green: 2, blue: 6})]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", CubeSet { red: 1, green: 3, blue: 4 })]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", CubeSet { red: 20, green: 13, blue: 6 })]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", CubeSet { red: 14, green: 3, blue: 15 })]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", CubeSet { red: 6, green: 3, blue: 2 })]
    fn test_parse_line(#[case] line: &str, #[case] expected: CubeSet) {
        assert_eq!(
            expected,
            parse_game(line)
                .expect("Testing data should work!")
                .fewest_number_of_cubes(),
        )
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
