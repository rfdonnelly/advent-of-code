use aoc_runner_derive::{aoc, aoc_generator};

type Input = String;

fn char_to_index(c: u8) -> u8 {
    c - b'A'
}

fn is_valid_marker(chars: &[u8]) -> bool {
    let mut occurences = 0u64;
    for c in chars {
        let index = char_to_index(*c);
        let mask = 1 << index;
        if occurences & mask > 0 {
            return false;
        } else {
            occurences |= mask;
        }
    }
    return true;
}

fn find_start_of_message(datastream: &str, marker_len: usize) -> usize {
    marker_len
        + datastream
            .as_bytes()
            .windows(marker_len)
            .map(is_valid_marker)
            .enumerate()
            .find_map(|(i, valid)| valid.then_some(i))
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

    const INPUTS: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn test_parse() {}

    #[test]
    fn test_p1() {
        let actual: Vec<_> = INPUTS
            .iter()
            .map(|input| p1(&input.0.to_string()))
            .collect();
        let expected: Vec<_> = INPUTS.iter().map(|input| input.1).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_p2() {}
}
