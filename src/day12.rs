use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn solution() {
    let map = parse_map("input/day12.txt");

    println!("Day 12");
    println!("Part 1: {}", fewest_steps(&map));
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn at(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

struct Map {
    heights: Vec<Vec<char>>,
    start: Position,
    end: Position,
}

impl Map {
    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();

        if pos.row > 0 {
            neighbors.push(Position::at(pos.row - 1, pos.col));
        }

        if pos.row < self.heights.len() - 1 {
            neighbors.push(Position::at(pos.row + 1, pos.col));
        }

        if pos.col > 0 {
            neighbors.push(Position::at(pos.row, pos.col - 1));
        }

        if pos.col < self.heights[0].len() - 1 {
            neighbors.push(Position::at(pos.row, pos.col + 1));
        }

        let height = self.height(pos);

        neighbors.into_iter()
            .filter(|neighbor| self.height(neighbor) <= height + 1)
            .collect_vec()
    }

    fn height(&self, pos: &Position) -> i32 {
        return self.heights[pos.row][pos.col] as i32 - 'a' as i32;
    }
}

fn parse_map(filename: &str) -> Map {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    let mut heights = Vec::new();
    let mut start = Position::at(0, 0);
    let mut end = Position::at(0, 0);

    for (row, line) in f.lines().flatten().enumerate() {
        let mut map_row = Vec::new();

        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Position::at(row, col);
                map_row.push('a');
            } else if c == 'E' {
                end = Position::at(row, col);
                map_row.push('z');
            } else {
                map_row.push(c);
            }
        }

        heights.push(map_row);
    }

    Map { heights, start, end }
}

/// fewest_steps returns the fewest number of steps it takes to get from start to end on the map,
/// only going up at most one letter.
fn fewest_steps(map: &Map) -> usize {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: Position,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
                .then_with(|| self.position.row.cmp(&other.position.row))
                .then_with(|| self.position.col.cmp(&other.position.col))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // dist[row][col] = current shortest distance from `start` to `node`
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; map.heights[0].len()]; map.heights.len()];

    let mut heap = BinaryHeap::new();

    dist[map.start.row][map.start.col] = 0;
    heap.push(State { cost: 0, position: map.start });

    while let Some(State { cost, position }) = heap.pop() {
        // Found the goal - this is the shortest path
        if position == map.end {
            return cost;
        }

        // Already found a shorter path to this position.
        if cost > dist[position.row][position.col] {
            continue;
        }

        // For each node we can reach, see if we can find a way with a lower cost through this pos.
        for neighbor in map.neighbors(&position) {
            let next = State { cost: cost + 1, position: neighbor };

            if next.cost < dist[neighbor.row][neighbor.col] {
                heap.push(next);
                dist[neighbor.row][neighbor.col] = next.cost;
            }
        }
    }

    usize::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fewest_steps() {
        let map = parse_map("input/day12_sample.txt");

        assert_eq!(31, fewest_steps(&map));
    }
}