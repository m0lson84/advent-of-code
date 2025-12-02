advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for range in input.split(',') {
        let (first_id, last_id) = get_ids(range)?;
        result += (first_id..=last_id)
            .filter(|&id| is_invalid(id.to_string()))
            .sum::<u64>();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    for range in input.split(',') {
        let (first_id, last_id) = get_ids(range)?;
        result += (first_id..=last_id)
            .filter(|&id| is_invalid_strict(id.to_string()))
            .sum::<u64>();
    }
    Some(result)
}

fn get_ids(range: &str) -> Option<(u64, u64)> {
    let ids: Vec<&str> = range.trim().split('-').collect();
    let first = ids[0].parse::<u64>().ok()?;
    let last = ids[1].parse::<u64>().ok()?;
    Some((first, last))
}

fn is_invalid(id: String) -> bool {
    let length = id.len();
    if !length.is_multiple_of(2) {
        return false;
    }

    let mid = length / 2;
    id[0..mid] == id[mid..]
}

fn is_invalid_strict(id: String) -> bool {
    let length = id.len();
    for i in 1..=length / 2 {
        if !length.is_multiple_of(i) {
            continue;
        }

        let candidate = &id[0..i];
        if candidate.repeat(length / i) == id {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
