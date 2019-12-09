use std::collections::VecDeque;
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

    (part1(program.clone()), part2(program.clone()))
}

pub(crate) fn parse_line(line: &str) -> Vec<i32> {
    line
        .split(",")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

fn part1(program: Vec<i32>) -> i32 {
    *Computer::new(program, vec![1])
        .run()
        .unwrap()
        .last()
        .unwrap()
}

fn part2(program: Vec<i32>) -> i32 {
    *Computer::new(program, vec![5])
        .run()
        .unwrap()
        .last()
        .unwrap()
}

pub(crate) struct Computer {
    /// Instruction pointer
    ip: usize,
    /// Memory
    mem: Vec<i32>,
    inputs: VecDeque<i32>,
}

type Output = Vec<i32>;

enum StepResult {
    Continue(usize, Option<i32>),
    Halt,
}

enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Op {
    fn from_instruction(instruction: i32) -> Self {
        match instruction % 100 {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            _ => panic!("Invalid instruction"),
        }
    }
}

impl Computer {
    pub fn new(mem: Vec<i32>, inputs: Vec<i32>) -> Self {
        Self {
            ip: 0,
            mem,
            inputs: inputs.into(),
        }
    }

    pub fn run(&mut self) -> Result<Output, ()> {
        let mut outputs = Vec::new();

        loop {
            let result = self.step();
            match result {
                Ok(step_result) => {
                    match step_result {
                        StepResult::Continue(ip, output) => {
                            self.ip = ip;
                            if let Some(output) = output {
                                outputs.push(output);
                            }
                        }
                        StepResult::Halt => {
                            return Ok(outputs);
                        }
                    }
                }
                Err(()) => return Err(()),
            }
        }
    }

    fn step(&mut self) -> Result<StepResult, ()> {
        let instruction = self.mem[self.ip];
        let op = Op::from_instruction(instruction);

        match op {
            Op::Add
            | Op::Multiply
            | Op::LessThan
            | Op::Equals => {
                let params = [
                    self.mem[self.ip + 1],
                    self.mem[self.ip + 2],
                ];
                let modes = Self::decode_modes(instruction, params.len());
                let values = self.address_params(&params, &modes);
                let address = self.mem[self.ip + 3] as usize;

                self.mem[address] = match op {
                    Op::Add => values.iter().sum(),
                    Op::Multiply => values.iter().product(),
                    Op::LessThan => (values[0] < values[1]) as i32,
                    Op::Equals => (values[0] == values[1]) as i32,
                    _ => unreachable!(),
                };

                Ok(StepResult::Continue(self.ip + 4, None))
            }
            Op::Input
            | Op::Output => {
                let address = self.mem[self.ip + 1] as usize;

                match op {
                    Op::Input => {
                        self.mem[address] = self.inputs.pop_front().unwrap();
                        Ok(StepResult::Continue(self.ip + 2, None))
                    }
                    Op::Output => {
                        Ok(StepResult::Continue(self.ip + 2, Some(self.mem[address])))
                    }
                    _ => unreachable!(),
                }
            }
            Op::JumpIfTrue
            | Op::JumpIfFalse => {
                let params = [
                    self.mem[self.ip + 1],
                    self.mem[self.ip + 2],
                ];
                let modes = Self::decode_modes(instruction, params.len());
                let values = self.address_params(&params, &modes);

                let jump = match op {
                    Op::JumpIfTrue => values[0] != 0,
                    Op::JumpIfFalse => values[0] == 0,
                    _ => unreachable!(),
                };

                if jump {
                    Ok(StepResult::Continue(values[1] as usize, None))
                } else {
                    Ok(StepResult::Continue(self.ip + 3, None))
                }
            }
            Op::Halt => {
                Ok(StepResult::Halt)
            }
        }
    }

    fn decode_modes(instruction: i32, param_count: usize) -> Vec<Mode> {
        let mut modes = Vec::new();

        for i in 0..param_count {
            let mod_div = 10_i32.pow(i as u32 + 3);
            let div_div = 10_i32.pow(i as u32 + 2);
            modes.push(Mode::from((instruction % mod_div) / div_div));
        }

        modes
    }

    fn address_params(&self, params: &[i32], modes: &[Mode]) -> Vec<i32> {
        params
            .iter()
            .zip(modes.iter())
            .map(|(&param, mode)| {
                match mode {
                    Mode::Position => self.mem[param as usize],
                    Mode::Immediate => param,
                }
            })
            .collect()
    }
}

enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn from(mode: i32) -> Self {
        match mode {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Computer::new(parse_line("1101,100,-5,0,4,0,99"), vec![0]).run().unwrap(), vec![95]);
    }

    #[test]
    fn test_io() {
        assert_eq!(Computer::new(parse_line("3,0,4,0,99"), vec![-34]).run().unwrap(), vec![-34]);
    }

    #[test]
    fn test_day5() {
        assert_eq!(day5(), (7157989, 7873292))
    }
}
