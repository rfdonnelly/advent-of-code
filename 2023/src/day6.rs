use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

type Num = u16;

#[derive(Debug, PartialEq)]
struct Race {
    time: Num,
    dist: Num,
}

impl Race {
    fn new(time: Num, dist: Num) -> Self {
        Self { time, dist }
    }

    fn winning_combinations(&self) -> usize {
        (1..self.time)
            .map(|hold_time| {
                let velocity = hold_time;
                let remaining_time = self.time - hold_time;
                velocity * remaining_time
            })
            .filter(|&dist| dist > self.dist)
            .count()
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Race> {
    let mut values = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(Num::from_str)
            .filter_map(Result::ok)
    });

    let times = values.next().unwrap();
    let dists = values.next().unwrap();

    times
        .zip(dists)
        .map(|(time, dist)| Race { time, dist })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Race]) -> usize {
    dbg!(&input);
    input.iter().map(Race::winning_combinations).product()
}

#[aoc(day6, part2)]
fn part2(input: &[Race]) -> Num {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_parse() {
        let expected = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 46);
    }
}
