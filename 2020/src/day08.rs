use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Result;

use crate::lib::{self, Error};

pub fn day(day: usize, input: &str) -> Result<()> {
    let data = lib::parse_input(input)?;
    println!("day{:02}::part1: {}", day, part1(&data)?);
    println!("day{:02}::part2: {}", day, part2(&data)?);

    Ok(())
}

fn part1(instrs: &[Instr]) -> Result<i32> {
    match execute(instrs) {
        Execution::Loop(acc) => Ok(acc),
        Execution::Term(_) | Execution::OutOfBounds => Err(Error::NoSolution)?,
    }
}

fn execute(instrs: &[Instr]) -> Execution {
    let mut ip: usize = 0;
    let mut acc: i32 = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        let instr = instrs.get(ip);
        match instr {
            Some(instr) => {
                match visited.get(&ip) {
                    Some(_) => {
                        return Execution::Loop(acc)
                    }
                    None => {
                        visited.insert(ip);
                        match instr {
                            Instr::Nop(_) => ip += 1,
                            Instr::Acc(arg) => {
                                ip += 1;
                                acc += arg;
                            }
                            Instr::Jmp(arg) => {
                                if arg.is_negative() {
                                    ip = match ip.checked_sub(arg.abs() as usize) {
                                        Some(ip) => ip,
                                        None => return Execution::OutOfBounds,
                                    }
                                } else {
                                    ip += *arg as usize;
                                }
                            }
                        }
                    }
                }
            }
            None => {
                return Execution::Term(acc);
            }
        }
    }
}

fn part2(instrs: &[Instr]) -> Result<i32> {
    let mut instrs = instrs.to_vec();

    let mut start_index = 0;

    loop {
        let (relative_index, instr) = &instrs.get(start_index..).unwrap().iter()
            .enumerate()
            .find(|(_, instr)| matches!(instr, Instr::Nop(_) | Instr::Jmp(_)))
            .ok_or(Error::NoSolution)?;
        let index = start_index + relative_index;
        start_index = index + 1;

        let instr_copy = *instr.clone();
        *instrs.get_mut(index).unwrap() = swap(instr);

        match execute(&instrs) {
            Execution::Term(acc) => return Ok(acc),
            Execution::Loop(_) | Execution::OutOfBounds =>
                *instrs.get_mut(index).unwrap() = instr_copy,
        }
    }
}

fn swap(instr: &Instr) -> Instr {
    match instr {
        Instr::Nop(arg) => Instr::Jmp(*arg),
        Instr::Jmp(arg) => Instr::Nop(*arg),
        Instr::Acc(_) => unreachable!(),
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Execution {
    Loop(i32),
    Term(i32),
    OutOfBounds,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Instr {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ");

        let op = iter.next().ok_or(Error::ParseError)?;
        let arg = iter.next().ok_or(Error::ParseError)?.parse().map_err(|_| Error::ParseError)?;

        match op {
            "nop" => Ok(Instr::Nop(arg)),
            "acc" => Ok(Instr::Acc(arg)),
            "jmp" => Ok(Instr::Jmp(arg)),
            _ => Err(Error::ParseError),
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::{lib, Instr};

    #[test]
    fn parse() {
        let input = indoc!{"
            nop +0
            acc +1
            jmp -4
        "};

        let actual: Vec<Instr> = lib::parse_input(input).unwrap();
        let expected = vec![
            Instr::Nop(0),
            Instr::Acc(1),
            Instr::Jmp(-4),
        ];

        assert_eq!(actual, expected);
    }

    fn data() -> Vec<Instr> {
        let input = indoc!{"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "};

        lib::parse_input(input).unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&data()).unwrap(), 5);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&data()).unwrap(), 8);
    }
}
