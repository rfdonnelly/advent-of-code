use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range(u32, u32);

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut iter = s.split("-").map(u32::from_str).map(Result::unwrap);

        Self(iter.next().unwrap(), iter.next().unwrap())
    }
}

impl Range {
    fn inside(&self, rhs: Self) -> bool {
        rhs.0 >= self.0 && rhs.1 <= self.1
    }

    fn overlaps(&self, rhs: Self) -> bool {
        (rhs.0 >= self.0 && rhs.0 <= self.1)
            || (rhs.1 >= self.0 && rhs.1 <= self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair(Range, Range);

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let mut iter = s.split(",").map(Range::from);
        Self(iter.next().unwrap(), iter.next().unwrap())
    }
}

impl Pair {
    fn redundant(&self) -> bool {
        self.1.inside(self.0) || self.0.inside(self.1)
    }

    fn overlap(&self) -> bool {
        self.0.overlaps(self.1) || self.1.overlaps(self.0)
    }
}

type Input = Vec<Pair>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input.lines().map(Pair::from).collect()
}

#[aoc(day4, part1)]
fn p1(input: &Input) -> usize {
    input.iter().filter(|pair| pair.redundant()).count()
}

#[aoc(day4, part2)]
fn p2(input: &Input) -> usize {
    input.iter().filter(|pair| pair.overlap()).count()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_parse() {
        let input = INPUT.lines().take(2).collect::<Vec<&str>>().join("\n");
        let expected = vec![
            Pair(Range(2, 4), Range(6, 8)),
            Pair(Range(2, 3), Range(4, 5)),
        ];
        assert_eq!(parse(&input), expected);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), 4);
    }
}
