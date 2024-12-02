use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

type Number = i32;
type Parsed = (Vec<Number>, Vec<Number>);

#[aoc_generator(day1, part1)]
fn parse_p1(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(str::parse::<Number>)
                .map(Result::unwrap);
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip()
}

#[aoc_generator(day1, part2)]
fn parse_p2(input: &str) -> Parsed {
    parse_p1(input)
}

#[aoc(day1, part1)]
fn part1(input: &Parsed) -> Number {
    let mut ls = input.0.clone();
    let mut rs = input.1.clone();

    ls.sort();
    rs.sort();

    ls.iter().zip(rs.iter()).map(|(l, r)| (l - r).abs()).sum()
}

#[aoc(day1, part2)]
fn part2(input: &Parsed) -> Number {
    let ls = input.0.clone();
    let rs = input.1.clone();

    let counts = rs.iter().fold(HashMap::new(), |mut hash, value| {
        hash.entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        hash
    });

    ls.iter()
        .map(|value| value * counts.get(value).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_P1: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    const INPUT_P2: &str = INPUT_P1;

    #[test]
    fn test_parse_p1() {
        let expected = (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(parse_p1(INPUT_P1), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(INPUT_P1)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_p2(INPUT_P2)), 31);
    }
}
