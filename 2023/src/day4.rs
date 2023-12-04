use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Card {
    id: u8,
    winning: u128,
    have: u128,
}

impl Card {
    fn count_winning(&self) -> u8 {
        (self.winning & self.have).count_ones() as u8
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
            .map(Result::unwrap)
            .fold(0_u128, |bitvec, value| bitvec | 1 << value);
        let have = have
            .split_ascii_whitespace()
            .map(u8::from_str)
            .map(Result::unwrap)
            .fold(0_u128, |bitvec, value| bitvec | 1 << value);

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
    input.iter().map(Card::count_winning).map(score).sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> usize {
    input
        .iter()
        .map(Card::count_winning)
        .enumerate()
        .fold(vec![1; input.len()], |mut counts, (i, count)| {
            let start_idx = i + 1;
            let end_idx = start_idx + count as usize;
            (start_idx..end_idx).for_each(|idx| counts[idx] += counts[i]);
            counts
        })
        .iter()
        .sum()
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
                winning: 1u128 << 41 | 1u128 << 48 | 1u128 << 83 | 1u128 << 86 | 1u128 << 17,
                have: 1u128 << 83
                    | 1u128 << 86
                    | 1u128 << 6
                    | 1u128 << 31
                    | 1u128 << 17
                    | 1u128 << 9
                    | 1u128 << 48
                    | 1u128 << 53,
            },
            Card {
                id: 2,
                winning: 1u128 << 13 | 1u128 << 32 | 1u128 << 20 | 1u128 << 16 | 1u128 << 61,
                have: 1u128 << 61
                    | 1u128 << 30
                    | 1u128 << 68
                    | 1u128 << 82
                    | 1u128 << 17
                    | 1u128 << 32
                    | 1u128 << 24
                    | 1u128 << 19,
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
        assert_eq!(part2(&parse(INPUT)), 30);
    }
}
