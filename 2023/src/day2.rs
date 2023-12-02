use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Set {
    r: u8,
    g: u8,
    b: u8,
}

impl Set {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn lte(&self, rhs: &Set) -> bool {
        self.r <= rhs.r && self.g <= rhs.g && self.b <= rhs.b
    }
}

impl FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = [0; 3];
        for cube in s.split(", ") {
            let (count, color) = cube.split_once(" ").unwrap();
            let count = count.parse().unwrap();
            let index = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => unreachable!(),
            };
            set[index] = count;
        }
        Ok(Set {
            r: set[0],
            g: set[1],
            b: set[2],
        })
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

impl Game {
    fn is_possible(&self, set: &Set) -> bool {
        self.sets.iter().all(|iset| iset.lte(&set))
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, sets) = s.split_once(": ").unwrap();
        let (_, id) = game.split_once(" ").unwrap();
        let id = id.parse().unwrap();
        let sets = sets
            .split("; ")
            .map(Set::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
        Ok(Game { id, sets })
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(Game::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> usize {
    let set = Set::new(12, 13, 14);
    input
        .iter()
        .filter(|game| game.is_possible(&set))
        .map(|game| game.id as usize)
        .sum()
}

// #[aoc(day2, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_parse() {
        let expected = vec![
            Game {
                id: 1,
                sets: vec![Set::new(4, 0, 3), Set::new(1, 2, 6), Set::new(0, 2, 0)],
            },
            Game {
                id: 2,
                sets: vec![Set::new(0, 2, 1), Set::new(1, 3, 4), Set::new(0, 1, 1)],
            },
        ];
        let input = INPUT.lines().take(2).collect::<Vec<_>>().join("\n");
        assert_eq!(parse(&input), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 8);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
