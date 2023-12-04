use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Card {
    id: u8,
    winning: Vec<u8>,
    have: Vec<u8>,
}

impl Card {
    fn count_winning(&self) -> u8 {
        let winning: HashSet<u8> = HashSet::from_iter(self.winning.iter().copied());
        let have: HashSet<u8> = HashSet::from_iter(self.have.iter().copied());
        winning.intersection(&have).count() as u8
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, numbers) = s.split_once(": ").unwrap();
        let (_, id) = id.split_once(" ").unwrap();
        let id = id.trim().parse().unwrap();

        let (winning, have) = numbers.split_once(" | ").unwrap();
        let winning = winning
            .split_ascii_whitespace()
            .map(u8::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
        let have = have
            .split_ascii_whitespace()
            .map(u8::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        Ok(Self { id, winning, have })
    }
}

fn score(count: u8) -> u32 {
    if count == 0 {
        0
    } else {
        2_u32.pow((count - 1).into())
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(Card::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day4, part1)]
fn part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(Card::count_winning)
        .map(score)
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_parse() {
        let input = INPUT.lines().take(2).collect::<Vec<_>>().join("\n");
        let expected = vec![
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                have: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                id: 2,
                winning: vec![13, 32, 20, 16, 61],
                have: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
        ];
        assert_eq!(parse(&input), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 467835);
    }
}
