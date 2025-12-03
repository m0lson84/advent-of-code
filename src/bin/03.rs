advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    input.lines().map(|line| get_max_value(line, 2)).sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    input.lines().map(|line| get_max_value(line, 12)).sum()
}

fn get_max_value(input: &str, n: usize) -> Option<u64> {
    let length = input.len();
    let chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    let mut digits = Vec::with_capacity(n);
    while digits.len() < n {
        let needed = n - digits.len();
        let end = length - needed + 1;

        let mut max_digit = '0';
        let mut max_pos = i;
        for (j, &c) in chars.iter().enumerate().skip(i).take(end - i) {
            if c == '9' {
                max_digit = '9';
                max_pos = j;
                break;
            }

            if c > max_digit {
                max_digit = c;
                max_pos = j;
            }
        }

        digits.push(max_digit);
        i = max_pos + 1;
    }

    let result: String = digits.iter().collect();
    result.parse::<u64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
