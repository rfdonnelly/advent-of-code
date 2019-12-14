use std::collections::HashMap;
use std::fs;
use std::io;

use crate::computer::{
    Computer,
    Program,
    State,
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
    let mut state = GameState::new();
    update(&mut state, instructions);

    state.count_blocks()
}

fn part2(program: &Program) -> i64 {
    let mut program = program.clone();
    program[0] = 2;
    let mut computer = Computer::new(program, vec![]);
    let mut state = GameState::new();

    loop {
        let result = computer.run().unwrap();
        let instructions = instructions_from_outputs(&result.outputs);
        update(&mut state, instructions);

        if state.paddle.x == state.ball.x {
            computer.push_input(0);
        } else if state.paddle.x > state.ball.x {
            computer.push_input(-1);
        } else {
            computer.push_input(1);
        }

        if result.state == State::Halt {
            return state.score;
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TileKind {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Tile(Tile),
    Score(i64),
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    p: Point,
    kind: TileKind,
}

type Screen = HashMap<Point, TileKind>;

#[derive(Debug)]
struct GameState {
    screen: Screen,
    score: i64,
    paddle: Point,
    ball: Point,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {x, y}
    }
}

impl From<i64> for TileKind {
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
    fn new(values: &[i64]) -> Self {
        if values[0] == -1 && values[1] == 0 {
            Self::Score(values[2])
        } else {
            Self::Tile(
                Tile {
                    p: Point::new(values[0], values[1]),
                    kind: TileKind::from(values[2]),
                }
            )
        }
    }
}

impl GameState {
    fn new() -> Self {
        Self {
            screen: HashMap::new(),
            score: 0,
            paddle: Point::new(0, 0),
            ball: Point::new(0, 0),
        }
    }

    fn count_blocks(&self) -> usize {
        self.count_tiles(TileKind::Block)
    }

    fn count_tiles(&self, kind: TileKind) -> usize {
        self
            .screen
            .values()
            .filter(|&&value| value == kind)
            .count()
    }
}

fn instructions_from_outputs(outputs: &[i64]) -> impl Iterator<Item=Instruction> + '_ {
    outputs
        .chunks(3)
        .map(|chunk| Instruction::new(chunk))
}

fn update(state: &mut GameState, instructions: impl Iterator<Item=Instruction>) {
    for instruction in instructions {
        match instruction {
            Instruction::Score(score) => {
                state.score = score
            }
            Instruction::Tile(tile) => {
                match tile.kind {
                    TileKind::Empty => {
                        state.screen.remove(&tile.p);
                    }
                    _ => {
                        state.screen.insert(tile.p, tile.kind);
                        match tile.kind {
                            TileKind::Paddle => state.paddle = tile.p,
                            TileKind::Ball => state.ball = tile.p,
                            _ => {}
                        }
                    }
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13() {
        assert_eq!(day13(), (363, 17159))
    }
}
