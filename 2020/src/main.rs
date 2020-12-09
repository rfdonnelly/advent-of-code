mod lib;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let days = [
        day01::day,
        day02::day,
        day03::day,
        day04::day,
        day05::day,
        day06::day,
        day07::day,
        day08::day,
    ];

    for (day_idx, day) in days.iter().enumerate() {
        let path = lib::input_file_path(day_idx + 1);
        let input = lib::load_input(&path)?;
        day(day_idx + 1, &input)
            .with_context(|| format!("Failed to solve input: '{}'", path.display()))?;
    }

    Ok(())
}
