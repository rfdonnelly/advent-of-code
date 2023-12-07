use aoc_runner_derive::{aoc, aoc_generator};

use std::cmp::Ordering;
use std::collections::HashMap;

use tap::Tap;

type Num = u64;

#[derive(Debug, PartialEq, Hash, Clone, Copy, Eq)]
struct Card {
    label: char,
    strength: u8,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    fn new(label: char, jokers_wild: bool) -> Self {
        let strength = label.to_digit(10).unwrap_or_else(|| match label {
            'T' => 10,
            'J' => {
                if jokers_wild {
                    1
                } else {
                    11
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        }) as u8;

        Self { label, strength }
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Hand {
    cards: Vec<Card>,
    counts: HashMap<Card, Num>,
    bid: Num,
    jokers_wild: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn n_of_a_kind(&self, n: Num) -> bool {
        if self.jokers_wild {
            self.counts
                .iter()
                .filter(|(&card, _)| card != Card::new('J', self.jokers_wild))
                .any(|(_, count)| *count == n)
        } else {
            self.counts.iter().any(|(_, count)| *count == n)
        }
    }

    fn n_wild(&self) -> Num {
        if self.jokers_wild {
            *self
                .counts
                .get(&Card::new('J', self.jokers_wild))
                .unwrap_or(&0) as Num
        } else {
            0
        }
    }

    fn n_pairs(&self) -> Num {
        if self.jokers_wild {
            self.counts
                .iter()
                .filter(|(&card, &count)| card != Card::new('J', self.jokers_wild) && count == 2)
                .count() as Num
        } else {
            self.counts.iter().count() as Num
        }
    }

    fn type_(&self) -> Type {
        let n_wild = self.n_wild();

        if n_wild >= 4 || self.n_of_a_kind(5 - n_wild) {
            Type::FiveOfAKind
        } else if n_wild == 3 || self.n_of_a_kind(4 - n_wild) {
            Type::FourOfAKind
        } else {
            if n_wild == 2 && self.n_of_a_kind(2) && self.n_of_a_kind(1)
                || n_wild == 1 && self.n_pairs() == 2
                || self.n_of_a_kind(3) && self.n_of_a_kind(2)
            {
                Type::FullHouse
            } else if n_wild == 2 || self.n_of_a_kind(3 - n_wild) {
                Type::ThreeOfAKind
            } else if self.n_pairs() == 2 - n_wild {
                Type::TwoPair
            } else if self.n_pairs() == 1 || n_wild == 1 {
                Type::OnePair
            } else if self.counts.keys().count() == 5 {
                Type::HighCard
            } else {
                Type::None
            }
        }
    }

    fn new(s: &str, jokers_wild: bool) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|c| Card::new(c, jokers_wild))
            .collect::<Vec<_>>();
        let counts = cards
            .iter()
            .copied()
            .fold(HashMap::new(), |mut hash, card| {
                hash.entry(card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                hash
            });
        let bid = bid.parse().unwrap();

        Self {
            cards,
            counts,
            bid,
            jokers_wild,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.type_().cmp(&other.type_()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str, jokers_wild: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| Hand::new(line, jokers_wild))
        .collect()
}

#[aoc_generator(day7, part1)]
fn parse_p1(input: &str) -> Vec<Hand> {
    parse(input, false)
}

#[aoc_generator(day7, part2)]
fn parse_p2(input: &str) -> Vec<Hand> {
    parse(input, true)
}

#[aoc(day7, part1)]
fn part1(input: &[Hand]) -> Num {
    input
        .to_vec()
        .tap_mut(|hands| hands.sort())
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = i + 1;
            hand.bid * rank as Num
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Hand]) -> Num {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(INPUT)), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_p2(INPUT)), 5905);
    }
}
