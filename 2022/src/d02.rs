use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

use Choice::*;
use Outcome::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Round(Choice, char);

impl From<char> for Choice {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => unimplemented!(),
        }
    }
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => unimplemented!(),
        }
    }
}

impl Choice {
    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn loses_against(&self) -> Choice {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn wins_against(&self) -> Choice {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let choice = chars.next().unwrap().into();
        chars.next().unwrap();
        let second = chars.next().unwrap();
        Ok(Self(
                choice,
                second,
        ))
    }
}

impl Round {
    fn outcome_p1(&self) -> u32 {
        match (self.0, self.1.into()) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 6,
            (Rock, Scissors) => 0,
            (Paper, Rock) => 0,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 6,
            (Scissors, Rock) => 6,
            (Scissors, Paper) => 0,
            (Scissors, Scissors) => 3,
        }
    }

    fn score_p1(&self) -> u32 {
        self.outcome_p1() + Choice::from(self.1).score()
    }

    fn outcome_p2(&self) -> u32 {
        match self.1.into() {
            Lose => 0 + self.0.wins_against().score(),
            Draw => 3 + self.0.score(),
            Win => 6 + self.0.loses_against().score(),
        }
    }

    fn score_p2(&self) -> u32 {
        self.outcome_p2()
    }
}

type Input = Vec<Round>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(Round::from_str)
        .map(Result::unwrap)
        .collect()
}

#[aoc(day2, part1)]
fn p1(input: &Input) -> u32 {
    input
        .iter()
        .map(Round::score_p1)
        .sum()
}

#[aoc(day2, part2)]
fn p2(input: &Input) -> u32 {
    input
        .iter()
        .map(Round::score_p2)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
            A Y
            B X
            C Z
        "};

    #[test]
    fn test_parse() {
        let expected = vec![
            Round(Rock, 'Y'),
            Round(Paper, 'X'),
            Round(Scissors, 'Z'),
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
