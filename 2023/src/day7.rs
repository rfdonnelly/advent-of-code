use aoc_runner_derive::{aoc, aoc_generator};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

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
    fn new(label: char) -> Self {
        let strength = label.to_digit(10).unwrap_or_else(|| match label {
            'T' => 10,
            'J' => 1,
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
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards.chars().map(Card::new).collect::<Vec<_>>();
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

        Ok(Self { cards, counts, bid })
    }
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
        self.counts
            .iter()
            .filter(|(&card, _)| card != Card::new('J'))
            .any(|(_, count)| *count == n)
    }

    fn n_jokers(&self) -> Num {
        *self.counts.get(&Card::new('J')).unwrap_or(&0) as Num
    }

    fn n_pairs(&self) -> Num {
        self.counts
            .iter()
            .filter(|(&card, &count)| card != Card::new('J') && count == 2)
            .count() as Num
    }

    fn type_(&self) -> Type {
        let n_jokers = self.n_jokers();

        if n_jokers >= 4 || self.n_of_a_kind(5 - n_jokers) {
            Type::FiveOfAKind
        } else if n_jokers == 3 || self.n_of_a_kind(4 - n_jokers) {
            // 3 jokers
            // 2 jokers + 2oaK
            // 1 joker + 3oaK
            Type::FourOfAKind
        } else {
            if n_jokers == 2 && self.n_of_a_kind(2) && self.n_of_a_kind(1)
                || n_jokers == 1 && self.n_pairs() == 2
                || self.n_of_a_kind(3) && self.n_of_a_kind(2)
            {
                Type::FullHouse
            } else if n_jokers == 2 || self.n_of_a_kind(3 - n_jokers) {
                Type::ThreeOfAKind
            } else if self.n_pairs() == 2 - n_jokers {
                Type::TwoPair
            } else if self.n_pairs() == 1 || n_jokers == 1 {
                Type::OnePair
            } else if self.counts.keys().count() == 5 {
                Type::HighCard
            } else {
                Type::None
            }
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

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(Hand::from_str)
        .map(Result::unwrap)
        .collect()
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
        assert_eq!(part1(&parse(INPUT)), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 71503);
    }
}
