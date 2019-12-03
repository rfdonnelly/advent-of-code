mod day1;
mod day2;
mod day3;

use std::io;

fn main() -> io::Result<()> {
    day1::main()?;
    day2::main()?;
    day3::main()?;

    Ok(())
}
