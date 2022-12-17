use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use itertools::Itertools;
use serde_json::Value;

#[allow(dead_code)]
pub fn solution() {
    let pairs = parse_pairs("input/day13.txt");
    let nodes = parse_nodes("input/day13.txt");

    println!("Day 13");
    println!("Part 1: {}", count_ordered(&pairs));
    println!("Part 2: {}", divider(&nodes));
}

#[derive(Eq, PartialEq, Clone)]
enum Node {
    Array(Vec<Node>),
    Number(i64),
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(value: Value) -> Node {
            match value {
                Value::Array(arr) => Node::Array(arr.into_iter().map(parse).collect_vec()),
                Value::Number(num) => Node::Number(num.as_i64().unwrap()),
                _ => panic!("Unsupported value: {:?}", value),
            }
        }

        // [[1],[2,3,4]]
        let json = serde_json::from_str(s).unwrap();

        Ok(parse(json))
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Array(values) => {
                let result = write!(f, "[");
                if result.is_err() {
                    return result;
                }

                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        let result = write!(f, ",");
                        if result.is_err() {
                            return result;
                        }
                    }

                    let result = value.fmt(f);
                    if result.is_err() {
                        return result;
                    }
                }

                write!(f, "]")
            },
            Node::Number(value) => write!(f, "{}", value)
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        fn is_less(left: &Node, right: &Node) -> Option<bool> {
            match (left, right) {
                (Node::Array(l), Node::Array(r)) => {
                    l.into_iter().zip(r.into_iter())
                        .map(|(left_item, right_item)| is_less(left_item, right_item))
                        .flatten()
                        .nth(0)
                        .or(if l.len() == r.len() {
                            None
                        } else {
                            Some(l.len() < r.len())
                        })
                },
                (Node::Number(l), Node::Array(_)) => {
                    is_less(&Node::Array(vec![Node::Number(*l)]), right)
                },
                (Node::Array(_), Node::Number(r)) => {
                    is_less(left, &Node::Array(vec![Node::Number(*r)]))
                },
                (Node::Number(l), Node::Number(r)) => {
                    if l == r {
                        return None
                    }

                    Some(l < r)
                },
            }
        }

        match is_less(self, other) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Pair {
    left: Node,
    right: Node,
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

fn parse_nodes(filename: &str) -> Vec<Node> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

fn count_ordered(pairs: &Vec<Pair>) -> usize {
    pairs.iter().enumerate()
        .filter(|(_, pair)| pair.left < pair.right)
        .map(|(i, _)| i + 1)
        .sum()
}

/// divider sorts all of the nodes, and inserts packets `[[2]]` and `[[6]]`.  It returns
/// the product of the indexes of the divider packets.
fn divider(nodes: &Vec<Node>) -> usize {
    let sorted = nodes.iter().cloned().sorted().collect_vec();

    let two = "[[2]]".parse::<Node>().unwrap();
    let two_index = sorted.iter().position(|node| two < *node).unwrap() + 1;

    let six = "[[6]]".parse::<Node>().unwrap();
    let six_index = sorted.iter().position(|node| six < *node).unwrap() + 2;

    two_index * six_index
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
            assert_eq!(exp, pair.left < pair.right, "Pair {}", i+1);
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

        assert_eq!(false, pair.left < pair.right);

        let pair = Pair {
            left: "[[]]".parse().unwrap(),
            right: "[[2]]".parse().unwrap(),
        };

        assert_eq!(true, pair.left < pair.right);
    }

    #[test]
    fn test_divider() {
        let nodes = parse_nodes("input/day13_sample.txt");

        assert_eq!(140, divider(&nodes));
    }
}