use std::fs;
use std::io;

pub(crate) fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/5")?;

    let program: Vec<i32> = input
        .lines()
        .map(parse_line)
        .next()
        .unwrap();

    println!("day2::part1: {}", part1(&mut program.clone()));
    // println!("day2::part2: {}", part2(&mut program.clone()));

    Ok(())
}

fn parse_line(line: &str) -> Vec<i32> {
    line
        .split(",")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

fn part1(program: &mut [i32]) -> i32 {
    let outputs = execute_program(program, 1).unwrap();

    dbg!(&outputs);

    outputs[outputs.len() - 1]
}

#[derive(Debug, Copy, Clone)]
struct Continue {
    next_index: usize,
    output: Option<i32>,
}

impl Continue {
    fn new(next_index: usize) -> Self {
        Self {
            next_index,
            output: None
        }
    }

    fn with_output(next_index: usize, output: i32) -> Self {
        Self {
            next_index,
            output: Some(output),
        }
    }
}

enum Next {
    Continue(Continue),
    Halt,
}

fn execute_program(program: &mut [i32], input: i32) -> Result<Vec<i32>, String> {
    let mut index = 0;
    let mut outputs = Vec::new();

    loop {
        let result = execute_opcode(index, program, input);
        match result {
            Ok(Next::Continue(value)) => {
                index = value.next_index;
                if let Some(output) = value.output {
                    outputs.push(output);
                }
            }
            Ok(Next::Halt) => return Ok(outputs),
            Err(message) => return Err(message),
        }
    }
}

enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn from(mode: i32) -> Result<Self, String> {
        match mode {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(format!("Invalid mode {}", mode)),
        }
    }
}

fn execute_opcode(index: usize, program: &mut [i32], input: i32) -> Result<Next, String> {
    let instruction_code = program[index];

    let opcode = instruction_code % 100;

    dbg!(instruction_code);

    match opcode {
        1 | 2 => {
            let modes = [
                Mode::from((instruction_code % 1000) / 100)?,
                Mode::from((instruction_code % 10000) / 1000)?,
                Mode::from((instruction_code % 100000) / 10000)?,
            ];

            if let Mode::Immediate = modes[2] {
                return Err(format!(
                    "Invalid instruction code {} at index {}.  3rd position does not support immediate mode",
                    instruction_code, index));
            }

            let params = [
                program[index + 1],
                program[index + 2],
                program[index + 3],
            ];

            let mut message = format!("instruction_code:{} opcode:{} ", instruction_code, opcode);
            for (&param, mode) in params.iter().zip(modes.iter()) {
                match mode {
                    Mode::Position => {
                        message.push_str(&format!("{}@{} ", program[param as usize], param));
                    }
                    Mode::Immediate => {
                        message.push_str(&format!("{} ", param));
                    }
                }
            }
            println!("{}", message);

            let values: Vec<i32> = params
                .iter()
                .zip(modes.iter())
                .map(|(&param, mode)| {
                    match mode {
                        Mode::Position => program[param as usize],
                        Mode::Immediate => param,
                    }
                })
                .collect();

            program[params[2] as usize] =
                match opcode {
                    1 => values[0] + values[1],
                    2 => values[0] * values[1],
                    _ => unreachable!(),
                };

            Ok(Next::Continue(Continue::new(index + 4)))
        }
        3 | 4 => {
            let io_index = program[index + 1] as usize;

            match opcode {
                3 => {
                    program[io_index] = input;
                    println!("input:{}@{}", program[io_index], io_index);
                    Ok(Next::Continue(Continue::new(index + 2)))
                }
                4 => {
                    println!("output:{}@{}", program[io_index], io_index);
                    Ok(Next::Continue(Continue::with_output(index + 2, program[io_index])))
                }
                _ => unreachable!(),
            }

        }
        99 => Ok(Next::Halt),
        _ => Err(format!("Unknown opcode {} at index {}", opcode, index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn execute_program_from_str(program: &str, input: i32) -> Vec<i32> {
        let mut program: Vec<i32> = parse_line(program);
        execute_program(&mut program, input).unwrap()
    }

    #[test]
    fn test_add() {
        assert_eq!(execute_program_from_str("1101,100,-5,0,4,0,99", 0), vec![95]);
    }

    #[test]
    fn test_io() {
        assert_eq!(execute_program_from_str("3,0,4,0,99", -34), vec![-34]);
    }
}
