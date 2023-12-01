use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use tap::Tap;

#[aoc_generator(day1, part1)]
fn parse_p1(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(char_to_digit);
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .collect()
}

fn char_to_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

#[aoc_generator(day1, part2)]
fn parse_p2(input: &str) -> Vec<u32> {
    let map = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m
    };

    let strs: Vec<String> = map
        .keys()
        .map(|k| k.to_string())
        .chain((0..=9).map(|i| i.to_string()))
        .collect();

    input
        .lines()
        .map(|line| {
            let digits = strs
                .iter()
                .filter_map(|s| line.find(s).map(|i| (s, i)))
                // NOTE: find only returns first occurrence.
                // To account for subsequent occurrences, add rfind results too
                .chain(
                    strs.iter()
                    .filter_map(|s| line.rfind(s).map(|i| (s, i)))
                )
                .collect::<Vec<_>>()
                .tap_mut(|v| v.sort_by_key(|&(_, i)| i));
            let digits = digits.iter().filter_map(|(s, _)| str_to_digit(s, &map)).collect::<Vec<_>>();

            let mut digits = digits.iter();
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .collect()
}

fn str_to_digit(s: &str, map: &HashMap<&str, u32>) -> Option<u32> {
    map.get(s)
        .copied()
        .or_else(|| s.chars().next().unwrap().to_digit(10))
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_P1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const INPUT_P2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_parse_p1() {
        let expected = vec![12, 38, 15, 77];

        assert_eq!(parse_p1(INPUT_P1), expected);
    }

    #[test]
    fn test_parse_p2() {
        let expected = vec![29, 83, 13, 24, 42, 14, 76];

        assert_eq!(parse_p2(INPUT_P2), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(INPUT_P1)), 142);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_p2(INPUT_P2)), 281);
    }
}
