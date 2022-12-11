use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let (configs, states) = parse_monkeys("input/day11.txt");

    println!("Day 11");
    println!("Part 1: {}", monkey_business(&configs, &states));
}

/// parse_monkeys parses a list of monkeys out of the given file.
/// A monkey looks like the following, and monkeys are separated by a blank line:
/// ```
/// Monkey 0:
///   Starting items: 79, 98
///   Operation: new = old * 19
///   Test: divisible by 23
///     If true: throw to monkey 2
///     If false: throw to monkey 3
/// ```
fn parse_monkeys(filename: &str) -> (Vec<MonkeyConfig>, Vec<MonkeyState>) {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    let mut lines = f.lines().flatten();
    let (mut configs, mut states) = (Vec::new(), Vec::new());

    loop {
        // First line is 'Monkey 0:', or none if we're done parsing
        let maybe_monkey = lines.next();
        if maybe_monkey.is_none() {
            return (configs, states);
        }

        // Next line is 'Starting items: '
        let holding = lines.next().unwrap()
            .replace("  Starting items: ", "")
            .split(", ")
            .map(|item| item.parse::<i32>().unwrap())
            .collect_vec();

        // Next line is the operation.
        let operation = parse_operation(lines.next().unwrap().replace("  Operation: new = ", ""));

        // Next line is the Test
        let test = lines.next().unwrap().replace("  Test: divisible by ", "").parse().unwrap();

        // Next line is the true monkey
        let true_monkey = lines.next().unwrap().replace("    If true: throw to monkey ", "").parse().unwrap();

        // Next line is the false monkey
        let false_monkey = lines.next().unwrap().replace("    If false: throw to monkey ", "").parse().unwrap();

        // Last line for a single monkey is blank
        lines.next();

        configs.push(MonkeyConfig { operation, test, true_monkey, false_monkey });
        states.push(MonkeyState { inspected: 0, holding });
    }
}

fn parse_operation(statement: String) -> Box<dyn Fn(i32) -> i32> {
    // A statement looks like 'old + 8', where the left side is always old, and the right side
    // is either 'old' or a number.  operand can be * or +.
    let parts = statement.split_whitespace().collect_vec();


    match (&parts[0..3], parts[2].parse::<i32>()) {
        (["old", "*", "old"], _) => Box::new(|item| item * item),
        (["old", "+", "old"], _) => Box::new(|item| item + item),
        (["old", "*", _], Ok(num)) => Box::new(move |item| item * num),
        (["old", "+", _], Ok(num)) => Box::new(move |item| item + num),
        _ => panic!("Unsupported operation: {}", statement)
    }
}

struct MonkeyConfig {
    operation: Box<dyn Fn(i32) -> i32>,
    test: i32,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Clone, Debug)]
struct MonkeyState {
    inspected: usize,
    holding: Vec<i32>,
}

/// round runs a full round of monkeys inspecting and throwing items, modifying the list
/// of monkeys in the process.  A monkey looks at all of the items it's holding, increases
/// their worry score by the operation, get bored with the item and divides the score by 3,
/// then tests the item and throws it to another monkey.
fn round(configs: &Vec<MonkeyConfig>, states: &mut Vec<MonkeyState>) {
    for i in 0..configs.len() {
        let config = &configs[i];

        let items = states[i].holding.clone();
        states[i].inspected += &items.len();
        states[i].holding.clear();

        for item in items {
            let new_item = (config.operation)(item) / 3;

            if new_item % config.test == 0 {
                states[config.true_monkey].holding.push(new_item);
            } else {
                states[config.false_monkey].holding.push(new_item);
            };
        }
    }
}

/// monkey_business returns the monkey business score, which is the product of the number of times
/// the two most active monkeys inspected items over 20 rounds.
fn monkey_business(configs: &Vec<MonkeyConfig>, states: &Vec<MonkeyState>) -> i32 {
    let mut states = states.clone();

    for _ in 0..20 {
        round(configs, &mut states)
    }

    states.iter().map(|m| m.inspected as i32).sorted().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_business() {
        let (configs, states) = parse_monkeys("input/day11_sample.txt");

        assert_eq!(10605, monkey_business(&configs, &states));
    }
}