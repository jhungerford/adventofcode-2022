use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let assignments = parse_assignments("input/day4.txt");

    println!("Day 4");
    println!("Part 1: {}", num_overlap(&assignments));
}

fn parse_assignments(filename: &str) -> Vec<Assignment> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten()
        .map(|line| line.parse()).flatten()
        .collect::<Vec<Assignment>>()
}

struct Assignment {
    a: RangeInclusive<i32>,
    b: RangeInclusive<i32>,
}

impl FromStr for Assignment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Assignment looks like '2-4,6-8'

        let assignments = s.split(",").collect_vec();
        let a_bounds: Vec<i32> = assignments[0].split("-").map(|s| s.parse()).flatten().collect_vec();
        let b_bounds: Vec<i32> = assignments[1].split("-").map(|s| s.parse()).flatten().collect_vec();

        Ok(Assignment {
            a: RangeInclusive::new(a_bounds[0], a_bounds[1]),
            b: RangeInclusive::new(b_bounds[0], b_bounds[1]),
        })
    }
}

impl Assignment {
    /// overlap returns whether one of the ranges in this assignment completely contains the other.
    fn overlap(&self) -> bool {
        (self.a.contains(self.b.start()) && self.a.contains(self.b.end()))
            || (self.b.contains(self.a.start()) && self.b.contains(self.a.end()))
    }
}

/// num_overlap returns the number of assignments where one assignment completely covers the other.
fn num_overlap(assignments: &Vec<Assignment>) -> usize {
    assignments.iter().filter(|a| a.overlap()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_overlap() {
        let assignments = parse_assignments("input/day4_sample.txt");

        assert_eq!(2, num_overlap(&assignments));
    }
}