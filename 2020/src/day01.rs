use anyhow::Result;
use itertools::Itertools;

use crate::lib::{
    self,
    Error
};

pub fn day(input: &str) -> Result<()> {
    let data = lib::parse_input(input)?;
    println!("part1:\n{}", part1(&data)?);
    println!("part2:\n{}", part2(&data)?);

    Ok(())
}

fn part1(input: &[u32]) -> Result<u32> {
    for combination in input.into_iter().combinations(2) {
        if combination.iter().map(|x| **x).sum::<u32>() == 2020 {
            return Ok(combination.iter().map(|x| **x).product())
        }
    }

    Err(Error::NoSolution.into())
}

fn part2(input: &[u32]) -> Result<u32> {
    for combination in input.into_iter().combinations(3) {
        if combination.iter().map(|x| **x).sum::<u32>() == 2020 {
            return Ok(combination.iter().map(|x| **x).product())
        }
    }

    Err(Error::NoSolution.into())
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::lib;

    fn data() -> Vec<u32> {
        let input = indoc!{"
            1721
            979
            366
            299
            675
            1456
        "};
        lib::parse_input(input).unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&data()).unwrap(), 514579);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&data()).unwrap(), 241861950);
    }
}
