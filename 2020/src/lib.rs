use thiserror::Error;
use anyhow::{Context, Result};

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No solution found")]
    NoSolution,
    #[error("Failed to parse input")]
    ParseError,
}

pub fn input_file_path(day: usize) -> PathBuf {
    PathBuf::from(format!("{}/inputs/day{:02}", env!("CARGO_MANIFEST_DIR"), day))
}

pub fn load_input(path: &Path) -> Result<String> {
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
