use std::fs;
use std::io;

use itertools::Itertools;

use crate::day5::{
    execute_program,
    parse_line,
};
use crate::lib::parse_input;

type Program = Vec<i32>;
type ProgramRef<'a> = &'a[i32];

pub(crate) fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/7")?;

    let program: Program =
        parse_input(&input, parse_line)
        .into_iter()
        .next()
        .unwrap();

    println!("day7::part1: {}", part1(&program));
    // println!("day7::part2: {}", part2(&mut program.clone()));

    Ok(())
}

fn part1(program: ProgramRef) -> i32 {
    (0..5)
        .into_iter()
        .permutations(5)
        .map(|phases| amplify(program, &phases))
        .max()
        .unwrap()
}

fn amplify(
    program: ProgramRef,
    phases: &[i32],
) -> i32 {
    let mut input = 0;
    let mut output = 0;

    for &phase in phases {
        let mut program = program.to_vec();
        let outputs = execute_program(&mut program, &mut vec![phase, input]).unwrap();
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
        let program: Program = parse_line(input);
        assert_eq!(amplify(&program, &[4,3,2,1,0]), 43210);
        assert_eq!(part1_from_str(input), 43210);
    }
}
