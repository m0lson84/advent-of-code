advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);

    let result = (0..grid.len())
        .flat_map(|row| (0..grid[row].len()).map(move |col| (row, col)))
        .filter(|&(row, col)| grid[row][col] == '@')
        .filter(|&(row, col)| count_neighbors(&grid, row, col) < 4)
        .count();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_grid(input);
    let mut result = 0;

    loop {
        let to_remove: Vec<_> = (0..grid.len())
            .flat_map(|row| (0..grid[row].len()).map(move |col| (row, col)))
            .filter(|&(row, col)| grid[row][col] == '@')
            .filter(|&(row, col)| count_neighbors(&grid, row, col) < 4)
            .collect();

        if to_remove.is_empty() {
            break;
        }

        for (row, col) in &to_remove {
            grid[*row][*col] = '.';
        }

        result += to_remove.len() as u64;
    }

    Some(result)
}

fn count_neighbors(grid: &[Vec<char>], row: usize, col: usize) -> u32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in directions {
        let current_row = row as i32 + dr;
        let current_col = col as i32 + dc;

        if current_row >= 0
            && current_row < rows
            && current_col >= 0
            && current_col < cols
            && grid[current_row as usize][current_col as usize] == '@'
        {
            count += 1;
        }
    }

    count
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
