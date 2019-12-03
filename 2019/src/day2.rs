use std::fs;
use std::io;

pub(crate) fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/2")?;

    let program: Vec<u32> = input
        .lines()
        .map(parse_line)
        .next()
        .unwrap();

    println!("day2::part1: {}", part1(&mut program.clone()));
    println!("day2::part2: {}", part2(&mut program.clone()));

    Ok(())
}

enum Next {
    Continue(usize),
    Halt(u32),
}

fn parse_line(line: &str) -> Vec<u32> {
    line
        .split(",")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

fn part1(program: &mut [u32]) -> u32 {
    program[1] = 12;
    program[2] = 2;

    execute_program(program).unwrap()
}

fn part2(program: &mut Vec<u32>) -> u32 {
    for noun in 0..program.len() as u32 {
        for verb in 0..program.len() as u32 {
            program[1] = noun;
            program[2] = verb;

            if let Ok(result) = execute_program(&mut program.clone()) {
                if result == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
    }

    panic!()
}

fn execute_program(program: &mut [u32]) -> Result<u32, String> {
    let mut index = 0;

    loop {
        let result = execute_opcode(index, program);
        match result {
            Ok(Next::Continue(next_index)) => index = next_index,
            Ok(Next::Halt(value)) => return Ok(value),
            Err(message) => return Err(message),
        }
    }
}

fn execute_opcode(index: usize, program: &mut [u32]) -> Result<Next, String> {
    let opcode = program[index];

    match opcode {
        1 => {
            let (a, b, c) = (program[index + 1] as usize, program[index + 2] as usize, program[index + 3] as usize);
            program[c] = program[a] + program[b];
            Ok(Next::Continue(index + 4))
        }
        2 => {
            let (a, b, c) = (program[index + 1] as usize, program[index + 2] as usize, program[index + 3] as usize);
            program[c] = program[a] * program[b];
            Ok(Next::Continue(index + 4))
        }
        99 => Ok(Next::Halt(program[0])),
        _ => Err(format!("Unknown opcode {} at index {}", opcode, index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn execute_program_from_str(input: &str) -> u32 {
        let mut program: Vec<u32> = parse_line(input);
        execute_program(&mut program).unwrap()
    }

    #[test]
    fn test_execute_program() {
        assert_eq!(execute_program_from_str("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
        assert_eq!(execute_program_from_str("1,0,0,0,99"), 2);
        assert_eq!(execute_program_from_str("2,3,0,3,99"), 2);
        assert_eq!(execute_program_from_str("2,4,4,5,99,0"), 2);
        assert_eq!(execute_program_from_str("1,1,1,4,99,5,6,0,99"), 30);
    }
}

