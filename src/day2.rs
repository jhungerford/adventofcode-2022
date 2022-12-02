use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let rounds = load_rounds("input/day2.txt");

    println!("Day 2");
    println!("Part 1: {}", total_score(&rounds, Round::shape_points));
    println!("Part 2: {}", total_score(&rounds, Round::outcome_points));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Shape {
    Rock, Paper, Scissors
}

impl Shape {
    /// parse returns a shape for the given code.
    fn parse(value: &str) -> Self {
        match value {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid shape: {}", value)
        }
    }

    /// loss returns the shape that beats this shape.
    fn loss(self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    /// tie returns the shape that this shape ties with.
    fn tie(self) -> Self {
        return self.clone()
    }

    /// win returns the shape that this shape beats.
    fn win(self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    /// points returns the number of points this shape is worth if you play it.
    fn points(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Loss, Win, Tie
}

impl Outcome {
    /// parse converts the given code into a desired outcome.
    fn parse(value: &str) -> Self {
        match value {
            "X" => Outcome::Loss,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome: {}", value)
        }
    }

    /// play returns the shape you should play to reach this outcome against the opponent.
    fn play(&self, opponent: Shape) -> Shape {
        match self {
            Outcome::Loss => opponent.win(),
            Outcome::Tie => opponent.tie(),
            Outcome::Win => opponent.loss(),
        }
    }

    /// of returns the outcome of you playing a shape against an opponent.
    fn of(you: Shape, opponent: Shape) -> Outcome {
        match opponent {
            _ if opponent == you.tie() => Outcome::Tie,
            _ if opponent == you.win() => Outcome::Win,
            _ if opponent == you.loss() => Outcome::Loss,
            _ => panic!("win, loss, tie tables are incorrect"),
        }
    }

    /// points returns the number of points this round is worth.
    fn points(&self) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

struct Round {
    you: String,
    opponent: String,
}

impl Round {
    /// shape_points returns the number of points that you score in this round where your
    /// guide is the shape you should play.
    fn shape_points(&self) -> i32 {
        let (you, opponent) = (Shape::parse(&self.you), Shape::parse(&self.opponent));

        you.points() + Outcome::of(you, opponent).points()
    }

    /// shape_points returns the number of points that you score in this round where your
    /// guide is the outcome of the round.
    fn outcome_points(&self) -> i32 {
        let (opponent, outcome) = (Shape::parse(&self.opponent), Outcome::parse(&self.you));
        let you = outcome.play(opponent);

        you.points() + outcome.points()
    }
}

/// load_rounds parses rounds out of the given file.  Lines contain an opponent's move and your move,
/// and the meaning of your move changes with the problem parts.
fn load_rounds(filename: &str) -> Vec<Round> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().flatten().map(|line| {
        let mut shapes = line.split_whitespace();

        Round {
            opponent: shapes.next().unwrap().to_string(),
            you: shapes.next().unwrap().to_string()
        }
    }).collect()
}

/// total_score returns the sum of scores in all rounds, using the given scoring function.
fn total_score(rounds: &Vec<Round>, round_score: impl Fn(&Round) -> i32) -> i32 {
    rounds.iter().map(round_score).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_score_shape() {
        let rounds = load_rounds("input/day2_sample.txt");

        assert_eq!(15, total_score(&rounds, Round::shape_points));
    }

    #[test]
    fn test_total_score_outcome() {
        let rounds = load_rounds("input/day2_sample.txt");

        assert_eq!(12, total_score(&rounds, Round::outcome_points));
    }
}
