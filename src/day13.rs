use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;
use serde_json::Value;
use serde_json::Value::{Array, Number};

#[allow(dead_code)]
pub fn solution() {
    let pairs = parse_pairs("input/day13.txt");

    println!("Day 13");
    println!("Part 1: {}", count_ordered(&pairs));
}

#[derive(Debug, Eq, PartialEq)]
enum Node {
    Array(Vec<Node>),
    Number(i64),
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(value: Value) -> Node {
            match value {
                Array(arr) => Node::Array(arr.into_iter().map(parse).collect_vec()),
                Number(num) => Node::Number(num.as_i64().unwrap()),
                _ => panic!("Unsupported value: {:?}", value),
            }
        }

        // [[1],[2,3,4]]
        let json = serde_json::from_str(s).unwrap();

        Ok(parse(json))
    }
}

struct Pair {
    left: Node,
    right: Node,
}

impl Pair {
    fn ordered(&self) -> bool {
        fn ordered_recursive(left: &Node, right: &Node) -> Option<bool> {
            match (left, right) {
                (Node::Array(l), Node::Array(r)) => {
                    l.into_iter().zip(r.into_iter())
                        .map(|(left_item, right_item)| ordered_recursive(left_item, right_item))
                        .flatten()
                        .nth(0)
                        .or(if l.len() == r.len() {
                            None
                        } else {
                            Some(l.len() < r.len())
                        })
                },
                (Node::Number(l), Node::Array(_)) => {
                    ordered_recursive(&Node::Array(vec![Node::Number(*l)]), right)
                },
                (Node::Array(_), Node::Number(r)) => {
                    ordered_recursive(left, &Node::Array(vec![Node::Number(*r)]))
                },
                (Node::Number(l), Node::Number(r)) => {
                    if l == r {
                        return None
                    }

                    Some(l < r)
                },
            }
        }

        ordered_recursive(&self.left, &self.right).unwrap()
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
            (Some(left), Some(right), _) => pairs.push(Pair {
                left: left.parse().unwrap(),
                right: right.parse().unwrap(),
            }),
            _ => return pairs,
        }
    }
}

fn count_ordered(pairs: &Vec<Pair>) -> usize {
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
    fn test_ordered_sample() {
        let pairs = parse_pairs("input/day13_sample.txt");
        let expected = vec![true, true, false, true, false, true, false, false];

        for (i, (pair, exp)) in pairs.into_iter().zip(expected.into_iter()).enumerate() {
            assert_eq!(exp, pair.ordered(), "Pair {}", i+1);
        }
    }

    #[test]
    fn test_parse() {
        let node: Node = "[[[1,[],0]]]".parse().unwrap();

        let expected = Node::Array(vec![
            Node::Array(vec![
                Node::Array(vec![
                    Node::Number(1), Node::Array(vec![]), Node::Number(0),
                ])
            ])
        ]);

        assert_eq!(expected, node);
    }

    #[test]
    fn test_ordered() {
        let pair = Pair {
            left: "[[[1,[],0]]]".parse().unwrap(),
            right: "[[1,5,[9,5]]]".parse().unwrap(),
        };

        assert_eq!(false, pair.ordered());
    }
}