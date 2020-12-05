use std::str::FromStr;

use anyhow::Result;

use crate::lib;

pub fn day(input: &str) -> Result<()> {
    let data = lib::parse_input(input)?;
    println!("part1:\n{}", part1(&data)?);
    println!("part2:\n{}", part2(&data)?);

    Ok(())
}

fn part1(data: &[Entry]) -> Result<usize> {
    Ok(trees_on_slope(data, 3, 1))
}

fn part2(data: &[Entry]) -> Result<usize> {
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    Ok(
        slopes.iter()
            .map(|(dx, dy)| trees_on_slope(data, *dx, *dy))
            .product()
    )
}

fn trees_on_slope(data: &[Entry], dx: usize, dy: usize) -> usize {
    let width = data[0].map.len();
    let (count, _) = data.iter()
        .enumerate()
        .filter_map(|(i, entry)|
            if i % dy == 0 {
                Some(entry)
            } else {
                None
            }
        )
        .fold((0, 0), |(count, x), entry|
            if entry.map[x % width] {
                (count + 1, x + dx)
            } else {
                (count, x + dx)
            }
        );

    count
}

#[derive(Debug, Eq, PartialEq)]
struct Entry {
    map: Vec<bool>,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.chars().map(|c| c == '#').collect();
        Ok(Entry { map })
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::{lib, Entry};

    fn data() -> Vec<Entry> {
        let input = indoc!{"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "};
        lib::parse_input(input).unwrap()
    }

    #[test]
    fn parse() {
        let input = indoc!{"
            ..##.......
            #...#...#..
        "};

        let actual: Vec<Entry> = lib::parse_input(input).unwrap();
        let expected = vec![
            Entry { map: vec![false, false, true, true, false, false, false, false, false, false, false] },
            Entry { map: vec![true, false, false, false, true, false, false, false, true, false, false] },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&data()).unwrap(), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&data()).unwrap(), 336);
    }
}
