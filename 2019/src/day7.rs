use std::fs;
use std::io;

use itertools::Itertools;

use crate::computer::{
    Computer,
    Program,
    State,
};
use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day7();

    println!("day7::part1: {}", part1);
    println!("day7::part2: {}", part2);

    Ok(())
}

fn day7() -> (i64, i64) {
    let input = fs::read_to_string("input/7").unwrap();

    let program: Program =
        parse_input(&input, Program::from)
        .into_iter()
        .next()
        .unwrap();

    (part1(&program), part2(&program))
}

fn part1(program: &Program) -> i64 {
    let phase_permutations = (0..5)
        .into_iter()
        .permutations(5);

    phase_permutations
        .map(|phases| amplify(program, &phases))
        .max()
        .unwrap()
}

fn part2(program: &Program) -> i64 {
    let phase_permutations = (5..10)
        .into_iter()
        .permutations(5);

    phase_permutations
        .map(|phases| amplify_with_feedback(program, &phases))
        .max()
        .unwrap()
}

fn amplify(
    program: &Program,
    phases: &[i64],
) -> i64 {
    let mut amplifiers: Vec<Computer> = phases
        .iter()
        .map(|&phase| Computer::new(program.clone(), vec![phase]))
        .collect();

    let mut input = 0;
    let mut output = 0;

    for amplifier in amplifiers.iter_mut() {
        amplifier.push_input(input);
        output = *amplifier.run().unwrap().outputs.first().unwrap();
        input = output;
    }

    output
}

fn amplify_with_feedback(
    program: &Program,
    phases: &[i64],
) -> i64 {
    let mut amplifiers: Vec<Computer> = phases
        .iter()
        .map(|&phase| Computer::new(program.clone(), vec![phase]))
        .collect();

    let mut input = 0;

    loop {
        for (i, amplifier) in amplifiers.iter_mut().enumerate() {
            amplifier.push_input(input);
            let result = amplifier.run().unwrap();
            let output = *result.outputs.first().unwrap();

            if i == 4 {
                if let State::Halt = result.state {
                    return output;
                }
            }
            input = output;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let program = Program::from(input);
        assert_eq!(amplify(&program, &[4,3,2,1,0]), 43210);
        assert_eq!(part1(&Program::from(input)), 43210);
    }

    #[test]
    fn test_part2_example1() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let program = Program::from(input);
        assert_eq!(amplify_with_feedback(&program, &[9,8,7,6,5]), 139629729);
    }

    #[test]
    fn test_day7() {
        assert_eq!(day7(), (24625, 36497698))
    }
}
