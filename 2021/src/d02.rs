use crate::input;

use std::ops::Add;

pub fn run() {
    let input = input(2);
    println!("d02p1: {}", d02p1(&input));
    println!("d02p2: {}", d02p2(&input));
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => panic!(),
        }
    }
}

struct Instruction {
    direction: Direction,
    magnitude: i32,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut splits = s.split(" ");
        Self {
            direction: Direction::from(splits.next().unwrap()),
            magnitude: splits.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

struct Vector {
    x: i32,
    y: i32,
}

impl From<Instruction> for Vector {
    fn from(instruction: Instruction) -> Self {
        match instruction.direction {
            Direction::Forward => Self { x: instruction.magnitude, y: 0 },
            Direction::Down => Self { x: 0, y: instruction.magnitude },
            Direction::Up => Self { x: 0, y: 0 - instruction.magnitude },
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn d02p1(input: &str) -> i32 {
    let p = input
        .lines()
        .map(Instruction::from)
        .map(Vector::from)
        .reduce(|a, b| a + b)
        .unwrap();

    p.x * p.y
}

struct State {
    x: i32,
    y: i32,
    aim: i32,
}

impl State {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            aim: 0,
        }
    }

    fn next(self, instruction: Instruction) -> Self {
        match instruction.direction {
            Direction::Down => Self {
                x: self.x,
                y: self.y,
                aim: self.aim + instruction.magnitude,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y,
                aim: self.aim - instruction.magnitude,
            },
            Direction::Forward => Self {
                x: self.x + instruction.magnitude,
                y: self.y + self.aim * instruction.magnitude,
                aim: self.aim,
            },
        }
    }
}

fn d02p2(input: &str) -> i32 {
    let p = input
        .lines()
        .map(Instruction::from)
        .fold(State::new(), |state, instruction| state.next(instruction));

    p.x * p.y
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn d02p1() {
        let input = indoc!{"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "};
        assert_eq!(super::d02p1(input), 150);

        let input = super::input(2);
        assert_eq!(super::d02p1(&input), 1690020);
    }

    #[test]
    fn d02p2() {
        let input = indoc!{"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "};
        assert_eq!(super::d02p2(input), 900);

        let input = super::input(2);
        assert_eq!(super::d02p2(&input), 1408487760);
    }
}

