use std::fs;
use std::io;

use itertools::Itertools;

use crate::day5::{
    Computer,
    parse_line,
};
use crate::lib::parse_input;

type Program = Vec<i32>;
type ProgramRef<'a> = &'a[i32];

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day7();

    println!("day7::part1: {}", part1);
    println!("day7::part2: {}", part2);

    Ok(())
}

fn day7() -> (i32, i32) {
    let input = fs::read_to_string("input/7").unwrap();

    let program: Program =
        parse_input(&input, parse_line)
        .into_iter()
        .next()
        .unwrap();

    (part1(&program), part2(&program))
}

fn part1(program: ProgramRef) -> i32 {
    (0..5)
        .into_iter()
        .permutations(5)
        .map(|phases| amplify(program, &phases))
        .max()
        .unwrap()
}

fn part2(program: ProgramRef) -> i32 {
    0
}

fn amplify(
    program: ProgramRef,
    phases: &[i32],
) -> i32 {
    let mut input = 0;
    let mut output = 0;

    for &phase in phases {
        let outputs = Computer::new(program.to_vec(), vec![phase, input]).run().unwrap();
        output = outputs[0];
        input = output;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1_from_str(input: &str) -> i32 {
        let mut program: Program =
            parse_input(&input, parse_line)
            .into_iter()
            .next()
            .unwrap();

        part1(&mut program)
    }

    #[test]
    fn test_part1_example1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let program = parse_line(input);
        assert_eq!(amplify(&program, &[4,3,2,1,0]), 43210);
        assert_eq!(part1_from_str(input), 43210);
    }

    #[test]
    fn test_day7() {
        assert_eq!(day7(), (24625, 0))
    }
}
