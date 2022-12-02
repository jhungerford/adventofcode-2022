use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let rounds = load_rounds("input/day2.txt");

    println!("Day 2");
    println!("Part 1: {}", total_score(&rounds));
}

#[derive(Eq, PartialEq)]
enum Shape {
    Rock, Paper, Scissors
}

impl Shape {
    fn parse(value: &str) -> Self {
        match value {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid shape: {}", value)
        }
    }

    fn points(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Eq, PartialEq)]
enum Outcome {
    Loss, Win, Tie
}

impl Outcome {
    fn points(&self) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

struct Round {
    you: Shape,
    opponent: Shape,
}

impl Round {
    fn points(&self) -> i32 {
        return self.you.points() + self.outcome().points()
    }

    fn outcome(&self) -> Outcome {
        if self.you == self.opponent {
            return Outcome::Tie;
        }

        let self_beats_shape = match self.you {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        };

        if self_beats_shape == self.opponent {
            return Outcome::Win;
        }

        return Outcome::Loss;
    }
}

fn load_rounds(filename: &str) -> Vec<Round> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().flatten().map(|line| {
        let mut shapes: Vec<Shape> = line.split_whitespace().map(Shape::parse).collect();

        Round { opponent: shapes.remove(0), you: shapes.remove(0) }
    }).collect()
}

fn total_score(rounds: &Vec<Round>) -> i32 {
    rounds.iter().map(Round::points).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scores() {
        // Lose
        assert_eq!(1, Round{you: Shape::Rock, opponent: Shape::Paper}.points());
        assert_eq!(2, Round{you: Shape::Paper, opponent: Shape::Scissors}.points());
        assert_eq!(3, Round{you: Shape::Scissors, opponent: Shape::Rock}.points());

        // Tie
        assert_eq!(4, Round{you: Shape::Rock, opponent: Shape::Rock}.points());
        assert_eq!(5, Round{you: Shape::Paper, opponent: Shape::Paper}.points());
        assert_eq!(6, Round{you: Shape::Scissors, opponent: Shape::Scissors}.points());

        // Win
        assert_eq!(7, Round{you: Shape::Rock, opponent: Shape::Scissors}.points());
        assert_eq!(8, Round{you: Shape::Paper, opponent: Shape::Rock}.points());
        assert_eq!(9, Round{you: Shape::Scissors, opponent: Shape::Paper}.points());
    }

    #[test]
    fn test_total_score() {
        let rounds = load_rounds("input/day2_sample.txt");

        assert_eq!(3, rounds.len());
        assert_eq!(15, total_score(&rounds));
    }
}
