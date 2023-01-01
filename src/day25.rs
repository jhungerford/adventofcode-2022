use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let numbers = load_numbers("input/day25.txt");

    println!("Day 25");
    println!("Part 1: {}", sum_numbers(&numbers));
}

fn load_numbers(filename: &str) -> Vec<String> {
    let f = File::open(filename);
    let f = BufReader::new(f.unwrap());

    f.lines().flatten().collect_vec()
}

fn from_snafu(str: &str) -> i64 {
    let mut num = 0;

    for c in str.chars() {
        num *= 5;
        num += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid char: {}", c)
        }
    }

    num
}

fn to_snafu(num: i64) -> String {
    let mut str = String::new();
    let mut num = num;

    while num != 0 {
        let c = match (num + 2) % 5 {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!(),
        };

        num = (num + 2) / 5;

        str = format!("{}{}", c, str);
    }

    str
}

fn sum_numbers(numbers: &Vec<String>) -> String {
    let sum = numbers.into_iter().map(|snafu| from_snafu(snafu.as_str())).sum();

    to_snafu(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_from_snafu() {
        for (expected_num, expected_snafu) in vec![
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ] {
            let actual_snafu = to_snafu(expected_num);
            let actual_num = from_snafu(&expected_snafu);

            assert_eq!(expected_snafu, &actual_snafu, "snafu - {}, {}", expected_num, expected_snafu);
            assert_eq!(expected_num, actual_num, "num - {}, {}", expected_num, expected_snafu);
        }
    }

    #[test]
    fn test_sum_numbers() {
        let numbers = load_numbers("input/day25_sample.txt");
        assert_eq!("2=-1=0", sum_numbers(&numbers))
    }
}

