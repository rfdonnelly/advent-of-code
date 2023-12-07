use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

type Num = u64;

#[derive(Debug, PartialEq)]
struct Race {
    time: Num,
    dist: Num,
}

impl Race {
    fn winning_combinations(&self) -> Num {
        let t = self.time;
        let d = self.dist;
        // v^2 - tv + d = 0
        // v = (t ± sqrt(t^2 - 4d))/2
        let sqrt_inner = t.pow(2) - 4 * d;
        let sqrt_outer = f64::sqrt(sqrt_inner as f64);
        let v0 = ((t as f64 - sqrt_outer) / 2.0).floor() as u64 + 1;
        let v1 = ((t as f64 + sqrt_outer) / 2.0).ceil() as u64 - 1;
        v1 - v0 + 1
    }
}

#[aoc_generator(day6, part1)]
fn parse_p1(input: &str) -> Vec<Race> {
    let mut values = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .map(Num::from_str)
            .map(Result::unwrap)
    });

    let times = values.next().unwrap();
    let dists = values.next().unwrap();

    times
        .zip(dists)
        .map(|(time, dist)| Race { time, dist })
        .collect()
}

#[aoc_generator(day6, part2)]
fn parse_p2(input: &str) -> Race {
    let mut values = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .flat_map(str::chars)
            .collect::<String>()
            .parse()
            .unwrap()
    });

    let time = values.next().unwrap();
    let dist = values.next().unwrap();

    Race { time, dist }
}

#[aoc(day6, part1)]
fn part1(input: &[Race]) -> Num {
    input.iter().map(Race::winning_combinations).product()
}

#[aoc(day6, part2)]
fn part2(input: &Race) -> Num {
    input.winning_combinations()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    impl Race {
        fn new(time: Num, dist: Num) -> Self {
            Self { time, dist }
        }
    }

    #[test]
    fn test_parse() {
        let expected = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
        assert_eq!(parse_p1(INPUT), expected);

        let expected = Race::new(71530, 940200);
        assert_eq!(parse_p2(INPUT), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(INPUT)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_p2(INPUT)), 71503);
    }
}
