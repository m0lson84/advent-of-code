advent_of_code::solution!(5);

type Range = (u64, u64);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input)?;

    let count = ids.iter().filter(|&&id| is_fresh(id, &ranges)).count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input)?;
    let merged = merge_ranges(ranges);

    let count: u64 = merged.iter().map(|(start, end)| end - start + 1).sum();

    Some(count)
}

fn parse_input(input: &str) -> Option<(Vec<Range>, Vec<u64>)> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 2 {
        return None;
    }

    let ranges = get_ranges(parts[0]);
    let ids = get_ids(parts[1]);

    Some((ranges, ids))
}

fn get_ranges(section: &str) -> Vec<Range> {
    section
        .lines()
        .filter_map(|line| {
            let nums: Vec<&str> = line.split('-').collect();
            if nums.len() == 2 {
                let start = nums[0].parse().ok()?;
                let end = nums[1].parse().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

fn get_ids(section: &str) -> Vec<u64> {
    section
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn is_fresh(id: u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut sorted = ranges;
    sorted.sort_by_key(|&(start, _)| start);

    let mut merged = vec![sorted[0]];

    for &(start, end) in &sorted[1..] {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        if start <= last_end + 1 {
            merged[last_idx] = (last_start, last_end.max(end));
        } else {
            merged.push((start, end));
        }
    }

    merged
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
        assert_eq!(result, Some(14));
    }
}
