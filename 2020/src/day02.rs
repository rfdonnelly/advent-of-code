use std::str::FromStr;

use anyhow::Result;

use crate::lib::{self, Error};

pub fn day(day: usize, input: &str) -> Result<()> {
    let data = lib::parse_input(input)?;
    println!("day{:02}::part1: {}", day, part1(&data)?);
    println!("day{:02}::part2: {}", day, part2(&data)?);

    Ok(())
}

fn part1(data: &[Entry]) -> Result<usize> {
    let count = data
        .iter()
        .map(|entry| (
            entry,
            entry.password.chars().filter(|c| c == &entry.c).count()
        ))
        .filter(|(entry, count)| count >= &entry.min && count <= &entry.max )
        .count();

    Ok(count)
}

fn part2(data: &[Entry]) -> Result<usize> {
    let count = data
        .iter()
        .map(|entry| (
            entry.password.chars().nth(entry.min - 1).unwrap(),
            entry.password.chars().nth(entry.max - 1).unwrap(),
            entry.c
        ))
        .filter(|(c0, c1, c)| (c == c0) ^ (c == c1))
        .count();

    Ok(count)
}

struct Entry {
    min: usize,
    max: usize,
    c: char,
    password: String,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hyphen_idx = s.find('-').ok_or(anyhow::Error::new(Error::ParseError))?;
        let space_idx = s.find(' ').ok_or(anyhow::Error::new(Error::ParseError))?;
        let colon_idx = s.find(':').ok_or(anyhow::Error::new(Error::ParseError))?;

        Ok(
            Entry {
                min: s[0..hyphen_idx].parse()?,
                max: s[(hyphen_idx+1)..space_idx].parse()?,
                c: s[colon_idx - 1..colon_idx].chars().next().ok_or(anyhow::Error::new(Error::ParseError))?,
                password: s[colon_idx + 2..].into(),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::{lib, Entry};

    fn data() -> Vec<Entry> {
        let input = indoc!{"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "};
        lib::parse_input(input).unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&data()).unwrap(), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&data()).unwrap(), 1);
    }
}
