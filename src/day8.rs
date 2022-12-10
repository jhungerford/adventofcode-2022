use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use rayon::prelude::*;

#[allow(dead_code)]
pub fn solution() {
    let trees = load_trees("input/day8.txt");

    println!("Day 8");
    println!("Part 1: {}", count_visible(&trees));
    println!("Part 2: {}", highest_score(&trees));
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
        for col in 0..width {
            let right = (0..width).map(|c| Point::at(row, c)).collect_vec();
            let left = (0..width).rev().map(|c| Point::at(row, c)).collect_vec();
            let down = (0..height).map(|r| Point::at(r, col)).collect_vec();
            let up = (0..height).rev().map(|r| Point::at(r, col)).collect_vec();

            for order in [right, left, down, up] {
                let mut tallest =  -1;
                for point in order {
                    let height = tree_at(trees, point) as i32;
                    if height > tallest {
                        visible.insert(point);
                        tallest = height;
                    }
                }
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

/// highest_score returns the highest tree score out of all the trees, where a tree score is the
/// product of the distance from a tree to a tree with it's height or higher in all directions.
fn highest_score(trees: &Trees) -> usize {
    let points = (0..trees.len()).cartesian_product(0..trees[0].len())
        .map(|(row, col)| Point::at(row, col))
        .collect_vec();

    points.par_iter()
        .map(|&point| tree_score(trees, point))
        .max()
        .unwrap_or(0)
}

/// tree_score returns the score for the given tree.  A tree's score is the product of the
/// number of trees visible from a location that are shorter than the tree.
fn tree_score(trees: &Trees, at: Point) -> usize {
    let (height, width) = (trees.len(), trees[0].len());

    let right = (at.col..width).map(|c| Point::at(at.row, c)).collect_vec();
    let left = (0..=at.col).rev().map(|c| Point::at(at.row, c)).collect_vec();
    let down = (0..=at.row).rev().map(|r| Point::at(r, at.col)).collect_vec();
    let up= (at.row..height).map(|r| Point::at(r, at.col)).collect_vec();

    let tree_height = tree_at(trees, at);

    let mut score = 1;

    for order in [up, left, right, down] {
        let mut dir_score = 0;
        for point in order.into_iter().skip(1) {
            dir_score += 1;

            if tree_height <= tree_at(trees, point) {
                break;
            }
        }

        score *= dir_score;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_visible() {
        let trees = load_trees("input/day8_sample.txt");

        assert_eq!(21, count_visible(&trees));
    }

    #[test]
    fn test_highest_score() {
        let trees = load_trees("input/day8_sample.txt");

        assert_eq!(8, highest_score(&trees));
    }
}