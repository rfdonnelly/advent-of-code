use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Item(char);
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rucksack(Vec<Item>, Vec<Item>);

impl From<char> for Item {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl Item {
    fn priority(&self) -> u32 {
        match self.0 {
            'A'..='Z' => 27 + self.0 as u32 - 'A' as u32,
            'a'..='z' => 1 + self.0 as u32 - 'a' as u32,
            _ => unimplemented!(),
        }
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let midpoint = s.len() / 2;
        let (a, b) = s.split_at(midpoint);
        Self(
            a.chars().map(Item::from).collect(),
            b.chars().map(Item::from).collect(),
        )
    }
}

impl Rucksack {
    fn error(&self) -> Item {
        let a: HashSet<Item> = HashSet::from_iter(self.0.iter().cloned());
        let b: HashSet<Item> = HashSet::from_iter(self.1.iter().cloned());

        *a.intersection(&b).next().unwrap()
    }

    fn hashset(&self) -> HashSet<Item> {
        let iter = self.0.iter().chain(self.1.iter()).cloned();
        HashSet::from_iter(iter)
    }
}

type Input = Vec<Rucksack>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(Rucksack::from)
        .collect()
}

#[aoc(day3, part1)]
fn p1(input: &Input) -> u32 {
    input
        .iter()
        .map(|rucksack| rucksack.error().priority())
        .sum()
}

#[aoc(day3, part2)]
fn p2(input: &Input) -> u32 {
    input
        .chunks_exact(3)
        .map(|group| {
            let sets: Vec<HashSet<Item>> = group.iter().map(Rucksack::hashset).collect();
            let a: HashSet<Item> = HashSet::from_iter(sets[0].intersection(&sets[1]).cloned());
            let badge = a.intersection(&sets[2]).next().unwrap();
            badge.priority()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_parse() {
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 157);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), 70);
    }
}

