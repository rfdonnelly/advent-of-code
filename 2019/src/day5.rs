use std::fs;
use std::io;

use crate::computer::{
    Computer,
    Program,
};
use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day5();

    println!("day5::part1: {}", part1);
    println!("day5::part2: {}", part2);

    Ok(())
}

fn day5() -> (i32, i32) {
    let input = fs::read_to_string("input/5").unwrap();

    let program: Program =
        parse_input(&input, Program::from)
        .into_iter()
        .next()
        .unwrap();

    (part1(program.clone()), part2(program.clone()))
}

fn part1(program: Program) -> i32 {
    *Computer::new(program, vec![1])
        .run()
        .unwrap()
        .outputs
        .last()
        .unwrap()
}

fn part2(program: Program) -> i32 {
    *Computer::new(program, vec![5])
        .run()
        .unwrap()
        .outputs
        .last()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5() {
        assert_eq!(day5(), (7157989, 7873292))
    }
}
