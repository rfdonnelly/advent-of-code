use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;
use std::str::FromStr;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::integer::lcm;

type Num = u64;

#[derive(Debug, PartialEq)]
enum Direction {
    L,
    R,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => L,
            'R' => R,
            _ => unreachable!(),
        }
    }
}

use Direction::*;

#[derive(Debug, PartialEq)]
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (directions, nodes) = s.split_once("\n\n").unwrap();

        let directions = directions.chars().map(Direction::from).collect();
        let nodes = nodes
            .lines()
            .map(|line| {
                let node = line[0..3].to_string();
                let l = line[7..10].to_string();
                let r = line[12..15].to_string();
                (node, (l, r))
            })
            .collect();
        Ok(Self { directions, nodes })
    }
}

type ParseOutput = Map;
type SolveInput = Map;

#[aoc_generator(day8)]
fn parse(input: &str) -> ParseOutput {
    input.parse().unwrap()
}

#[aoc(day8, part1)]
fn part1(input: &SolveInput) -> Num {
    let (_, count) = input
        .directions
        .iter()
        .cycle()
        .fold_while(("AAA", 0), |(node, count), direction| {
            let node = input
                .nodes
                .get(node)
                .map(|(l, r)| match direction {
                    L => l,
                    R => r,
                })
                .unwrap();

            if node == "ZZZ" {
                Done((node, count + 1))
            } else {
                Continue((node, count + 1))
            }
        })
        .into_inner();

    count
}

#[aoc(day8, part2)]
fn part2(input: &SolveInput) -> Num {
    let starting_nodes = input
        .nodes
        .keys()
        .filter(|node| &node[2..3] == "A")
        .collect::<Vec<_>>();

    let cycle_counts = starting_nodes
        .iter()
        .map(|&starting_node| {
            let (_, count) = input
                .directions
                .iter()
                .cycle()
                .fold_while((starting_node, 0), |(node, count), direction| {
                    let node = input
                        .nodes
                        .get(node.as_str())
                        .map(|(l, r)| match direction {
                            L => l,
                            R => r,
                        })
                        .unwrap();

                    if &node[2..3] == "Z" {
                        Done((node, count + 1))
                    } else {
                        Continue((node, count + 1))
                    }
                })
                .into_inner();

            count
        })
        .collect::<Vec<_>>();

    cycle_counts
        .into_iter()
        .reduce(|acc, count| lcm(acc, count))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT_P1: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const INPUT_P2: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT_P1)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT_P2)), 6);
    }
}
