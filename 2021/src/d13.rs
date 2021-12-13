use crate::input;

use tap::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

const DAY: usize = 13;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i16,
    y: i16,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut values = s.split(",").map(str::parse::<i16>).map(Result::unwrap);

        Self {
            x: values.next().unwrap(),
            y: values.next().unwrap(),
        }
    }
}

impl Point {
    fn fold(&self, fold: Fold) -> Point {
        match fold {
            Fold::Left(value) => {
                if self.x > value {
                    let diff = self.x - value;
                    let x = value - diff;
                    let y = self.y;
                    Self { x, y }
                } else {
                    *self
                }
            }
            Fold::Up(value) => {
                if self.y > value {
                    let diff = self.y - value;
                    let x = self.x;
                    let y = value - diff;
                    Self { x, y }
                } else {
                    *self
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    Left(i16),
    Up(i16),
}

impl From<&str> for Fold {
    fn from(s: &str) -> Self {
        let (axis, value) = s.split_once("=").unwrap();
        let value = value.parse().unwrap();
        let (_, axis) = axis.rsplit_once(" ").unwrap();

        match axis {
            "x" => Self::Left(value),
            "y" => Self::Up(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let (points, folds) = s.split_once("\n\n").unwrap();
        let points = points.lines().map(Point::from).collect();
        let folds = folds.lines().map(Fold::from).collect();
        Self { points, folds }
    }
}

fn fold_many(points: &[Point], fold: Fold) -> Vec<Point> {
    points
        .iter()
        .map(|point| point.fold(fold))
        .fold(HashSet::new(), |mut set, point| {
            set.insert(point);
            set
        })
        .into_iter()
        .collect()
}

fn p1(input: &str) -> usize {
    let input = Input::from(input);

    let fold = input.folds.first().unwrap();
    fold_many(&input.points, *fold).len()
}

fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 17);

        let input = input(DAY);
        assert_eq!(super::p1(&input), 770);
    }

    #[test]
    #[ignore]
    fn p2() {
        assert_eq!(super::p2(INPUT), 36);

        let input = input(DAY);
        assert_eq!(super::p2(&input), 104834);
    }
}
