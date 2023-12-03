use crate::custom_error::AocError;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

/// Set of colored cubes
#[derive(Debug, PartialEq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    tirages: Vec<CubeSet>,
}

// /^Game (\d+):(( (\d+) (red|green|blue),?)+;?)+/mg

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let result = input
        // Read input line by line
        .lines()
        // Parse game from each line
        // TODO: Error propagation
        .map(|line| parse_game(line).unwrap())
        // Filter the games that could have been possible
        // if the Elf had 12 red, 13 green and 14 blue cubes
        .filter(|game| {
            game.tirages
                .iter()
                .all(|h| h.red <= 12 && h.green <= 13 && h.blue <= 14)
        })
        // Get their ids
        .map(|game| game.id)
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
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game{id: 1, tirages: vec![CubeSet{blue: 3, red: 4, green: 0}, CubeSet{red: 1, green: 2, blue: 6}, CubeSet{green: 2, red: 0, blue: 0}]})]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Game{id: 2, tirages: vec![CubeSet{blue: 1, green: 2, red: 0}, CubeSet{green: 3, blue: 4, red: 1}, CubeSet{green:1 , blue:1, red:0}]
    })]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Game{id: 3, tirages: vec![CubeSet{green: 8, blue: 6, red: 20}, CubeSet{blue: 5, red: 4, green: 13}, CubeSet{green: 5, red: 1, blue:0}]})]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Game{id: 4, tirages: vec![CubeSet{green: 1, red: 3, blue: 6}, CubeSet{green: 3, red: 6, blue: 0}, CubeSet{green: 3, blue: 15, red: 14}]})]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Game{id: 5, tirages: vec![CubeSet{red: 6, blue: 1, green: 3}, CubeSet{blue: 2, red: 1, green: 2}]})]
    fn test_parse_line(#[case] line: &str, #[case] expected: Game) {
        assert_eq!(
            parse_game(line).expect("Testing data should work!"),
            expected
        )
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
