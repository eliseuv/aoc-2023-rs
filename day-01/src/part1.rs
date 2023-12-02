use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let output = input.lines().map(process_line).sum::<u32>();

    Ok(output)
}

fn process_line(line: &str) -> u32 {
    let (first, last) = get_fist_and_last_digits(line);

    (first * 10) + last
}

fn get_fist_and_last_digits(line: &str) -> (u32, u32) {
    // Transform line in an iterator over the decimal digits it contains
    let mut num_iter = line.chars().filter_map(|character| character.to_digit(10));

    // Retrieve the first digit in the line
    // It is guaranteed to have at least one digit per line
    let first = num_iter
        .next()
        .expect("There should be at least one number in each line!");

    // Get the last digit in the line
    let last = match num_iter.last() {
        // If the line contains only one digit, then use it twice
        Some(num) => num,
        None => first,
    };

    (first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
