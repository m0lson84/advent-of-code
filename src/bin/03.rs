advent_of_code::solution!(3);
use std::collections::HashMap;

use array2d::Array2D;

/// Part one of today's puzzle.
pub fn part_one(input: &str) -> Option<u32> {
    let chars = get_char_array(input);
    let mut result = 0;
    for (row, iter) in chars.rows_iter().enumerate() {
        let mut checked = false;
        for (column, element) in iter.enumerate() {
            let current: char = *element;
            if current.is_ascii_digit()
                && !checked
                && !get_neighbors(&chars, row, column).is_empty()
            {
                result += get_number(&chars, &row, &column);
                checked = true;
                continue;
            }
            if !current.is_ascii_digit() {
                checked = false;
                continue;
            }
        }
    }

    Some(result)
}

/// Part two of today's puzzle.
pub fn part_two(input: &str) -> Option<u32> {
    let chars = get_char_array(input);

    let mut stars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (row, iter) in chars.rows_iter().enumerate() {
        let mut checked = false;
        for (column, element) in iter.enumerate() {
            let current: char = *element;
            if current.is_ascii_digit() && !checked {
                let star = get_neighboring_stars(&chars, row, column);
                if !star.is_empty() {
                    let (selected_row, selected_column, _) = star.first().unwrap().clone();
                    let key = (selected_row, selected_column);
                    let value = get_number(&chars, &row, &column);
                    if stars.contains_key(&key) {
                        let mut vector: Vec<u32> =
                            stars.get(&(selected_row, selected_column)).unwrap().clone();
                        vector.push(value);
                        stars.insert(key, vector);
                    } else {
                        stars.insert(key, vec![value]);
                    }
                    checked = true;
                }
            }
            if !get_neighbors(&chars, row, column).is_empty() {
                checked = true;
            }
            if !current.is_ascii_digit() {
                checked = false;
                continue;
            }
        }
    }

    let result = stars.iter().fold(0u32, |acc, (_, gears)| {
        if gears.len() == 2 {
            acc + gears.first().unwrap() * gears.last().unwrap()
        } else {
            acc
        }
    });

    Some(result)
}

/// Convert the input string into a 2D array of characters.
fn get_char_array(input: &str) -> Array2D<char> {
    let rows: Vec<&str> = input.lines().collect();

    let mut array = Vec::new();
    for row in rows {
        array.push(row.chars().collect());
    }

    Array2D::from_rows(&array).unwrap()
}

/// Get the neighbors of a given cell.
fn get_neighbors(chars: &Array2D<char>, row: usize, column: usize) -> Vec<&char> {
    let mut candidates = Vec::new();
    candidates.push(chars.get(row, column.checked_add(1).unwrap()));
    candidates.push(chars.get(row.checked_add(1).unwrap(), column));
    candidates.push(chars.get(row.checked_add(1).unwrap(), column.checked_add(1).unwrap()));

    if row > 0 {
        candidates.push(chars.get(row.checked_sub(1).unwrap(), column));
        candidates.push(chars.get(row.checked_sub(1).unwrap(), column.checked_add(1).unwrap()));
    }

    if column > 0 {
        candidates.push(chars.get(row, column.checked_sub(1).unwrap()));
        candidates.push(chars.get(row.checked_add(1).unwrap(), column.checked_sub(1).unwrap()));
    }

    if row > 0 && column > 0 {
        candidates.push(chars.get(row.checked_sub(1).unwrap(), column.checked_sub(1).unwrap()));
    }
    candidates
        .into_iter()
        .flatten()
        .filter(|c| !c.is_ascii_digit() && **c != '.')
        .collect()
}

fn get_neighboring_stars(
    chars: &Array2D<char>,
    row: usize,
    column: usize,
) -> Vec<(usize, usize, Option<&char>)> {
    let mut candidates: Vec<(usize, usize, Option<&char>)> = Vec::new();
    candidates.push((
        column.checked_add(1).unwrap(),
        row,
        chars.get(row, column.checked_add(1).unwrap()),
    ));
    candidates.push((
        column,
        row.checked_add(1).unwrap(),
        chars.get(row.checked_add(1).unwrap(), column),
    ));
    candidates.push((
        column.checked_add(1).unwrap(),
        row.checked_add(1).unwrap(),
        chars.get(row.checked_add(1).unwrap(), column.checked_add(1).unwrap()),
    ));

    if row > 0 {
        candidates.push((
            column,
            row.checked_sub(1).unwrap(),
            chars.get(row.checked_sub(1).unwrap(), column),
        ));
        candidates.push((
            column.checked_add(1).unwrap(),
            row.checked_sub(1).unwrap(),
            chars.get(row.checked_sub(1).unwrap(), column.checked_add(1).unwrap()),
        ));
    }

    if column > 0 {
        candidates.push((
            column.checked_sub(1).unwrap(),
            row,
            chars.get(row, column.checked_sub(1).unwrap()),
        ));
        candidates.push((
            column.checked_sub(1).unwrap(),
            row.checked_add(1).unwrap(),
            chars.get(row.checked_add(1).unwrap(), column.checked_sub(1).unwrap()),
        ));
    }

    if row > 0 && column > 0 {
        candidates.push((
            column.checked_sub(1).unwrap(),
            row.checked_sub(1).unwrap(),
            chars.get(row.checked_sub(1).unwrap(), column.checked_sub(1).unwrap()),
        ));
    }

    candidates
        .into_iter()
        .filter(|(_, _, c)| c.is_some() && c.unwrap() == &'*')
        .collect::<Vec<(usize, usize, Option<&char>)>>()
}

/// Get the numbers in a given row.
fn get_number(chars: &Array2D<char>, row: &usize, column: &usize) -> u32 {
    let mut i = *column;
    let mut j = *column - 1;

    let mut next = Vec::new();
    while chars.get(*row, i).is_some() && chars.get(*row, i).unwrap().is_ascii_digit() {
        next.push(chars.get(*row, i).unwrap());
        i += 1;
    }

    let mut prev = Vec::new();
    while chars.get(*row, j).is_some() && chars.get(*row, j).unwrap().is_ascii_digit() {
        prev.push(chars.get(*row, j).unwrap());
        if j == 0 {
            break;
        }
        j -= 1;
    }

    let mut numbers: Vec<&char> = prev.clone().into_iter().rev().collect();
    numbers.append(&mut next);

    numbers
        .into_iter()
        .fold(String::new(), |a, b| a + &b.to_string())
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
