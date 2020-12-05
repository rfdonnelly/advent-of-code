mod lib;
mod day01;
mod day02;
mod day03;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let days = [
        day01::day,
        day02::day,
        day03::day,
    ];

    for (day_idx, day) in days.iter().enumerate() {
        let path = lib::input_file_path(day_idx + 1);
        let input = lib::load_input(&path)?;
        day(&input)
            .with_context(|| format!("Failed to solve input: '{}'", path.display()))?;
    }

    Ok(())
}
