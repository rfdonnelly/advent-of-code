mod lib;
mod day01;

use anyhow::Result;
use lib::Day;

fn main() -> Result<()> {
    let days : &mut [Box<dyn Day>] = &mut [
        Box::new(day01::Day01::new()),
    ];

    for (day_idx, day) in days.iter_mut().enumerate() {
        let input = lib::load_input(day_idx + 1)?;
        day.parse(&input)?;
        println!("day{:02}-part1:\n{}", day_idx + 1, day.part1()?);
        println!("day{:02}-part2:\n{}", day_idx + 1, day.part2()?);
    }

    Ok(())
}
