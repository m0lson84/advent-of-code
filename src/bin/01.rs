advent_of_code::solution!(1);

/// Part one of today's puzzle.
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let chars = get_numeric_chars(line);
        if chars.is_empty() {
            continue;
        }
        let calibration = chars[0].to_string() + &chars[chars.len() - 1].to_string();
        sum += calibration.parse::<u32>().unwrap();
    }
    Some(sum)
}

/// Part two of today's puzzle.
pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let replaced = replace_numeric_words(line);
        let chars = get_numeric_chars(replaced.as_str());
        let calibration = chars[0].to_string() + &chars[chars.len() - 1].to_string();
        sum += calibration.parse::<u32>().unwrap();
    }
    Some(sum)
}

/// Get all numeric characters from the input string.
fn get_numeric_chars(input: &str) -> Vec<char> {
    input.chars().filter(|c| c.is_numeric()).collect()
}

/// Replace number words with their numeric representation.
fn replace_numeric_words(input: &str) -> String {
    let mut replaced = input.to_owned();
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, digit) in digits.iter().enumerate() {
        replaced = replaced.replace(digit, format!("{}{}{}", digit, i, digit).as_str());
    }
    replaced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
