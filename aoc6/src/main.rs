use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input
	.lines()
        .collect();

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance(&self, b: Point) -> i32 {
        (b.x - self.x).abs() + (b.y - self.y).abs()
    }
}

#[derive(Debug, PartialEq)]
struct Rect {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

fn part1(lines: &[&str]) -> i32 {
}

fn part2(lines: &[&str]) -> i32 {
}

fn extremes(points: &[Point]) -> Rect {
    Rect {
        left: points.map(|point| point.x).min().unwrap(),
        right: points.map(|point| point.x).max().unwrap(),
        top: points.map(|point| point.y).min().unwrap(),
        bottom: points.map(|point| point.y).max().unwrap(),
    }
}

#[cfg(test)]
module tests {
    use super::*;

    #[test]
    fn part1() {
        let points = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];
    }
}
