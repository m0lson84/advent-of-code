use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let start = find_start(&grid)?;
    let result = simulate_beams(&grid, start);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let start = find_start(&grid)?;

    let mut memo = HashMap::new();
    let result = count_paths(&grid, start, &mut memo);

    Some(result)
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                return Some((row, col));
            }
        }
    }
    None
}

fn simulate_beams(grid: &[Vec<char>], start: (usize, usize)) -> u64 {
    let mut count = 0;
    let mut active = VecDeque::new();
    let mut visited = HashSet::new();

    active.push_back((start.0, start.1));

    while let Some(beam_pos) = active.pop_front() {
        if let Some(splitter_pos) = find_next(grid, beam_pos)
            && visited.insert(splitter_pos)
        {
            count += 1;
            add_split_beams(grid, splitter_pos, &mut active);
        }
    }

    count
}

fn find_next(grid: &[Vec<char>], start: (usize, usize)) -> Option<(usize, usize)> {
    let (mut row, col) = start;

    loop {
        row += 1;

        if row >= grid.len() || col >= grid[row].len() {
            return None;
        }

        if grid[row][col] == '^' {
            return Some((row, col));
        }
    }
}

fn add_split_beams(
    grid: &[Vec<char>],
    splitter: (usize, usize),
    beams: &mut VecDeque<(usize, usize)>,
) {
    let (row, col) = splitter;

    if col > 0 {
        beams.push_back((row, col - 1));
    }

    if col < grid[row].len() - 1 {
        beams.push_back((row, col + 1));
    }
}

fn count_paths(
    grid: &[Vec<char>],
    pos: (usize, usize),
    memo: &mut std::collections::HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&cached) = memo.get(&pos) {
        return cached;
    }

    let result = if let Some(splitter) = find_next(grid, pos) {
        let (row, col) = splitter;
        let mut total = 0;

        // Count paths going left
        if col > 0 {
            total += count_paths(grid, (row, col - 1), memo);
        }

        // Count paths going right
        if col < grid[row].len() - 1 {
            total += count_paths(grid, (row, col + 1), memo);
        }

        total
    } else {
        // Exited the grid - this counts as 1 timeline
        1
    };

    memo.insert(pos, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
