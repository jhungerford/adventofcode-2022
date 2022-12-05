use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let puzzle = parse_puzzle("input/day5.txt");

    println!("Day 5");
    println!("Part 1: {}", puzzle.run());
}

struct Puzzle {
    crates: Vec<VecDeque<char>>,
    instructions: Vec<Instruction>,
}

fn parse_puzzle(filename: &str) -> Puzzle {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    let mut parsing_crates = true;
    let mut crate_lines = Vec::new();
    let mut instructions = Vec::new();

    for line in f.lines().flatten() {
        if parsing_crates {
            // Beginning of file is vertical ascii art of crates, followed by a newline.
            if &line == "" {
                parsing_crates = false;
            } else {
                crate_lines.push(line);
            }
        } else {
            // Rest of the file is instructions
            instructions.push(line.parse::<Instruction>().unwrap());
        }
    }

    let crates = parse_crates(&crate_lines);

    Puzzle { crates, instructions }
}

/// parse_crates parses lines that look like the following, with an arbitrary number of crates:
/// ```
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
/// ```
fn parse_crates(lines: &Vec<String>) -> Vec<VecDeque<char>> {
    // Last line has crate numbers, crates may appear at the 1, 5, 9, etc. index on each line.
    // The last line doesn't have trailing whitespace or a column, so num_crates = (len + 2) / 4.
    fn num_crates(line_len: usize) -> usize {
        (line_len + 2) / 4
    }

    let mut crates = vec![VecDeque::new(); num_crates(lines.last().unwrap().len())];

    // Crates can't float over empty air, so look at them in reverse.
    // The last line has crate numbers, so skip it.  Crate contents are at index 1, +4, etc.
    lines.iter().rev().skip(1).for_each(|line: &String| {
        let indexes = 0..num_crates(line.len());
        let contents = line.chars().skip(1).step_by(4);

        indexes.zip(contents)
            .filter(|(_, c)| c != &' ')
            .for_each(|(i, c)| crates[i].push_back(c));
    });

    crates
}

impl Puzzle {
    fn run(&self) -> String {
        let mut crates = self.crates.clone();

        for instruction in &self.instructions {
            instruction.run(&mut crates);
        }

        // Puzzle solution is the letters in the crate at the top of each stack.
        // Some stacks may be empty - flatten to ignore them in the answer.
        crates.iter()
            .map(|c: &VecDeque<char>| c.back()).flatten()
            .cloned().collect::<String>()
    }
}

struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Instructions look like 'move 3 from 1 to 3'
        let nums = s.replace("move ", "").replace("from ", "").replace("to ", "")
            .split_whitespace()
            .map(|n| n.parse::<usize>())
            .flatten()
            .collect_vec();

        return Ok(Instruction {
            num: nums[0],
            from: nums[1],
            to: nums[2],
        })
    }
}

impl Instruction {
    fn run(&self, crates: &mut Vec<VecDeque<char>>) {
        // Instructions look like 'move 3 from 1 to 3'
        for _ in 0..self.num {
            if let Some(c) = crates[self.from.clone() - 1].pop_back() {
                crates[self.to.clone() - 1].push_back(c)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_run() {
        let puzzle = parse_puzzle("input/day5_sample.txt");

        assert_eq!("CMZ", puzzle.run());
    }
}