use std::fs::read_to_string;
use itertools::Itertools;

pub fn solution() {
    let data = read_to_string("input/day6.txt").unwrap();

    println!("Part 1: {}", first_marker(data.trim()));
}

fn first_marker(data: &str) -> usize {
    4 + data.as_bytes()
        .windows(4)
        .position(|packet| packet.iter().unique().count() == 4)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_marker() {
        assert_eq!(5, first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, first_marker("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));

    }
}