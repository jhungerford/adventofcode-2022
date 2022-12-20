use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let lines = parse_lines("input/day14.txt");

    println!("Day 14");
    println!("Part 1: {}", sand_falling(&lines));
    println!("Part 2: {}", sand_floor(&lines));
}

fn parse_lines(filename: &str) -> Vec<Vec<Point>> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten().map(|line| line.split(" -> ").map(|point| {
        let mut coords = point.split(",");
        Point {
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
        }
    }).collect_vec()).collect_vec()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn at(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn to(&self, end: &Self) -> Vec<Self> {
        if self.x == end.x {
            (self.y.min(end.y)..=self.y.max(end.y)).map(|y| Point::at(self.x, y)).collect_vec()
        } else {
            (self.x.min(end.x)..=self.x.max(end.x)).map(|x| Point::at(x, self.y)).collect_vec()
        }
    }

    fn down(&self) -> Self {
        return Self::at(self.x, self.y + 1);
    }

    fn left(&self) -> Self {
        return Self::at(self.x - 1, self.y + 1);
    }

    fn right(&self) -> Self {
        return Self::at(self.x + 1, self.y + 1);
    }
}

/// sand_falling returns the number of grains of sand that come to rest after being dropped
/// from 500,0 with the given rocks, with no floor.
fn sand_falling(lines: &Vec<Vec<Point>>) -> usize {
    let mut filled = draw_rocks(lines);

    let num_rocks = filled.len();
    let lowest_rock = filled.iter().map(|rock| rock.y).max().unwrap();

    // Sand falls from 500,0, and tries to drop down,
    // down and to the left, then down and to the right.
    loop {
        let mut sand = Point::at(500, 0);
        if filled.contains(&sand) {
            break
        }

        while sand.y < lowest_rock {
            let (down, left, right) = (sand.down(), sand.left(), sand.right());

            if !filled.contains(&down) {
                sand = down;
            } else if !filled.contains(&left) {
                sand = left;
            } else if !filled.contains(&right) {
                sand = right;
            } else {
                filled.insert(sand);
                break;
            }
        }

        if sand.y >= lowest_rock {
            break;
        }
    }

    filled.len() - num_rocks
}

/// sand_floor returns the number of grains of sand that come to rest after being dropped
/// from 500,0 with the given rocks, with a floor below the lowest rock.
fn sand_floor(lines: &Vec<Vec<Point>>) -> usize {
    let mut filled = draw_rocks(lines);

    let num_rocks = filled.len();
    let lowest_rock = filled.iter().map(|rock| rock.y).max().unwrap();

    // Sand falls from 500,0, and tries to drop down,
    // down and to the left, then down and to the right.
    loop {
        let mut sand = Point::at(500, 0);
        if filled.contains(&sand) {
            break
        }

        while sand.y < lowest_rock + 1 {
            let (down, left, right) = (sand.down(), sand.left(), sand.right());

            if !filled.contains(&down) {
                sand = down;
            } else if !filled.contains(&left) {
                sand = left;
            } else if !filled.contains(&right) {
                sand = right;
            } else {
                filled.insert(sand);
                break;
            }
        }

        if sand.y == lowest_rock + 1 {
            filled.insert(sand);
        }
    }

    filled.len() - num_rocks
}

fn draw_rocks(lines: &Vec<Vec<Point>>) -> HashSet<Point> {
    let mut filled: HashSet<Point> = HashSet::new();

    // Fill in all of the solid lines.
    for line in lines {
        for segment in line.windows(2) {
            for point in segment[0].to(&segment[1]) {
                filled.insert(point);
            }
        }
    }

    filled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sand_falling() {
        let lines = parse_lines("input/day14_sample.txt");
        assert_eq!(24, sand_falling(&lines));
    }

    #[test]
    fn test_sand_floor() {
        let lines = parse_lines("input/day14_sample.txt");
        assert_eq!(93, sand_floor(&lines));
    }
}
