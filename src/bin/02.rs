use std::{cmp::max, u32};

advent_of_code::solution!(2);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

/// Part one of today's puzzle.
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let game_id = get_game_id(line);
        let is_valid = validate(line);
        if is_valid {
            sum += game_id;
        }
    }
    Some(sum)
}

/// Part two of today's puzzle.
pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        sum += calculate_power(line);
    }
    Some(sum)
}

/// Calculate the power of a game.
fn calculate_power(input: &str) -> u32 {
    let sets = get_sets(input);
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for set in sets {
        for cube in set.split(',') {
            let (color, count) = get_cube(cube);
            match color {
                "red" => red = max(red, count),
                "green" => green = max(green, count),
                "blue" => blue = max(blue, count),
                _ => panic!("Unknown color"),
            }
        }
    }
    red * green * blue
}

/// Get the color and count of a cube.
fn get_cube(input: &str) -> (&str, u32) {
    let mut iter = input.split_whitespace();
    let count = iter.next().unwrap().parse::<u32>().unwrap();
    let color = iter.next().unwrap();
    (color, count)
}

/// Get the identifier for a given game.
fn get_game_id(input: &str) -> u32 {
    let game_id = input.split(':').next().unwrap();
    game_id
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

/// Get the sets for a given game.
fn get_sets(input: &str) -> Vec<&str> {
    let sets = input.split(':').nth(1).unwrap();
    sets.split(';').collect()
}

/// Check if a given game is valid.
fn validate(input: &str) -> bool {
    let sets = get_sets(input);
    for set in sets {
        for cube in set.split(',') {
            let (color, count) = get_cube(cube);
            match color {
                "red" => {
                    if count > MAX_RED {
                        return false;
                    }
                }
                "green" => {
                    if count > MAX_GREEN {
                        return false;
                    }
                }
                "blue" => {
                    if count > MAX_BLUE {
                        return false;
                    }
                }
                _ => panic!("Unknown color"),
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
