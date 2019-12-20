use std::fs;
use std::io;

use crate::computer::{
    Computer,
    Program,
    State,
};
use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day19();

    println!("day19::part1: {}", part1);
    println!("day19::part2: {}", part2);

    Ok(())
}

fn day19() -> (usize, i64) {
    let input = fs::read_to_string("input/19").unwrap();

    let program: Program = Program::from(&input);
    (part1(program.clone()), part2(program))
}

fn part1(program: Program) -> usize {
    let mut computer = Computer::new(program, Vec::new());
    let mut count = 0;

    for x in 0..50 {
        for y in 0..50 {
            computer.reset();

            computer.push_input(x);
            computer.push_input(y);

            let result = computer
                .run()
                .unwrap();
            let outputs = result
                .outputs;
            println!("{},{} {:?} {:?}", x, y, result.state, outputs);
            let output = outputs
                .first()
                .unwrap();

            count += *output as usize;
        }
    }

    count
}

fn part2(program: Program) -> i64 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_day19() {
        assert_eq!(day19(), (203, 1))
    }
}
