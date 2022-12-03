use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let rucksacks = parse_rucksacks("input/day3.txt");

    println!("Day 3");
    println!("Part 1: {}", total_priority(&rucksacks));
}

fn parse_rucksacks(filename: &str) -> Vec<Rucksack> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten()
        .map(|line| Rucksack::from_str(&line))
        .collect()
}

struct Rucksack {
    left: String,
    right: String,
}

impl Rucksack {

    /// from_str parses a Rucksack from the given string.
    fn from_str(str: &str) -> Self {
        let middle = str.len() / 2;
        let (left, right) = (str[..middle].to_string(), str[middle..].to_string());

        Rucksack {left, right}
    }

    /// shared returns the item that is shared between two rucksacks.
    fn shared(&self) -> char {
        let left_items = self.left.chars().collect::<HashSet<char>>();
        let right_items = self.right.chars().collect::<HashSet<char>>();

        left_items.intersection(&right_items).at_most_one().unwrap().unwrap().to_owned()
    }
}

/// priority returns the priority score of the given item.  a-z has 1-26, A-Z has 27-52.
fn priority(item: char) -> i32 {
    if item.is_ascii_lowercase() {
        return ((item as i32) - ('a' as i32)) + 1
    } else if item.is_ascii_uppercase() {
        return ((item as i32) - ('A' as i32)) + 27
    }

    panic!("Invalid item - items must be a-z or A-Z: {}", item)
}

/// total_priority returns the sum of the priority of the shared item in each rucksack.
fn total_priority(rucksacks: &Vec<Rucksack>) -> i32 {
    rucksacks.iter().map(Rucksack::shared).map(priority).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared() {
        let rucksacks = parse_rucksacks("input/day3_sample.txt");
        let sample_shared = ['p', 'L', 'P', 'v', 't', 's'];

        for (rucksack, expected) in rucksacks.iter().zip(sample_shared) {
            assert_eq!(expected, rucksack.shared());
        }
    }

    #[test]
    fn test_priority() {
        assert_eq!(16, priority('p'));
        assert_eq!(38, priority('L'));
    }

    #[test]
    fn test_total_priority() {
        let rucksacks = parse_rucksacks("input/day3_sample.txt");

        assert_eq!(157, total_priority(&rucksacks));
    }
}