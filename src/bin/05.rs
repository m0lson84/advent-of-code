use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    destination: u32,
    source: u32,
    length: u32,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        Ok(Map {
            destination: split.next().unwrap().parse().unwrap(),
            source: split.next().unwrap().parse().unwrap(),
            length: split.next().unwrap().parse().unwrap(),
        })
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        format!(
            "{{ source: {}, destination: {}, length: {} }}",
            self.source, self.destination, self.length
        )
    }
}

impl Map {
    fn apply(&self, input: &u32) -> u32 {
        let source = Vec::from_iter(self.source..self.source + self.length);
        let destination = Vec::from_iter(self.destination..self.destination + self.length);

        let index = match source.iter().position(|x| x == input) {
            Some(index) => index,
            None => return *input,
        };

        return *destination.get(index).unwrap();
    }
}

/// Part one of today's puzzle.
pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let seeds = get_seeds(lines[0]);

    let mut maps = Vec::new();
    for line in lines[1..].iter() {
        if !contains_numeric(line) {
            continue;
        }
        maps.push(Map::from_str(line).unwrap());
    }

    let mut mapped = Vec::new();
    for seed in seeds {
        let mut current = seed;
        for map in maps.iter() {
            current = map.apply(&current);
        }
        mapped.push(current);
    }

    let result = *mapped.iter().min().unwrap();

    Some(result)
}

/// Part two of today's puzzle.
pub fn part_two(_input: &str) -> Option<u32> {
    Some(0)
}

fn contains_numeric(s: &str) -> bool {
    s.chars().any(|c| c.is_numeric())
}

fn get_seeds(input: &str) -> Vec<u32> {
    input
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
