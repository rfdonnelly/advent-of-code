use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

type Number = u64;

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<Number>,
    maps: Vec<Map>,
}

impl Almanac {
    fn map_seed(&self, seed: Number) -> Number {
        self.maps.iter().fold(seed, |value, map| map.map(value))
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, src: Number) -> Number {
        let range = self
            .ranges
            .iter()
            .find(|range| src >= range.src && src < range.src + range.len);
        if let Some(range) = range {
            src - range.src + range.dst
        } else {
            src
        }
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    dst: Number,
    src: Number,
    len: Number,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut paragraphs = s.split("\n\n");
        let first_paragraph = paragraphs.next().unwrap();
        let (_, seeds) = first_paragraph.split_once(": ").unwrap();
        let seeds = seeds
            .split_ascii_whitespace()
            .map(Number::from_str)
            .map(Result::unwrap)
            .collect();
        let maps = paragraphs.map(Map::from_str).map(Result::unwrap).collect();

        Ok(Self { seeds, maps })
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let first_line = lines.next().unwrap();
        let (src_dst, _) = first_line.split_once(" ").unwrap();
        let (src, dst) = src_dst.split_once("-to-").unwrap();

        let ranges = lines.map(Range::from_str).map(Result::unwrap).collect();

        Ok(Self {
            src: src.to_string(),
            dst: dst.to_string(),
            ranges,
        })
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s
            .split_ascii_whitespace()
            .map(Number::from_str)
            .map(Result::unwrap);
        let dst = values.next().unwrap();
        let src = values.next().unwrap();
        let len = values.next().unwrap();

        Ok(Self { dst, src, len })
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
    Almanac::from_str(input).unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &Almanac) -> Number {
    input
        .seeds
        .iter()
        .map(|&seed| input.map_seed(seed))
        .min()
        .unwrap()
}

fn chunk_to_range(chunk: &[Number]) -> std::ops::Range<Number> {
    let &[start, len] = chunk else { unreachable!() };
    let end = start + len;
    start..end
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> Number {
    input
        .seeds
        .chunks(2)
        .flat_map(chunk_to_range)
        .map(|seed| input.map_seed(seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_parse() {
        let input = INPUT.split("\n\n").take(3).collect::<Vec<_>>().join("\n\n");
        let expected = Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                Map {
                    src: "seed".to_string(),
                    dst: "soil".to_string(),
                    ranges: vec![
                        Range {
                            dst: 50,
                            src: 98,
                            len: 2,
                        },
                        Range {
                            dst: 52,
                            src: 50,
                            len: 48,
                        },
                    ],
                },
                Map {
                    src: "soil".to_string(),
                    dst: "fertilizer".to_string(),
                    ranges: vec![
                        Range {
                            dst: 0,
                            src: 15,
                            len: 37,
                        },
                        Range {
                            dst: 37,
                            src: 52,
                            len: 2,
                        },
                        Range {
                            dst: 39,
                            src: 0,
                            len: 15,
                        },
                    ],
                },
            ],
        };

        assert_eq!(parse(&input), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 46);
    }
}
