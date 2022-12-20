use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;
use regex::Regex;

pub fn solution() {
    let sensors = load_sensors("input/day15.txt");

    println!("Day 15");
    println!("Part 1: {}", no_beacon(&sensors, 2000000));
}

fn load_sensors(filename: &str) -> Vec<Sensor> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten().map(|line| line.parse()).flatten().collect_vec()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn at(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

struct Sensor {
    sensor: Point,
    beacon: Point,
}

impl Sensor {
    /// beacon_dist returns the manhattan distance between the sensor and it's closest beacon.
    fn beacon_dist(&self) -> i64 {
        (self.sensor.x - self.beacon.x).abs() + (self.sensor.y - self.beacon.y).abs()
    }
}

impl FromStr for Sensor {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let captures = re.captures(str).unwrap();

        Ok(Sensor {
            sensor: Point::at(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            beacon: Point::at(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        })
    }
}

fn no_beacon(sensors: &Vec<Sensor>, row: i64) -> usize {
    let mut filled: HashSet<Point> = HashSet::new();

    for sensor in sensors {
        let dist = sensor.beacon_dist();

        if (sensor.sensor.y - row).abs() > dist {
            continue
        }

        // number of squares to the left and right of sensor.x on the line
        let num_squares = dist - (row - sensor.sensor.y).abs();

        for x in -num_squares..=num_squares {
            filled.insert(Point::at(sensor.sensor.x + x, row));
        }
    }

    for sensor in sensors {
        filled.remove(&sensor.sensor);
        filled.remove(&sensor.beacon);
    }

    filled.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_beacon() {
        let sensors = load_sensors("input/day15_sample.txt");
        assert_eq!(26, no_beacon(&sensors, 10));
    }
}