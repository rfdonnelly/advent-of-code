use thiserror::Error;
use anyhow::{Context, Result};

use std::fs;
use std::path::Path;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No solution found")]
    NoSolution,
}

pub fn load_input(day: usize) -> Result<String> {
    let path = format!("{}/inputs/day{:02}", env!("CARGO_MANIFEST_DIR"), day);
    let path = Path::new(&path);
    Ok(fs::read_to_string(path)
       .with_context(|| format!("Failed to open: {}", path.display()))?)
}

pub trait Day {
    fn parse(&mut self, input: &str) -> Result<()>;
    fn part1(&self) -> Result<String>;
    fn part2(&self) -> Result<String>;
}
