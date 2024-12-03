use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

use itertools::Itertools;

type Number = i32;
type Parsed = Vec<Report>;

#[derive(Clone, Debug, PartialEq)]
struct Report {
    levels: Vec<Number>,
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Ok(Self { levels })
    }
}

impl Report {
    fn is_safe(levels: &[Number]) -> bool {
        let diffs: Vec<_> = levels.iter().tuple_windows().map(|(l, r)| r - l).collect();

        let all_increasing = diffs.iter().all(|&x| x > 0);
        let all_decreasing = diffs.iter().all(|&x| x < 0);
        let all_clamped = diffs.into_iter().map(Number::abs).all(|x| x >= 1 && x <= 3);

        (all_increasing || all_decreasing) && all_clamped
    }

    fn is_safe_p1(&self) -> bool {
        Report::is_safe(&self.levels)
    }

    fn is_safe_p2(&self) -> bool {
        if self.is_safe_p1() {
            true
        } else {
            for exclude_i in 0..self.levels.len() {
                let levels: Vec<_> = self
                    .levels
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != exclude_i)
                    .map(|(_, x)| x.clone())
                    .collect();
                if Report::is_safe(&levels) {
                    return true;
                }
            }
            false
        }
    }
}

#[aoc_generator(day2, part1)]
fn parse_p1(input: &str) -> Parsed {
    input
        .lines()
        .map(Report::from_str)
        .map(Result::unwrap)
        .collect()
}

#[aoc_generator(day2, part2)]
fn parse_p2(input: &str) -> Parsed {
    parse_p1(input)
}

#[aoc(day2, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .into_iter()
        .filter(|report| report.is_safe_p1())
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .into_iter()
        .filter(|report| report.is_safe_p2())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_P1: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    const INPUT_P2: &str = INPUT_P1;

    #[test]
    fn test_parse_p1() {
        let expected = vec![
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ];
        assert_eq!(parse_p1(INPUT_P1), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(INPUT_P1)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_p2(INPUT_P2)), 4);
    }
}
