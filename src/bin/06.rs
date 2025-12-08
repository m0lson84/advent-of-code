advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_problems(input)?;
    let total: u64 = problems.iter().map(solve_problem).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse_problems_rtl(input)?;
    let total: u64 = problems.iter().map(solve_problem).sum();
    Some(total)
}

fn parse_problems(input: &str) -> Option<Vec<Problem>> {
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();
    if lines.is_empty() {
        return None;
    }

    let operator_line = lines.last()?;
    let number_lines = &lines[..lines.len() - 1];

    let max_width = lines.iter().map(|l| l.len()).max()?;
    let separators = find_separators(&lines, max_width);
    let ranges = get_ranges(&separators, max_width);

    ranges
        .iter()
        .map(|&(start, end)| parse_problem(number_lines, operator_line, start, end))
        .collect()
}

fn parse_problems_rtl(input: &str) -> Option<Vec<Problem>> {
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();
    if lines.is_empty() {
        return None;
    }

    let operator_line = lines.last()?;
    let number_lines = &lines[..lines.len() - 1];

    let max_width = lines.iter().map(|l| l.len()).max()?;
    let separators = find_separators(&lines, max_width);
    let ranges = get_ranges(&separators, max_width);

    ranges
        .iter()
        .map(|&(start, end)| parse_problem_rtl(number_lines, operator_line, start, end))
        .collect()
}

fn find_separators(lines: &[&str], max_width: usize) -> Vec<usize> {
    (0..max_width)
        .filter(|&col| lines.iter().all(|line| is_space(line, col)))
        .collect()
}

fn is_space(line: &str, col: usize) -> bool {
    col >= line.len() || line.as_bytes().get(col) == Some(&b' ')
}

fn get_ranges(separators: &[usize], max_width: usize) -> Vec<(usize, usize)> {
    if separators.is_empty() {
        return vec![(0, max_width)];
    }

    let mut ranges = Vec::new();
    let mut start = 0;

    for &sep in separators {
        if sep > start {
            ranges.push((start, sep));
        }
        start = sep + 1;
    }

    if start < max_width {
        ranges.push((start, max_width));
    }

    ranges
}

fn parse_problem(
    number_lines: &[&str],
    operator_line: &str,
    start: usize,
    end: usize,
) -> Option<Problem> {
    let operator = get_operator(operator_line, start, end)?;
    let numbers = get_numbers(number_lines, start, end);

    if numbers.is_empty() {
        return None;
    }

    Some(Problem { numbers, operator })
}

fn parse_problem_rtl(
    number_lines: &[&str],
    operator_line: &str,
    start: usize,
    end: usize,
) -> Option<Problem> {
    let operator = get_operator(operator_line, start, end)?;

    let numbers: Vec<u64> = (start..end)
        .rev()
        .filter_map(|col| read_column(number_lines, col))
        .collect();

    if numbers.is_empty() {
        return None;
    }

    Some(Problem { numbers, operator })
}

fn read_column(lines: &[&str], col: usize) -> Option<u64> {
    let digits: String = lines
        .iter()
        .filter_map(|line| {
            if col < line.len() {
                let ch = line.as_bytes()[col] as char;
                if ch.is_ascii_digit() { Some(ch) } else { None }
            } else {
                None
            }
        })
        .collect();

    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

fn get_operator(line: &str, start: usize, end: usize) -> Option<char> {
    let end = end.min(line.len());
    if start >= end {
        return None;
    }

    line[start..end].chars().find(|&ch| ch == '+' || ch == '*')
}

fn get_numbers(lines: &[&str], start: usize, end: usize) -> Vec<u64> {
    lines
        .iter()
        .flat_map(|line| parse_line(line, start, end))
        .collect()
}

fn parse_line(line: &str, start: usize, end: usize) -> Vec<u64> {
    if start >= line.len() {
        return Vec::new();
    }

    let end = end.min(line.len());
    line[start..end]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn solve_problem(problem: &Problem) -> u64 {
    match problem.operator {
        '+' => problem.numbers.iter().sum(),
        '*' => problem.numbers.iter().product(),
        _ => 0,
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
