use thiserror::Error;
use anyhow::{Context, Result};

use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No solution found")]
    NoSolution,
    #[error("Failed to parse input")]
    ParseError,
}

pub fn load_input(day: usize) -> Result<String> {
    let path = format!("{}/inputs/day{:02}", env!("CARGO_MANIFEST_DIR"), day);
    let path = Path::new(&path);
    Ok(fs::read_to_string(path)
       .with_context(|| format!("Failed to open: {}", path.display()))?)
}

pub fn parse_input<'a, T>(input: &'a str) -> Result<Vec<T>>
where
    T: FromStr
{
    input
        .lines()
        .enumerate()
        .map(|(line_idx, line)|
            T::from_str(line).map_err(|_|
                anyhow::Error::new(Error::ParseError).context(
                    format!("Could not parse line {}: '{}'", line_idx + 1, line)
                )
            )
        )
        .collect::<Result<Vec<T>, _>>()
}

pub trait Day {
    fn parse(&mut self, input: &str) -> Result<()>;
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}
