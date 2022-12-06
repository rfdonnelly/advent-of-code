use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Input = String;

fn find_start_of_message(datastream: &str, marker_len: usize) -> usize {
    marker_len + datastream
        .as_bytes()
        .windows(marker_len)
        .map(|seq| HashSet::<u8>::from_iter(seq.iter().copied()).len())
        .enumerate()
        .find_map(|(i, len)| (len == marker_len).then_some(i))
        .unwrap()
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Input {
    input.to_string()
}

#[aoc(day6, part1)]
fn p1(input: &Input) -> usize {
    find_start_of_message(&input, 4)
}

#[aoc(day6, part2)]
fn p2(input: &Input) -> usize {
    find_start_of_message(&input, 14)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUTS: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn test_parse() {
    }

    #[test]
    fn test_p1() {
        let actual: Vec<_> = INPUTS.iter().map(|input| p1(&input.0.to_string())).collect();
        let expected: Vec<_> = INPUTS.iter().map(|input| input.1).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {
    }
}

