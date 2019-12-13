use std::collections::HashMap;
use std::fs;
use std::io;

use crate::computer::{
    Computer,
    Program,
};
use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day13();

    println!("day13::part1: {}", part1);
    println!("day13::part2: {}", part2);

    Ok(())
}

fn day13() -> (usize, i64) {
    let input = fs::read_to_string("input/13").unwrap();

    let program: Program =
        parse_input(&input, Program::from)
        .into_iter()
        .next()
        .unwrap();

    (part1(&program), part2(&program))
}

fn part1(program: &Program) -> usize {
    let result =
        Computer::new(program.clone(), vec![])
            .run()
            .unwrap();

    let instructions = instructions_from_outputs(&result.outputs);
    let board = board_from_instructions(instructions);

    board
        .values()
        .filter(|&&value| value == Tile::Block)
        .count()
}

fn part2(program: &Program) -> i64 {
    let mut program = program.clone();
    program[0] = 2;

    let result =
        Computer::new(program, vec![])
            .run()
            .unwrap();

    0
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    p: Point,
    tile: Tile,
}

impl From<i64> for Tile {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!(),
        }
    }
}

impl Instruction {
    fn new(x: i64, y: i64, tile: i64) -> Self {
        Instruction {
            p: Point {x, y},
            tile: Tile::from(tile),
        }
    }
}

fn instructions_from_outputs(outputs: &[i64]) -> impl Iterator<Item=Instruction> + '_ {
    outputs
        .chunks(3)
        .map(|chunk| Instruction::new(chunk[0], chunk[1], chunk[2]))
}

fn board_from_instructions(instructions: impl Iterator<Item=Instruction>) -> HashMap<Point, Tile> {
    let mut map = HashMap::new();

    for instruction in instructions {
        match instruction.tile {
            Tile::Empty => map.remove(&instruction.p),
            _ => map.insert(instruction.p, instruction.tile),
        };
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13() {
        assert_eq!(day13(), (363, 0))
    }
}
