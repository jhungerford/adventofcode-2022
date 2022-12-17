use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let pairs = parse_pairs("input/day13.txt");

    println!("Day 13");
    println!("Part 1: {}", count_ordered(&pairs)); // 6252 is too high, 6182 is too low.
}

struct Pair {
    left: String,
    right: String,
}

impl Pair {
    fn ordered(&self) -> bool {
        let mut left = self.left.chars().peekable();
        let mut right = self.right.chars().peekable();
        let (mut left_nest, mut right_nest) = (0, 0);

        loop {
            match (left.peek(), right.peek()) {
                // End of packet
                (None, Some(_)) => return true,
                (Some(_), None) => return false,

                // List
                (Some('['), Some('[')) => {
                    left.next();
                    right.next();
                },
                (Some(']'), Some(']')) => {
                    if left_nest > 0 {
                        left_nest -= 1;
                    } else {
                        left.next();
                    }

                    if right_nest > 0 {
                        right_nest -= 1;
                    } else {
                        right.next();
                    }
                },

                // List - left ran out first
                (Some(','), Some(',')) if left_nest > 0 => return true,
                (Some(',') | Some(']'), Some(r)) if r.is_ascii_digit() => return true,
                (Some(']'), Some(',') | Some('[')) => return true,

                // List - right ran out first
                (Some(','), Some(',')) if right_nest > 0 => return false,
                (Some(l), Some(',') | Some(']')) if l.is_ascii_digit() => return false,
                (Some(',') | Some('['), Some(']')) => return false,

                // List - iterating
                (Some(','), Some(',')) => {
                    left.next();
                    right.next();
                },

                // List - digits
                (Some(l), Some(r)) if l.is_ascii_digit() && r.is_ascii_digit() => {
                    let left_num = left.peeking_take_while(|c| c.is_ascii_digit())
                        .collect::<String>().parse::<i32>().unwrap();
                    let right_num = right.peeking_take_while(|c| c.is_ascii_digit())
                        .collect::<String>().parse::<i32>().unwrap();

                    if left_num < right_num {
                        return true;
                    } else if right_num < left_num {
                        return false;
                    }
                },
                (Some('['), Some(r)) if r.is_ascii_digit() => {
                    left.next();
                    right_nest += 1;
                },
                (Some(l), Some('[')) if l.is_ascii_digit() => {
                    left_nest += 1;
                    right.next();
                },

                // Unimplemented case
                (l, r) => {
                    unreachable!("'{}', '{}' - Unhandled case: {:?}, {:?}", self.left, self.right, l, r)
                },
            }
        }
    }
}

fn parse_pairs(filename: &str) -> Vec<Pair> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    let mut lines = f.lines().flatten();
    let mut pairs = Vec::new();

    // Pairs are two lines followed by a newline.
    loop {
        match (lines.next(), lines.next(), lines.next()) {
            (Some(left), Some(right), _) => pairs.push(Pair { left, right }),
            _ => return pairs,
        }
    }
}

fn count_ordered(pairs: &Vec<Pair>) -> usize {
    for pair in pairs {
        if pair.ordered() {
            println!("Ordered: {}\n\t{}\n\t{}", pair.ordered(), pair.left, pair.right);
        }
    }

    pairs.iter().enumerate()
        .filter(|(_, pair)| pair.ordered())
        .map(|(i, _)| i + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ordered() {
        let pairs = parse_pairs("input/day13_sample.txt");

        assert_eq!(13, count_ordered(&pairs));
    }

    #[test]
    fn test_ordered() {
        let pair = Pair {
            left: "[[[1,[],0]]]".to_string(),
            right: "[[1,5,[9,5]]".to_string(),
        };

        assert_eq!(false, pair.ordered(), "{}\n{}", pair.left, pair.right);
    }
}