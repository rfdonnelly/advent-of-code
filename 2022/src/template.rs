use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

type Input = Vec<Round>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    Default::default()
}

#[aoc(day2, part1)]
fn p1(input: &Input) -> u32 {
    0
}

#[aoc(day2, part2)]
fn p2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        "};

    #[test]
    fn test_parse() {
        let expected = vec![
        ];
        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 15);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), 12);
    }
}

