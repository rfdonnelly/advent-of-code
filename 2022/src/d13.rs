use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use tap::Tap;

use nom::{
    IResult,
    branch::alt,
    error::context,
    multi::{many1, separated_list0},
    combinator::{cut, map, map_res, recognize},
    sequence::{preceded, terminated},
    character::complete::{char, one_of},
};

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        map(parse_list, Item::List),
        map(parse_scalar, Item::Scalar),
    ))(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Item>> {
  context(
    "list",
    preceded(
      char('['),
      cut(terminated(
        separated_list0(char(','), parse_item),
        char(']'),
      )),
    ),
  )(input)
}

fn parse_scalar(input: &str) -> IResult<&str, u32> {
    map_res(
        recognize(
            many1(
                one_of("0123456789")
            )
        ),
        |out: &str| {
            out.parse()
        }
    )(input)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Item {
    Scalar(u32),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Scalar(a), Item::Scalar(b)) => a.cmp(b),
            (Item::Scalar(_), Item::List(_)) => Item::List(vec![self.clone()]).cmp(other),
            (Item::List(_), Item::Scalar(_)) => self.cmp(&Item::List(vec![other.clone()])),
            (Item::List(a), Item::List(b)) => a.cmp(b),
        }
    }
}

type Input = Vec<Item>;

impl From<&str> for Item {
    fn from(s: &str) -> Self {
        parse_item(s).unwrap().1
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Item::from)
        .collect()
}

#[aoc(day13, part1)]
fn p1(packets: &Input) -> usize {
    packets
        .chunks_exact(2)
        .enumerate()
        .filter_map(|(i, pair)| (pair[0] <= pair[1]).then_some(i + 1))
        .sum()
}

#[aoc(day13, part2)]
fn p2(packets: &Input) -> usize {
    let divider_packets = parse(indoc::indoc! {"
        [[2]]
        [[6]]
    "});

    let packets = packets
        .iter()
        .chain(divider_packets.iter())
        .collect::<Vec<&Item>>()
        .tap_mut(|packets| packets.sort());

    divider_packets
        .iter()
        .map(|divider_packet| {
            1 + packets
                .iter()
                .position(|&packet| packet == divider_packet)
                .unwrap()
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    fn test_parse() {
        let expected = vec![
            Item::List(vec![
                Item::Scalar(1),
                Item::Scalar(1),
                Item::Scalar(3),
                Item::Scalar(1),
                Item::Scalar(1),
            ]),
            Item::List(vec![
                Item::Scalar(1),
                Item::Scalar(1),
                Item::Scalar(5),
                Item::Scalar(1),
                Item::Scalar(1),
            ]),
            Item::List(vec![
                Item::List(vec![
                    Item::Scalar(1),
                ]),
                Item::List(vec![
                    Item::Scalar(2),
                    Item::Scalar(3),
                    Item::Scalar(4),
                ]),
            ]),
            Item::List(vec![
                Item::List(vec![
                    Item::Scalar(1),
                ]),
                Item::Scalar(4),
            ]),
        ];
        let actual = parse(INPUT);
        assert_eq!(actual[0..2], expected[0..2]);
        assert_eq!(actual[2..4], expected[2..4]);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 13);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), 140);
    }
}

