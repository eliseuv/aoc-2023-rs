use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let output: u32 = input.lines().map(process_line).sum();
    Ok(output)
}

fn process_line(line: &str) -> u32 {
    let (first, last) = get_fist_and_last_digits(line);

    (first * 10) + last
}

const NUM_NAMES: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_fist_and_last_digits(line: &str) -> (u32, u32) {
    // Iterator over the digits found in the line
    let mut num_iter = line.char_indices().filter_map(|(index, char)| {
        if let Some((_, num)) = NUM_NAMES
            .iter()
            .find(|(name, _)| line[index..].starts_with(name))
        {
            Some(*num)
        } else {
            char.to_digit(10)
        }
    });

    // Retrieve the first digit in the line
    // It is guaranteed to have at least one digit per line
    let first = num_iter
        .next()
        .expect("There should be at least one number in each line!");

    // Concatenate the first and last digit and parse it as a number
    // If the line contains only one digit, then use it twice
    let last = match num_iter.last() {
        Some(num) => num,
        None => first,
    };

    (first, last)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
