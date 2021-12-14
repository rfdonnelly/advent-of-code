use crate::input;

use std::ops::Add;

const DAY: usize = 2;

pub fn run() {
    let input = input(DAY);
    let time = std::time::Instant::now();
    println!("d{:02}p1: {} in {:?}", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    println!("d{:02}p2: {} in {:?}", DAY, p2(&input), time.elapsed());
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
            Direction::Forward => Self {
                x: instruction.magnitude,
                y: 0,
            },
            Direction::Down => Self {
                x: 0,
                y: instruction.magnitude,
            },
            Direction::Up => Self {
                x: 0,
                y: 0 - instruction.magnitude,
            },
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

impl Vector {
    fn checksum(self) -> i32 {
        self.x * self.y
    }
}

fn p1(input: &str) -> i32 {
    input
        .lines()
        .map(Instruction::from)
        .map(Vector::from)
        .reduce(|a, b| a + b)
        .unwrap()
        .checksum()
}

struct State {
    x: i32,
    y: i32,
    aim: i32,
}

impl State {
    fn initial() -> Self {
        Self { x: 0, y: 0, aim: 0 }
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

    fn checksum(self) -> i32 {
        self.x * self.y
    }
}

fn p2(input: &str) -> i32 {
    input
        .lines()
        .map(Instruction::from)
        .fold(State::initial(), |state, instruction| {
            state.next(instruction)
        })
        .checksum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn p1() {
        let input = indoc! {"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "};
        assert_eq!(super::p1(input), 150);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 1690020);
    }

    #[test]
    fn p2() {
        let input = indoc! {"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "};
        assert_eq!(super::p2(input), 900);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 1408487760);
    }
}
