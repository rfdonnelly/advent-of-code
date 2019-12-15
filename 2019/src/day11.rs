use std::collections::HashMap;
use std::fs;
use std::io;
use std::ops::AddAssign;

use crate::computer::{
    Computer,
    Program,
    State as RunState,
};
use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day11();

    println!("day11::part1: {}", part1);
    println!("day11::part2:\n{}", part2);

    Ok(())
}

fn day11() -> (usize, String) {
    let input = fs::read_to_string("input/11").unwrap();

    let program: Program =
        parse_input(&input, Program::from)
        .into_iter()
        .next()
        .unwrap();

    (part1(program.clone()), part2(program))
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {x, y}
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, other: Direction) {
        match other {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

impl From<i64> for Turn {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!(),
        }
    }
}

impl AddAssign<Turn> for Direction {
    fn add_assign(&mut self, other: Turn) {
        *self = match self {
            Self::Up => {
                match other {
                    Turn::Left => Self::Left,
                    Turn::Right => Self::Right,
                }
            }
            Self::Left => {
                match other {
                    Turn::Left => Self::Down,
                    Turn::Right => Self::Up,
                }
            }
            Self::Down => {
                match other {
                    Turn::Left => Self::Right,
                    Turn::Right => Self::Left,
                }
            }
            Self::Right => {
                match other {
                    Turn::Left => Self::Up,
                    Turn::Right => Self::Down,
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

impl From<i64> for Color {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!(),
        }
    }
}

impl Into<i64> for Color {
    fn into(self) -> i64 {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

type Map = HashMap<Point, Color>;

struct RobotState {
    direction: Direction,
    location: Point,
    map: Map,
}

impl RobotState {
    fn new() -> Self {
        Self {
            direction: Direction::Up,
            location: Point::new(0, 0),
            map: Map::new(),
        }
    }

    fn update(&mut self, color: Color, turn: Turn) {
        self.map.insert(self.location, color);
        self.direction += turn;
        self.location += self.direction;
    }

    fn current_color(&self) -> Color {
        *self.map
            .get(&self.location)
            .unwrap_or(&Color::Black)
    }
}

fn part1(program: Program) -> usize {
    let mut state = RobotState::new();

    run(program, &mut state);

    state.map.len()
}

fn run(program: Program, state: &mut RobotState) {
    let mut computer = Computer::new(program, vec![]);

    loop {
        computer.push_input(state.current_color().into());
        let result = computer.run().unwrap();

        match result.state {
            RunState::Halt => {
                return;
            }
            RunState::WaitForInput => {
                let color = result.outputs[0];
                let turn = result.outputs[1];
                state.update(color.into(), turn.into());
            }
        }
    }
}

fn part2(program: Program) -> String {
    let mut state = RobotState::new();
    state.map.insert(Point::new(0, 0), Color::White);

    run(program, &mut state);

    let points = white_points(&state.map);
    render(points)
}

fn white_points(map: &Map) -> Vec<Point> {
    map
        .iter()
        .filter_map(|(&point, color)| {
            match color {
                Color::White => Some(point),
                Color::Black => None,
            }
        })
        .collect()
}

fn render(points: Vec<Point>) -> String {
    let (min, max) = corners(&points);
    let translation = Point::new(min.x * -1, min.y * -1);

    let mut points = points;
    let mut max = max;
    translate_point(&mut max, translation);
    translate_points(&mut points, translation);

    flip(&mut points, max.y);

    let width = max.x + 1;
    let height = max.y + 1;
    let mut bitmap: Vec<char> = vec![' '; (width * height) as usize];
    for point in points {
        bitmap[(point.y * width + point.x) as usize] = '#';
    }

    bitmap
        .chunks(width as usize)
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn corners(points: &[Point]) -> (Point, Point) {
    let mut max = Point::new(i64::min_value(), i64::min_value());
    let mut min = Point::new(i64::max_value(), i64::max_value());

    for point in points {
        if point.x < min.x {
            min.x = point.x;
        }
        if point.x > max.x {
            max.x = point.x;
        }
        if point.y < min.y {
            min.y = point.y;
        }
        if point.y > max.y {
            max.y = point.y;
        }
    }

    (min, max)
}

fn translate_points(points: &mut [Point], translation: Point) {
    for point in points.iter_mut() {
        translate_point(point, translation);
    }
}

fn translate_point(point: &mut Point, translation: Point) {
    point.x += translation.x;
    point.y += translation.y;
}

fn flip(points: &mut [Point], max_y: i64) {
    for point in points {
        point.y = max_y - point.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_day11() {
        let part2 = indoc!("
            #  #   ##  ##  #      ## #### #### #  #
            #  #    # #  # #       #    # #    #  #
            ####    # #  # #       #   #  ###  ####
            #  #    # #### #       #  #   #    #  #
            #  # #  # #  # #    #  # #    #    #  #
            #  #  ##  #  # ####  ##  #### #    #  #");
        assert_eq!(day11(), (1785, part2.into()))
    }
}
