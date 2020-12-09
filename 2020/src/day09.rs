use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

use crate::lib::{self, Error};

pub fn day(day: usize, input: &str) -> Result<()> {
    let data = lib::parse_input(input)?;
    println!("day{:02}::part1: {}", day, part1(&data, 25)?);
    println!("day{:02}::part2: {}", day, part2(&data)?);

    Ok(())
}

fn part1(values: &[u64], window_size: usize) -> Result<u64> {
    for window in values.windows(window_size + 1) {
        let (sub_window, value) = window.split_at(window_size);
        let value = value[0];
        let is_sum_of_two_values_in_sub_window = sub_window.iter()
            .combinations(2)
            .map(|combination| combination[0] + combination[1])
            .any(|sum| sum == value);

        if !is_sum_of_two_values_in_sub_window { return Ok(value); }
    }

    Err(Error::NoSolution)?
}

fn part2(values: &[u64]) -> Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::lib;

    fn data() -> Vec<u64> {
        let input = indoc!{"
            35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576
        "};

        lib::parse_input(input).unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&data(), 5).unwrap(), 127);
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(super::part2(&data()).unwrap(), 0);
    }
}
