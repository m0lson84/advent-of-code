advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut position: i64 = 50;
    let mut result: u64 = 0;
    for turn in input.lines() {
        let (direction, distance) = parse_rotation(turn)?;

        position = apply_rotation(position, direction, distance);

        if position == 0 {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position: i64 = 50;
    let mut result: u64 = 0;

    for turn in input.lines() {
        let (direction, distance) = parse_rotation(turn)?;

        // Count complete loops (each loop crosses 0 once)
        let complete_loops = distance / 100;
        result += complete_loops as u64;

        // Check if the partial rotation crosses 0
        let remaining = distance % 100;
        let crosses_zero = match direction {
            'R' => position + remaining >= 100,
            'L' => position - remaining <= 0 && position > 0,
            _ => false,
        };

        if crosses_zero {
            result += 1;
        }

        // Update position for next iteration
        position = apply_rotation(position, direction, distance);
    }

    Some(result)
}

fn apply_rotation(position: i64, direction: char, distance: i64) -> i64 {
    match direction {
        'R' => (position + distance).rem_euclid(100),
        'L' => (position - distance).rem_euclid(100),
        _ => position,
    }
}

fn parse_rotation(line: &str) -> Option<(char, i64)> {
    let direction = line.chars().next()?;
    let distance: i64 = line[1..].parse().ok()?;
    Some((direction, distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
