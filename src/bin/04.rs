use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    numbers: HashSet<u32>,
    winning: HashSet<u32>,
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = HashSet::from_iter(get_numeric_array(
            s.split(':').last().unwrap().split('|').last().unwrap(),
        ));
        let winning = HashSet::from_iter(get_numeric_array(
            s.split(':').last().unwrap().split('|').next().unwrap(),
        ));
        Ok(Card { numbers, winning })
    }
}

/// Part one of today's puzzle.
pub fn part_one(input: &str) -> Option<u32> {
    let cards = get_cards(input);

    let mut result: u32 = 0;
    for (_, Card { numbers, winning }) in cards.iter().enumerate() {
        let common: u32 = winning.intersection(numbers).count() as u32;
        if common == 0 {
            continue;
        }

        result += 2u32.pow(common.saturating_sub(1));
    }

    Some(result)
}

/// Part two of today's puzzle.
pub fn part_two(input: &str) -> Option<u32> {
    let cards = get_cards(input);

    let mut copies = vec![1u32; cards.len()];
    for (i, Card { numbers, winning }) in cards.iter().enumerate() {
        let common = winning.intersection(numbers).count();
        if common == 0 {
            continue;
        }

        let times = copies[i];
        for copy in &mut copies[(i + 1)..=(i + common)] {
            *copy += times;
        }
    }

    let result = copies.iter().sum();
    Some(result)
}

fn get_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let card = Card::from_str(line).unwrap();
        cards.push(card);
    }
    cards
}

/// Convert a string into a vector of numbers.
fn get_numeric_array(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
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
        assert_eq!(result, Some(30));
    }
}
