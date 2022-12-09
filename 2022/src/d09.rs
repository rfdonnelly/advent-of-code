use aoc_runner_derive::{aoc, aoc_generator};

use std::cell::Cell;
use std::collections::HashSet;
use std::ops::Add;
use std::ops::AddAssign;

use Direction::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_toward(&self, dest: Point) -> Point {
        let x_diff = dest.x - self.x;
        let y_diff = dest.y - self.y;
        let touching = x_diff.abs() <= 1 && y_diff.abs() <= 1;
        if touching {
            *self
        } else {
            let delta = Point {
                x: x_diff.checked_div(x_diff.abs()).unwrap_or_default(),
                y: y_diff.checked_div(y_diff.abs()).unwrap_or_default(),
            };

            *self + delta
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, 0)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl From<Direction> for Point {
    fn from(d: Direction) -> Self {
        match d {
            Up => Point { x: 0, y: 1 },
            Down => Point { x: 0, y: -1 },
            Left => Point { x: -1, y: 0 },
            Right => Point { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("bad input"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    magnitude: u32,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let (direction, magnitude) = s.split_once(' ').unwrap();
        Self {
            direction: Direction::from(direction),
            magnitude: magnitude.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl State {
    fn new(size: usize) -> Self {
        assert!(size >= 2);
        let initial = Default::default();
        let knots = vec![initial; size];
        let visited = HashSet::from([initial]);
        Self { knots, visited }
    }

    fn head_mut(&mut self) -> &mut Point {
        self.knots.first_mut().unwrap()
    }

    fn tail(&self) -> Point {
        *self.knots.last().unwrap()
    }

    fn next(mut self, instr: &Instruction) -> Self {
        for _ in 0..instr.magnitude {
            // Move the head knot
            *self.head_mut() += instr.direction.into();

            // Move the rest of the knots
            Cell::from_mut(&mut self.knots[..])
                .as_slice_of_cells()
                .windows(2)
                .for_each(|window| {
                    if let [a, b] = window {
                        b.set(b.get().move_toward(a.get()));
                    } else {
                        unreachable!();
                    }
                });

            // Record the tail position
            self.visited.insert(self.tail());
        }
        self
    }

    fn visited(&self) -> usize {
        self.visited.len()
    }
}

type Input = Vec<Instruction>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    input.lines().map(Instruction::from).collect()
}

#[aoc(day9, part1)]
fn p1(input: &Input) -> usize {
    input
        .iter()
        .fold(State::new(2), |state, instr| state.next(instr))
        .visited()
}

#[aoc(day9, part2)]
fn p2(input: &Input) -> usize {
    input
        .iter()
        .fold(State::new(10), |state, instr| state.next(instr))
        .visited()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    const INPUT_LARGER: &str = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    #[test]
    fn test_parse() {}

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 13);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT_LARGER)), 36);
    }
}
