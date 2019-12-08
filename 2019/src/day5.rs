use std::fs;
use std::io;

use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day5();

    println!("day5::part1: {}", part1);
    println!("day5::part2: {}", part2);

    Ok(())
}

fn day5() -> (i32, i32) {
    let input = fs::read_to_string("input/5").unwrap();

    let program: Vec<i32> =
        parse_input(&input, parse_line)
        .into_iter()
        .next()
        .unwrap();

    (part1(&program), part2(&program))
}

pub(crate) fn parse_line(line: &str) -> Vec<i32> {
    line
        .split(",")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

fn part1(program: &mut [i32]) -> i32 {
    *execute_program(program, &mut vec![1])
        .unwrap()
        .last()
        .unwrap()
}

fn part2(program: &mut [i32]) -> i32 {
    *execute_program(program, &mut vec![5])
        .unwrap()
        .last()
        .unwrap()
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

pub(crate) fn execute_program(program: &mut [i32], inputs: &mut Vec<i32>) -> Result<Vec<i32>, String> {
    let mut index = 0;
    let mut outputs = Vec::new();

    // Reverse so we can treat like a FIFO.
    // Could have used a VecDeque instead of a Vec but VecDeque requires a use
    // statement and much refactor.
    inputs.reverse();

    loop {
        let result = execute_opcode(index, program, inputs);
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

fn decode_modes(instruction_code: i32, param_count: u32) -> Result<Vec<Mode>, String> {
    let mut modes = Vec::new();

    for i in 0..param_count {
        let mod_div = 10_i32.pow(i + 3);
        let div_div = 10_i32.pow(i + 2);
        modes.push(Mode::from((instruction_code % mod_div) / div_div)?);
    }

    Ok(modes)
}

fn address_params(params: &[i32], modes: &[Mode], program: &[i32]) -> Vec<i32> {
    params
        .iter()
        .zip(modes.iter())
        .map(|(&param, mode)| {
            match mode {
                Mode::Position => program[param as usize],
                Mode::Immediate => param,
            }
        })
        .collect()
}

fn execute_opcode(index: usize, program: &mut [i32], inputs: &mut Vec<i32>) -> Result<Next, String> {
    let instruction_code = program[index];

    let opcode = instruction_code % 100;

    match opcode {
        1 | 2 | 7 | 8 => {
            let modes = decode_modes(instruction_code, 3)?;

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

            let values = address_params(&params, &modes, program);

            program[params[2] as usize] =
                match opcode {
                    1 => values[0] + values[1],
                    2 => values[0] * values[1],
                    7 => (values[0] < values[1]) as i32,
                    8 => (values[0] == values[1]) as i32,
                    _ => unreachable!(),
                };

            Ok(Next::Continue(Continue::new(index + 4)))
        }
        3 | 4 => {
            let io_index = program[index + 1] as usize;

            match opcode {
                3 => {
                    program[io_index] = inputs.pop().unwrap();
                    Ok(Next::Continue(Continue::new(index + 2)))
                }
                4 => {
                    Ok(Next::Continue(Continue::with_output(index + 2, program[io_index])))
                }
                _ => unreachable!(),
            }
        }
        5 | 6 => {
            let params = [
                program[index + 1],
                program[index + 2],
            ];
            let modes = decode_modes(instruction_code, 2)?;
            let values = address_params(&params, &modes, program);

            let jump =
                match opcode {
                    5 => values[0] != 0,
                    6 => values[0] == 0,
                    _ => unreachable!(),
                };

            if jump {
                Ok(Next::Continue(Continue::new(values[1] as usize)))
            } else {
                Ok(Next::Continue(Continue::new(index + 3)))
            }
        }
        99 => Ok(Next::Halt),
        _ => Err(format!("Unknown opcode {} at index {}", opcode, index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn execute_program_from_str(program: &str, inputs: &mut Vec<i32>) -> Vec<i32> {
        let mut program: Vec<i32> = parse_line(program);
        execute_program(&mut program, inputs).unwrap()
    }

    #[test]
    fn test_add() {
        assert_eq!(execute_program_from_str("1101,100,-5,0,4,0,99", &mut vec![0]), vec![95]);
    }

    #[test]
    fn test_io() {
        assert_eq!(execute_program_from_str("3,0,4,0,99", &mut vec![-34]), vec![-34]);
    }

    #[test]
    fn test_day5() {
        assert_eq!(day5(), (7157989, 7873292))
    }
}
