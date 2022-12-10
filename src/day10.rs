use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;

pub fn solution() {
    let instructions = parse_instructions("input/day10.txt");

    println!("Day 10");
    println!("Part 1: {}", signal_strength(&instructions));
}

struct Computer {
    register: i32,
}

impl Computer {
    fn new() -> Self {
        Computer { register: 1 }
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {},
            Instruction::AddX(value) => self.register += value,
        }
    }
}

enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {

    /// cycles returns the number of cycles this instruction takes to complete.
    fn cycles(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop)
        }

        Ok(Instruction::AddX(s.replace("addx ", "").parse().unwrap()))
    }
}

fn parse_instructions(filename: &str) -> Vec<Instruction> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten().flat_map(|line| line.parse::<Instruction>()).collect_vec()
}

/// signal_strength returns the signal strength - cycle number multiplied by the register during
/// the 20th cycle, and every 40 cycles after that
fn signal_strength(instructions: &Vec<Instruction>) -> i32 {
    let mut ss = 0;
    let mut comp = Computer::new();
    let mut i = 0;

    for instruction in instructions.iter() {
        for _ in 0..instruction.cycles() {
            i += 1;

            if (i - 20) % 40 == 0 {
                ss += i as i32 * comp.register;
            }
        }

        comp.run(instruction);
    }

    ss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_strength() {
        let instructions = parse_instructions("input/day10_sample.txt");

        assert_eq!(13140, signal_strength(&instructions));
    }
}
