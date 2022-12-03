use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let rucksacks = parse_rucksacks("input/day3.txt");

    println!("Day 3");
    println!("Part 1: {}", total_priority(&rucksacks));
    println!("Part 2: {}", badge_priority(&rucksacks));
}

/// parse_rucksacks parses rucksack contents out of the given file, one per line.
fn parse_rucksacks(filename: &str) -> Vec<Rucksack> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten()
        .map(|line| Rucksack{ contents: line})
        .collect()
}

struct Rucksack {
    contents: String
}

impl Rucksack {
    /// shared returns the item that is shared between the two halves of this rucksack.
    fn shared_halves(&self) -> char {
        let middle = self.contents.len() / 2;
        let left = self.contents[..middle].chars().collect::<HashSet<char>>();
        let right = self.contents[middle..].chars().collect::<HashSet<char>>();

        left.intersection(&right).at_most_one().unwrap().unwrap().to_owned()
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
    rucksacks.iter().map(Rucksack::shared_halves).map(priority).sum()
}

/// shared returns the item that is shared between all of the rucksacks.
fn shared_badge(rucksacks: &[Rucksack]) -> char {
    let intersection = rucksacks.iter()
        .map(|r| r.contents.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<char>>()).unwrap();

    intersection.into_iter().at_most_one().unwrap().unwrap().to_owned()
}

/// badge_priority returns the sum of priorities of items that are shared in three-Elf groups.
fn badge_priority(rucksacks: &Vec<Rucksack>) -> i32 {
    rucksacks.chunks_exact(3)
        .map(shared_badge)
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_halves() {
        let rucksacks = parse_rucksacks("input/day3_sample.txt");
        let sample_shared = ['p', 'L', 'P', 'v', 't', 's'];

        for (rucksack, expected) in rucksacks.iter().zip(sample_shared) {
            assert_eq!(expected, rucksack.shared_halves());
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

    #[test]
    fn test_badge_priority() {
        let rucksacks = parse_rucksacks("input/day3_sample.txt");

        assert_eq!(70, badge_priority(&rucksacks));
    }
}