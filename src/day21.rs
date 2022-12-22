use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;

pub fn solution() {
    let monkeys = load_monkeys("input/day21.txt");

    println!("Day 21");
    println!("Part 1: {}", root_yell(&monkeys));
}

enum Yell {
    Number(i64),
    Plus(String, String),
    Minus(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn yell_value(monkey: &String, monkeys: &HashMap<String, Yell>) -> i64 {
    let yell = &monkeys[monkey];

    match yell {
        Yell::Number(num) => *num,
        Yell::Plus(a, b) => yell_value(a, monkeys) + yell_value(b, monkeys),
        Yell::Minus(a, b) => yell_value(a, monkeys) - yell_value(b, monkeys),
        Yell::Multiply(a, b) => yell_value(a, monkeys) * yell_value(b, monkeys),
        Yell::Divide(a, b) => yell_value(a, monkeys) / yell_value(b, monkeys),
    }
}

impl FromStr for Yell {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // yells look like '5', or 'drzm * dbpl'
        if !s.contains(" ") {
            return Ok(Yell::Number(s.parse().unwrap()))
        }

        let parts = s.split_whitespace().collect_vec();

        match parts[1] {
            "+" => Ok(Yell::Plus(parts[0].to_string(), parts[2].to_string())),
            "-" => Ok(Yell::Minus(parts[0].to_string(), parts[2].to_string())),
            "*" => Ok(Yell::Multiply(parts[0].to_string(), parts[2].to_string())),
            "/" => Ok(Yell::Divide(parts[0].to_string(), parts[2].to_string())),
            _ => panic!("Invalid yell: {}", s),
        }
    }
}

fn load_monkeys(filename: &str) -> HashMap<String, Yell> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten().map(|line| {
        let mut parts = line.split(": ");
        (parts.next().unwrap().to_string(), parts.next().unwrap().parse().unwrap())
    }).collect()
}

fn root_yell(monkeys: &HashMap<String, Yell>) -> i64 {
    yell_value(&"root".to_string(), monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_yell() {
        let monkeys = load_monkeys("input/day21_sample.txt");
        assert_eq!(152, root_yell(&monkeys));
    }
}