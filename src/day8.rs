use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let trees = load_trees("input/day8.txt");

    println!("Day 8");
    println!("Part 1: {}", count_visible(&trees));
}

type Trees = Vec<Vec<u32>>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point { row: usize, col: usize}

impl Point {
    fn at(row: usize, col: usize) -> Self {
        Point { row, col }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

fn tree_at(trees: &Trees, point: Point) -> u32 {
    trees[point.row][point.col]
}

fn load_trees(filename: &str) -> Trees {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten()
        .map(|line| line.chars().map(|c| c.to_digit(10)).flatten().collect_vec())
        .collect_vec()
}

/// count_visible returns the total number of trees that are visible from the sides of the forest.
fn count_visible(trees: &Trees) -> usize {
    let mut visible: HashSet<Point> = HashSet::new();

    let (height, width) = (trees.len(), trees[0].len());

    for row in 0..height {
        let (mut tallest_left, mut tallest_right) = (-1, -1);

        for col in 0..width {
            let (left, right) = (Point::at(row, col), Point::at(row, width - col - 1));
            let (left_height, right_height) = (tree_at(trees, left), tree_at(trees, right));

            if left_height as i32 > tallest_left {
                visible.insert(left);
                tallest_left = left_height as i32;
            }

            if right_height as i32 > tallest_right {
                visible.insert(right);
                tallest_right = right_height as i32;
            }
        }
    }

    for col in 0..width {
        let (mut tallest_top, mut tallest_bottom) = (-1, -1);

        for row in 0..height {
            let (top, bottom) = (Point::at(row, col), Point::at(height - row - 1, col));
            let (top_height, bottom_height) = (tree_at(trees, top), tree_at(trees, bottom));

            if top_height as i32 > tallest_top {
                visible.insert(top);
                tallest_top = top_height as i32;
            }

            if bottom_height as i32 > tallest_bottom {
                visible.insert(bottom);
                tallest_bottom = bottom_height as i32;
            }
        }
    }

    visible.len()
}

#[allow(dead_code)]
fn print_visible(trees: &Trees, visible: &HashSet<Point>) {
    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let point = Point::at(row, col);

            if visible.contains(&point) {
                print!("#")
            } else {
                print!(" ")
            }
        }

        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_visible() {
        let trees = load_trees("input/day8_sample.txt");

        assert_eq!(21, count_visible(&trees));
    }
}