use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let moves = parse_moves("input/day9.txt");

    println!("Day 9");
    println!("Part 1: {}", count_visited(&moves));
}

enum Direction {
    Left, Right, Up, Down,
}

struct Move {
    dir: Direction,
    amount: usize,
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("L") {
            Ok(Move {
                dir: Direction::Left,
                amount: s.replace("L ", "").parse().unwrap()
            })
        } else if s.starts_with("R") {
            Ok(Move {
                dir: Direction::Right,
                amount: s.replace("R ", "").parse().unwrap()
            })
        } else if s.starts_with("U") {
            Ok(Move {
                dir: Direction::Up,
                amount: s.replace("U ", "").parse().unwrap()
            })
        } else if s.starts_with("D") {
            Ok(Move {
                dir: Direction::Down,
                amount: s.replace("D ", "").parse().unwrap()
            })
        } else {
            panic!("Invalid direction: '{}'", s)
        }
    }
}

fn parse_moves(filename: &str) -> Vec<Move> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten().flat_map(|line| line.parse()).collect_vec()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    fn at(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }

    fn follow(&mut self, head: &Position) {
        if i32::abs(self.x - head.x) == 2 || i32::abs(self.y - head.y) == 2 {
            self.x += (head.x - self.x).signum();
            self.y += (head.y - self.y).signum();
        }
    }
}

/// count_visited returns the number of positions that the tail visited.
fn count_visited(moves: &Vec<Move>) -> usize {
    let mut tail_positions: HashSet<Position> = HashSet::new();
    let mut head = Position::at(0, 0);
    let mut tail = Position::at(0, 0);

    tail_positions.insert(tail);

    for m in moves {
        for _ in 0..m.amount {
            head.step(&m.dir);
            tail.follow(&head);
            tail_positions.insert(tail);
        }
    }

    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow() {
        // Tail is at 2,2.  Cases are head, expected tail position
        let cases = vec![
            // Overlapping
            (Position::at(2, 2), Position::at(2, 2)),

            // Adjacent
            (Position::at(1, 2), Position::at(2, 2)),
            (Position::at(3, 2), Position::at(2, 2)),
            (Position::at(2, 1), Position::at(2, 2)),
            (Position::at(2, 3), Position::at(2, 2)),

            // Two away - left
            (Position::at(0, 0), Position::at(1, 1)),
            (Position::at(0, 1), Position::at(1, 1)),
            (Position::at(0, 2), Position::at(1, 2)),
            (Position::at(0, 3), Position::at(1, 3)),
            (Position::at(0, 4), Position::at(1, 3)),

            // Two away - right
            (Position::at(4, 0), Position::at(3, 1)),
            (Position::at(4, 1), Position::at(3, 1)),
            (Position::at(4, 2), Position::at(3, 2)),
            (Position::at(4, 3), Position::at(3, 3)),
            (Position::at(4, 4), Position::at(3, 3)),

            // Two away - up
            (Position::at(0, 4), Position::at(1, 3)),
            (Position::at(1, 4), Position::at(1, 3)),
            (Position::at(2, 4), Position::at(2, 3)),
            (Position::at(3, 4), Position::at(3, 3)),
            (Position::at(4, 4), Position::at(3, 3)),

            // Two away - down
            (Position::at(0, 0), Position::at(1, 1)),
            (Position::at(1, 0), Position::at(1, 1)),
            (Position::at(2, 0), Position::at(2, 1)),
            (Position::at(3, 0), Position::at(3, 1)),
            (Position::at(4, 0), Position::at(3, 1)),
        ];

        for (head, expected) in cases {
            let mut tail = Position::at(2, 2);

            tail.follow(&head);

            assert_eq!(expected, tail, "head: {:?} - expected: {:?}, actual: {:?}", head, expected, tail);
        }
    }

    #[test]
    fn test_count_visited() {
        let moves = parse_moves("input/day9_sample.txt");

        assert_eq!(13, count_visited(&moves));
    }
}