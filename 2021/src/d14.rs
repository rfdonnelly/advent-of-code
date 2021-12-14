use crate::input;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

use std::collections::HashMap;

const DAY: usize = 14;

pub fn run() -> String {
    let input = input(DAY);
    let mut output = String::new();
    let time = std::time::Instant::now();
    output += &format!("d{:02}p1: {} in {:?}\n", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    output += &format!("d{:02}p2: {} in {:?}\n", DAY, p2(&input), time.elapsed());
    output
}

type Pair = (char, char);

#[derive(Debug)]
struct Rule {
    pair: Pair,
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
    rules: HashMap<Pair, char>,
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
    polymerize(&input, 10)
}

// Inspired by: https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day14.rs
fn polymerize(input: &Input, steps: usize) -> usize {
    let template_pair_counts = input.template
        .chars()
        .tuple_windows()
        .counts();

    let pair_counts = (0..steps)
        .fold(template_pair_counts, |pair_counts, _| {
            pair_counts
                .iter()
                .fold(pair_counts.clone(), |mut next_pair_counts, (pair, count)| {
                    let insertion = input.rules.get(pair).unwrap();
                    *next_pair_counts.entry((pair.0, *insertion)).or_default() += count;
                    *next_pair_counts.entry((*insertion, pair.1)).or_default() += count;
                    *next_pair_counts.entry(*pair).or_default() -= count;

                    next_pair_counts
                })
        });

    let char_counts = pair_counts
        .iter()
        .fold(HashMap::new(), |mut char_counts, ((a, b), count)| {
            char_counts.entry(*a).or_insert((0, 0)).0 += count;
            char_counts.entry(*b).or_insert((0, 0)).1 += count;

            char_counts
        })
        .values()
        .map(|(l, r)| *l.max(r))
        .collect::<Vec<usize>>();

    if let MinMax(min, max) = char_counts.iter().minmax() {
        (max - min) as usize
    } else {
        unreachable!()
    }
}

fn p2(input: &str) -> usize {
    let input = Input::from(input);
    polymerize(&input, 40)
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
    fn p2() {
        assert_eq!(super::p2(INPUT), 2188189693529);

        let input = input(DAY);
        assert_eq!(super::p2(&input), 4110215602456);
    }
}
