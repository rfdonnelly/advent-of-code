use crate::input;

use itertools::Itertools;
use tap::prelude::*;

use std::collections::HashMap;

const DAY: usize = 14;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug)]
struct Rule {
    pair: (char, char),
    insert: char,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let (pair, insert) = s
            .split_once(" -> ")
            .unwrap();

        let pair = (
            pair.chars().nth(0).unwrap(),
            pair.chars().nth(1).unwrap(),
        );

        let insert = insert.chars().nth(0).unwrap();

        Self { pair, insert }
    }
}

#[derive(Debug)]
struct Input {
    template: String,
    rules: HashMap<(char, char), char>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let (template, rules) = s
            .split_once("\n\n")
            .unwrap();

        let template = template.into();

        let rules = rules
            .lines()
            .map(Rule::from)
            .fold(HashMap::new(), |mut rules, rule| {
                rules.insert(rule.pair, rule.insert);
                rules
            });

        Self { template, rules }
    }
}

fn p1(input: &str) -> usize {
    let input = Input::from(input);

    let polymer = (0..10)
        .fold(input.template, |polymer, _| {
            polymer
                .chars()
                .tuple_windows::<(_, _)>()
                .enumerate()
                .flat_map(|(i, (a, b))| {
                    let insert = input.rules.get(&(a, b)).unwrap();
                    if i == 0 {
                        [Some(a), Some(*insert), Some(b)]
                    } else {
                        [None, Some(*insert), Some(b)]
                    }
                })
                .filter_map(|o| o)
                .collect::<String>()
        });

    let counts = polymer
        .chars()
        .fold(HashMap::new(), |mut counts, c| {
            *counts.entry(c).or_insert(0) += 1;
            counts
        })
        .values()
        .map(|v| *v)
        .collect::<Vec<usize>>()
        .tap_mut(|counts| counts.sort());

    let min_count = counts.first().unwrap();
    let max_count = counts.last().unwrap();

    max_count - min_count
}

fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 1588);

        let input = input(DAY);
        assert_eq!(super::p1(&input), 3143);
    }

    #[test]
    #[ignore]
    fn p2() {
        assert_eq!(super::p2(INPUT), 17);

        let input = input(DAY);
        assert_eq!(super::p2(&input), 770);
    }
}
