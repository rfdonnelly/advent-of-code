use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

use std::str::FromStr;

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
    let tens_re = "(one|two|three|four|five|six|seven|eight|nine|[0-9])";
    let ones_re = "(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|[0-9])";

    let regexes = [Regex::new(&ones_re).unwrap(), Regex::new(&tens_re).unwrap()];

    input
        .lines()
        .map(|line| {
            regexes
                .iter()
                .enumerate()
                .map(|(i, re)| {
                    let line = if i == 0 {
                        line.chars().rev().collect::<String>()
                    } else {
                        line.to_string()
                    };

                    let value = re
                        .find(&line)
                        .unwrap()
                        .as_str();

                    Digit::from_str(value).unwrap().0
                })
                .enumerate()
                .map(|(i, digit)| digit * 10_u32.pow(i as u32))
                .sum()
        })
        .collect()
}

struct Digit(u32);

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digit = match s {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            "enin" => 9,
            "thgie" => 8,
            "neves" => 7,
            "xis" => 6,
            "evif" => 5,
            "ruof" => 4,
            "eerht" => 3,
            "owt" => 2,
            "eno" => 1,
            _ => unreachable!(),
        };
        Ok(Self(digit))
    }
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
