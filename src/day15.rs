use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::string::ParseError;

use itertools::Itertools;
use regex::Regex;

pub fn solution() {
    let sensors = load_sensors("input/day15.txt");

    println!("Day 15");
    println!("Part 1: {}", no_beacon(&sensors, 2000000));
    println!("Part 2: {}", tuning_frequency(&sensors, 4000000));
}

fn load_sensors(filename: &str) -> Vec<Sensor> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten().map(|line| line.parse()).flatten().collect_vec()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    col: i64,
    row: i64,
}

impl Point {
    fn at(col: i64, row: i64) -> Self {
        Point { col, row }
    }
}

struct Sensor {
    sensor: Point,
    beacon: Point,
    dist: i64
}

impl FromStr for Sensor {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let captures = re.captures(str).unwrap();

        let sensor =Point::at(captures[1].parse().unwrap(), captures[2].parse().unwrap());
        let beacon = Point::at(captures[3].parse().unwrap(), captures[4].parse().unwrap());
        let dist = (sensor.col - beacon.col).abs() + (sensor.row - beacon.row).abs();

        Ok(Sensor { sensor, beacon, dist })
    }
}

/// no_beacon returns the number of squares on the given row that can't have a beacon.
fn no_beacon(sensors: &Vec<Sensor>, row: i64) -> i64 {
    let mut ranges: Vec<RangeInclusive<i64>> = Vec::new();
    let mut beacons: HashSet<Point> = HashSet::new();

    for sensor in sensors {
        if (sensor.sensor.row - row).abs() > sensor.dist {
            continue
        }

        // number of squares to the left and right of sensor.x on the line
        let num_squares = sensor.dist - (row - sensor.sensor.row).abs();

        ranges.push(sensor.sensor.col - num_squares..=sensor.sensor.col + num_squares);

        if sensor.beacon.row == row {
            beacons.insert(sensor.beacon);
        }
    }

    // Collapse overlapping ranges
    ranges.sort_by_key(|range| *range.start());

    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i].end() >= ranges[i+1].start() {
            let end = *ranges[i].end().max(ranges[i+1].end());

            ranges[i] = *ranges[i].start()..=end;
            ranges.remove(i+1);
        } else {
            i += 1;
        }
    }

    // Remove beacons and sensors
    for beacon in beacons {
        let mut i = 0;
        while i < ranges.len() {
            if ranges[i].contains(&beacon.col) {
                let first = *ranges[i].start()..=beacon.col-1;
                let second = beacon.col+1..=*ranges[i].end();

                ranges[i] = first;
                ranges.insert(i+1, second);
                i += 2;
            } else {
                i += 1;
            }
        }
    }

    ranges.iter().map(|range| range.end() - range.start() + 1).sum()
}

/// tuning_frequency returns the tuning frequency of the distress beacon found between 0..=bound,
/// where a frequency is x * 4000000 + y.
fn tuning_frequency(sensors: &Vec<Sensor>, bound: i64) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_beacon() {
        let sensors = load_sensors("input/day15_sample.txt");
        assert_eq!(26, no_beacon(&sensors, 10));
    }

    #[test]
    fn test_tuning_frequency() {
        let sensors = load_sensors("input/day15_sample.txt");
        assert_eq!(56000011, tuning_frequency(&sensors, 20));
    }
}