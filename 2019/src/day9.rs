use std::fs;
use std::io;

use crate::computer::{
    Computer,
    Program,
};
use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day9();

    println!("day9::part1: {}", part1);
    println!("day9::part2: {}", part2);

    Ok(())
}

fn day9() -> (i64, i64) {
    let input = fs::read_to_string("input/9").unwrap();

    let program: Program =
        parse_input(&input, Program::from)
        .into_iter()
        .next()
        .unwrap();

    (part1(&program), part2(&program))
}

fn part1(program: &Program) -> i64 {
    *Computer::new(program.clone(), vec![1])
        .run()
        .unwrap()
        .outputs
        .last()
        .unwrap()
}

fn part2(program: &Program) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9() {
        assert_eq!(day9(), (4261108180, 0))
    }
}
