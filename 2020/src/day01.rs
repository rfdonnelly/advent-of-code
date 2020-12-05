use anyhow::Result;
use itertools::Itertools;

use crate::lib::{
    self,
    Day,
    Error
};

pub struct Day01 {
    data: Vec<u32>
}

impl Day for Day01 {
    fn parse(&mut self, input: &str) -> Result<()> {
        self.data = lib::parse_input(input)?;

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(format!("{}", part1(&self.data)?))
    }

    fn part2(&self) -> Result<String> {
        Ok(format!("{}", part2(&self.data)?))
    }
}

impl Day01 {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
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
    use super::*;
    use indoc::indoc;

    fn input() -> Vec<u32> {
        let mut day = Day01::new();
        let input = indoc!{"
            1721
            979
            366
            299
            675
            1456
        "};
        day.parse(input).unwrap();
        day.data
    }

    #[test]
    fn examples() {
        assert_eq!(part1(&input()).unwrap(), 514579);
        assert_eq!(part2(&input()).unwrap(), 241861950);
    }
}
