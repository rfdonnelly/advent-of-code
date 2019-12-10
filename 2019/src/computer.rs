use std::collections::VecDeque;

pub(crate) struct Computer {
    /// Instruction pointer
    ip: usize,
    /// Relative base
    rb: usize,
    /// Memory
    mem: Vec<i64>,
    inputs: VecDeque<i64>,
}

pub(crate) struct RunResult {
    pub outputs: Vec<i64>,
    pub state: State,
}

#[derive(Debug)]
pub(crate) enum State {
    Halt,
    WaitForInput,
}

enum StepResult {
    Continue(usize, Option<i64>),
    WaitForInput,
    Halt,
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Halt,
}

impl Op {
    fn from_instruction(instruction: i64) -> Self {
        match instruction % 100 {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            9 => Self::AdjustRelativeBase,
            99 => Self::Halt,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Program(Vec<i64>);

impl Program {
    pub(crate) fn from(s: &str) -> Self {
        let program: Vec<i64> = s
            .split(",")
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();

        Self(program)
    }
}

impl Computer {
    pub fn new(program: Program, inputs: Vec<i64>) -> Self {
        Self {
            ip: 0,
            rb: 0,
            mem: program.0,
            inputs: inputs.into(),
        }
    }

    pub fn push_input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }

    pub fn run(&mut self) -> Result<RunResult, ()> {
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
                        StepResult::WaitForInput => {
                            return Ok(RunResult { outputs, state: State::WaitForInput });
                        }
                        StepResult::Halt => {
                            return Ok(RunResult { outputs, state: State::Halt });
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
                    self.mem[self.ip + 3],
                ];
                let modes = Self::decode_modes(instruction, params.len());
                let values = self.address_params(&params, &modes);

                let result = match op {
                    Op::Add => values[0..2].iter().sum(),
                    Op::Multiply => values[0..2].iter().product(),
                    Op::LessThan => (values[0] < values[1]) as i64,
                    Op::Equals => (values[0] == values[1]) as i64,
                    _ => unreachable!(),
                };
                self.write_memory(params[2], modes[2], result);

                Ok(StepResult::Continue(self.ip + 4, None))
            }
            Op::Input
            | Op::Output => {
                let params = [
                    self.mem[self.ip + 1],
                ];
                let modes = Self::decode_modes(instruction, params.len());

                match op {
                    Op::Input => {
                        match self.inputs.pop_front() {
                            Some(input) => {
                                self.write_memory(params[0], modes[0], input);
                                Ok(StepResult::Continue(self.ip + 2, None))
                            }
                            None => {
                                Ok(StepResult::WaitForInput)
                            }
                        }
                    }
                    Op::Output => {
                        let output = self.read_memory(params[0], modes[0]);
                        Ok(StepResult::Continue(self.ip + 2, Some(output)))
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
            Op::AdjustRelativeBase => {
                let params = [
                    self.mem[self.ip + 1]
                ];
                let modes = Self::decode_modes(instruction, params.len());
                let values = self.address_params(&params, &modes);

                self.rb = ((self.rb as i64) + values[0]) as usize;
                Ok(StepResult::Continue(self.ip + 2, None))
            }
            Op::Halt => {
                Ok(StepResult::Halt)
            }
        }
    }

    fn decode_modes(instruction: i64, param_count: usize) -> Vec<Mode> {
        let mut modes = Vec::new();

        for i in 0..param_count {
            let mod_div = 10_i64.pow(i as u32 + 3);
            let div_div = 10_i64.pow(i as u32 + 2);
            modes.push(Mode::from((instruction % mod_div) / div_div));
        }

        modes
    }

    fn write_memory(&mut self, param: i64, mode: Mode, value: i64) {
        let addr = self.address(param, mode);

        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }

        self.mem[addr] = value;
    }

    fn address(&self, param: i64, mode: Mode) -> usize {
        match mode {
            Mode::Position => param as usize,
            Mode::Immediate => panic!(),
            Mode::Relative => ((self.rb as i64) + param) as usize,
        }
    }

    fn read_memory(&self, param: i64, mode: Mode) -> i64 {
        match mode {
            Mode::Position
            | Mode::Relative => {
                let addr = self.address(param, mode);
                *self.mem.get(addr).unwrap_or(&0)
            }
            Mode::Immediate => param,
        }
    }

    fn address_params(&self, params: &[i64], modes: &[Mode]) -> Vec<i64> {
        params
            .iter()
            .zip(modes.iter())
            .map(|(&param, &mode)| self.read_memory(param, mode))
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn from(mode: i64) -> Self {
        match mode {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Computer::new(Program::from("1101,100,-5,0,4,0,99"), vec![0]).run().unwrap().outputs, vec![95]);
    }

    #[test]
    fn test_io() {
        assert_eq!(Computer::new(Program::from("3,0,4,0,99"), vec![-34]).run().unwrap().outputs, vec![-34]);
    }

    #[test]
    fn test_relative_addressing() {
        let input = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        assert_eq!(Computer::new(Program(input.clone()), vec![]).run().unwrap().outputs, input);
    }
}
