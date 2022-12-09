use aoc_runner_derive::{aoc, aoc_generator};

use std::cell::Cell;
use std::collections::HashSet;
use std::fmt;
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

    fn move_toward(&mut self, dest: Point) {
        let x_diff = dest.x - self.x;
        let y_diff = dest.y - self.y;
        let touching = x_diff.abs() <= 1 && y_diff.abs() <= 1;
        if touching {
            return;
        } else {
            if x_diff != 0 { self.x += x_diff / x_diff.abs(); }
            if y_diff != 0 { self.y += y_diff / y_diff.abs(); }
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
        let mut tokens = s.split_ascii_whitespace();
        let direction = Direction::from(tokens.next().unwrap());
        let magnitude = tokens.next().unwrap().parse().unwrap();
        Self { direction, magnitude }
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

    fn head(&self) -> Point {
        *self.knots.first().unwrap()
    }

    fn tail(&self) -> Point {
        *self.knots.last().unwrap()
    }

    fn next(mut self, instr: Instruction) -> Self {
        for _ in 0..instr.magnitude {
            let head = self.knots.first_mut().unwrap();
            *head += instr.direction.into();

            let cell_slice = Cell::from_mut(&mut self.knots[..]);
            let knots = cell_slice.as_slice_of_cells();
            for window in knots.windows(2) {
                if let [a, b] = window {
                    let mut new_b = b.get();
                    new_b.move_toward(a.get());
                    b.replace(new_b);
                }
            }

            self.visited.insert(self.tail());
            // println!("{self}");
        }
        self
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..5).rev() {
            for x in 0..6 {
                let p = Point::new(x, y);
                if p == self.head() {
                    write!(f, "H")?;
                } else if p == self.tail() {
                    write!(f, "T")?;
                } else if p == Default::default() {
                    write!(f, "s")?;
                } else if self.visited.contains(&p) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
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
        .fold(State::new(2), |state, &instr| {
            state.next(instr)
        })
        .visited
        .len()
}

#[aoc(day9, part2)]
fn p2(input: &Input) -> usize {
    input
        .iter()
        .fold(State::new(10), |state, &instr| {
            state.next(instr)
        })
        .visited
        .len()
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
    fn test_parse() {
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 13);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT_LARGER)), 36);
    }
}

