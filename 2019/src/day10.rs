use std::collections::HashSet;
use std::fs;
use std::io;

use indoc::indoc;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day10();

    println!("day10::part1: {}", part1);
    println!("day10::part2: {}", part2);

    Ok(())
}

fn day10() -> (usize, usize) {
    let input = fs::read_to_string("input/10").unwrap();

    let points = parse_input(&input);

    (part1(&points), part2(&points))
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    fn slope_to(&self, b: &Point) -> f64 {
        (self.y - b.y) / (self.x - b.x)
    }
}

fn parse_input(s: &str) -> Vec<Point> {
    let mut points = Vec::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                points.push(Point::new(x as f64, y as f64));
            }
        }
    }

    points
}

fn part1(points: &[Point]) -> usize {
    let slopes = slopes(&Point::new(0.0, 1.0), points);

    let unique = count_unique(&slopes);

    unique
}

fn part2(points: &[Point]) -> usize {
    0
}

fn slopes(from: &Point, points: &[Point]) -> Vec<f64> {
    points
        .iter()
        .map(|point| from.slope_to(point))
        .collect()
}

fn count_unique<T>(entries: &[T]) -> usize
where
    T: Clone
{
    let set: HashSet<T> = entries.iter().cloned().collect();
    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = indoc!("
            ......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####
        ");

        let points = parse_input(input);
        assert_eq!(part1(&points), 33);
    }

    #[test]
    fn test_day10() {
        assert_eq!(day10(), (0, 0))
    }
}
