use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let instructions = parse_instructions("input/day10.txt");

    println!("Day 10");
    println!("Part 1: {}", signal_strength(&instructions));
    println!("Part 2:\n{}", render_image(&instructions));
}

struct Computer {
    register: i32,
}

impl Computer {
    /// new returns a new computer.
    fn new() -> Self {
        Computer { register: 1 }
    }

    /// run executes the given instruction.
    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {},
            Instruction::AddX(value) => self.register += value,
        }
    }

    /// is_lit returns whether the given pixel is lit based on the computer's register.
    /// The two pixels beside the register are lit, all other pixels are dark.
    fn is_lit(&self, pixel: usize) -> bool {
        (self.register-1..=self.register+1).contains(&(pixel as i32))
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
    let mut comp = Computer::new();
    let mut ss = 0;
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

/// print_image runs the instructions, printing the image that results.
fn render_image(instructions: &Vec<Instruction>) -> String {
    let mut comp = Computer::new();
    let mut str = String::new();
    let mut i = 0;

    for instruction in instructions.iter() {
        for _ in 0..instruction.cycles() {
            if comp.is_lit(i % 40) {
                str = format!("{}#", str);
            } else {
                str = format!("{}.", str);
            }

            i += 1;

            if (i % 40) == 0 {
                str = format!("{}\n", str);
            }
        }

        comp.run(instruction);
    }

    str.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_strength() {
        let instructions = parse_instructions("input/day10_sample.txt");

        assert_eq!(13140, signal_strength(&instructions));
    }

    #[test]
    fn test_render_image() {
        let instructions = parse_instructions("input/day10_sample.txt");
        let expected = "\
##..##..##..##..##..##..##..##..##..##..\n\
###...###...###...###...###...###...###.\n\
####....####....####....####....####....\n\
#####.....#####.....#####.....#####.....\n\
######......######......######......####\n\
#######.......#######.......#######.....\n";

        assert_eq!(expected, render_image(&instructions));
    }
}
