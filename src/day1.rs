use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solution() {
    let elves = load_elves("input/day1.txt");

    println!("Part 1: {}", most_food(&elves))
}

/// load_elves parses a list of elves and the calories they are carrying from the given file.
/// Lines list calories, and elves are separated by a blank line.
fn load_elves(filename: &str) -> Vec<Elf> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut elves = Vec::new();
    let mut calories = Vec::new();

    for line_res in f.lines() {
        let line = line_res.unwrap();

        if line == "" {
            elves.push(Elf{ calories });
            calories = Vec::new();
        } else {
            let cals = line.parse::<i32>().unwrap();
            calories.push(cals);
        }
    }

    if calories.len() > 0 {
        elves.push(Elf{ calories });
    }

    return elves
}

struct Elf {
    calories: Vec<i32>
}

impl Elf {
    /// total_calories returns the total number of calories this elf is carrying.
    fn total_calories(&self) -> i32 {
        return self.calories.iter().sum()
    }
}

/// most_food returns the largest number of calories that an elf is carrying.
fn most_food(elves: &Vec<Elf>) -> i32 {
    elves.iter().map(Elf::total_calories).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_food() {
        let elves = load_elves("input/day1_sample.txt");
        assert_eq!(24000, most_food(&elves));
    }
}
