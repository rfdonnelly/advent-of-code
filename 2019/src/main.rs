mod day1;
mod day2;

use std::io;

fn main() -> io::Result<()> {
    day1::main()?;
    day2::main()?;

    Ok(())
}
