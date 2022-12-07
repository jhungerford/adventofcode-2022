use std::fs::read_to_string;
use itertools::Itertools;

static PACKET: usize = 4;
static MESSAGE: usize = 14;

#[allow(dead_code)]
pub fn solution() {
    let data = read_to_string("input/day6.txt").unwrap();

    println!("Part 1: {}", marker_idx(data.trim(), PACKET));
    println!("Part 2: {}", marker_idx(data.trim(), MESSAGE));
}

fn marker_idx(data: &str, len: usize) -> usize {
    len + data.as_bytes()
        .windows(len)
        .position(|packet| packet.iter().unique().count() == len)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_idx_packet() {
        assert_eq!(7, marker_idx("mjqjpqmgbljsphdztnvjfqwrcgsmlb", PACKET));
        assert_eq!(5, marker_idx("bvwbjplbgvbhsrlpgdmjqwftvncz", PACKET));
        assert_eq!(6, marker_idx("nppdvjthqldpwncqszvftbrmjlhg", PACKET));
        assert_eq!(10, marker_idx("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", PACKET));
        assert_eq!(11, marker_idx("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", PACKET));
    }

    #[test]
    fn test_marker_idx_message() {
        assert_eq!(19, marker_idx("mjqjpqmgbljsphdztnvjfqwrcgsmlb", MESSAGE));
        assert_eq!(23, marker_idx("bvwbjplbgvbhsrlpgdmjqwftvncz", MESSAGE));
        assert_eq!(23, marker_idx("nppdvjthqldpwncqszvftbrmjlhg", MESSAGE));
        assert_eq!(29, marker_idx("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", MESSAGE));
        assert_eq!(26, marker_idx("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", MESSAGE));

    }
}