use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

type Num = u64;

#[derive(Debug, PartialEq)]
struct Input {
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Input> {
}

#[aoc(day6, part1)]
fn part1(input: &[Input]) -> Num {
    0
}

#[aoc(day6, part2)]
fn part2(input: &[Input]) -> Num {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
    "};

    #[test]
    fn test_parse() {
        let expected = vec![];
        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 1);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 1);
    }
}
