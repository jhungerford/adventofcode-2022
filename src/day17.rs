use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use itertools::Itertools;

pub fn solution() {
    let directions = load_directions("input/day17.txt");

    println!("Day 17");
    println!("Part 1: {}", height_after_rocks(&directions, 2022));
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left, Right
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!("Invalid direction: '{}'", c)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn direction(&self, dir: Direction) -> Self {
        match dir {
            Direction::Left => self + Position::at(0, -1),
            Direction::Right => self + Position::at(0, 1),
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<&Position> for Position {
    type Output = Position;

    fn add(self, rhs: &Self) -> Self::Output {
        self + *rhs
    }
}

impl Add<Position> for &Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        *self + rhs
    }
}

impl Position {
    fn at(row: isize, col: isize) -> Self {
        Position { row, col }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Rock {
    /// Center:
    /// ```
    /// *###
    /// ```
    Dash,

    /// Center:
    /// ```
    /// .#.
    /// #*#
    /// .#.
    /// ```
    Plus,

    /// Center:
    /// ```
    /// ..#
    /// ..#
    /// ##*
    /// ```
    L,

    /// Center:
    /// ```
    /// *
    /// #
    /// #
    /// #
    /// ```
    I,

    /// Center:
    /// ```
    /// *#
    /// ##
    /// ```
    Square,
}

impl Rock {
    fn order() -> Vec<Rock> {
        vec![Rock::Dash, Rock::Plus, Rock::L, Rock::I, Rock::Square]
    }

    fn positions(&self, pos: &Position) -> Vec<Position> {
        match self {
            Rock::Dash => vec![
                pos + Position::at(0, 0),
                pos + Position::at(0, 1),
                pos + Position::at(0, 2),
                pos + Position::at(0, 3),
            ],
            Rock::Plus => vec![
                pos + Position::at(0, -1),
                pos + Position::at(-1, 0),
                pos + Position::at(0, 0),
                pos + Position::at(1, 0),
                pos + Position::at(0, 1),
            ],
            Rock::L => vec![
                pos + Position::at(0, -2),
                pos + Position::at(0, -1),
                pos + Position::at(0, 0),
                pos + Position::at(1, 0),
                pos + Position::at(2, 0),
            ],
            Rock::I => vec![
                pos + Position::at(0, 0),
                pos + Position::at(-1, 0),
                pos + Position::at(-2, 0),
                pos + Position::at(-3, 0),
            ],
            Rock::Square => vec![
                pos + Position::at(0, 0),
                pos + Position::at(0, 1),
                pos + Position::at(-1, 0),
                pos + Position::at(-1, 1),
            ],
        }
    }
}

struct Board {
    rock_rows: Vec<[bool; 7]>,
}

impl Board {
    /// new returns a board with no rocks.
    fn new() -> Self {
        Board { rock_rows: Vec::new() }
    }

    /// start returns the starting position for the given type of rock.  A Rock appears with its
    /// left edge two units away from the left wall and its bottom edge three units above the
    /// highest rock in the room, or the floor.
    fn start(&self, rock: Rock) -> Position {
        match rock {
            Rock::Dash => Position::at(self.len() + 3, 2),
            Rock::Plus => Position::at(self.len() + 4, 3),
            Rock::L => Position::at(self.len() + 3, 4),
            Rock::I => Position::at(self.len() + 6, 2),
            Rock::Square => Position::at(self.len() + 4, 2),
        }
    }

    /// push returns the position of the rock after being pushed by the wind in the given direction.
    /// If the rock runs into the walls or another rock, it doesn't move.
    fn push(&self, rock: Rock, old_pos: Position, dir: Direction) -> Position {
        let new_pos = old_pos.direction(dir);

        if self.collision(rock, new_pos) {
            old_pos
        } else {
            new_pos
        }
    }

    /// fall returns the position of the rock falling one square down, or None if the rock
    /// has reached the floor or is on top of another rock.
    fn fall(&self, rock: Rock, old_pos: Position) -> Option<Position> {
        let new_pos = old_pos + Position::at(-1, 0);

        if self.collision(rock, new_pos) {
            None
        } else {
            Some(new_pos)
        }
    }

    /// end records the position of the rock on the board
    fn end(&mut self, rock: Rock, pos: Position) {
        // make sure there's enough rows on the board for the rock.
        let rock_top = match rock {
            Rock::Dash => pos.row,
            Rock::Plus => pos.row + 1,
            Rock::L => pos.row + 2,
            Rock::I => pos.row,
            Rock::Square => pos.row,
        } as usize;

        for _ in self.rock_rows.len()..=rock_top {
            self.rock_rows.push([false; 7])
        }

        // record the rock positions.
        for p in rock.positions(&pos) {
            self.rock_rows[p.row as usize][p.col as usize] = true
        }
    }

    /// collision returns whether the rock is out of bounds or
    /// overlaps with another rock on the board.
    fn collision(&self, rock: Rock, pos: Position) -> bool {
        !rock.positions(&pos).iter().all(|p| self.empty(p))
    }

    /// empty returns whether the given position is empty on this board.
    fn empty(&self, pos: &Position) -> bool {
        if pos.row < 0 || pos.col < 0 || pos.col >= 7 {
            return false;
        }

        if pos.row >= self.len() {
            return true;
        }

        !self.rock_rows[pos.row as usize][pos.col as usize]
    }

    /// len returns the number of rows in this board that have rocks.
    fn len(&self) -> isize {
        self.rock_rows.len() as isize
    }
}

fn load_directions(filename: &str) -> Vec<Direction> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten().next().unwrap().chars().map(|c| Direction::from(c)).collect_vec()
}

/// height_after_rocks returns the height of the tower after the given number of rocks have fallen.
fn height_after_rocks(directions: &Vec<Direction>, rocks: usize) -> isize {
    let mut wind = directions.into_iter().cycle();
    let mut rock_order = Rock::order().into_iter().cycle();
    let mut board = Board::new();

    for _ in 0..rocks {
        let rock = rock_order.next().unwrap();
        let mut pos = board.start(rock);

        let mut falling = true;
        while falling {
            // rock is pushed by the wind, then falls downward.
            let wind_dir = wind.next().unwrap();
            pos = board.push(rock, pos, *wind_dir);

            // rock falls until it's downward movement is blocked.
            let maybe_fall_pos = board.fall(rock, pos);
            if let Some(fall_pos) = maybe_fall_pos {
                pos = fall_pos;
            } else {
                board.end(rock, pos);
                falling = false;
            }
        }
    }

    board.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_after_rocks() {
        let directions = load_directions("input/day17_sample.txt");

        assert_eq!(3068, height_after_rocks(&directions, 2022));
    }
}
