mod lib;
mod day01;

use anyhow::Result;

fn main() -> Result<()> {
    let days = [
        day01::day,
    ];

    for (day_idx, day) in days.iter().enumerate() {
        let input = lib::load_input(day_idx + 1)?;
        day(&input)?
    }

    Ok(())
}
