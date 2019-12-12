use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;

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

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    fn angle_to(&self, b: &Point) -> Option<f64> {
        if self == b {
            None
        } else {
            let ydiff = (self.y - b.y) as f64;
            let xdiff = (self.x - b.x) as f64;
            Some(ydiff.atan2(xdiff))
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn parse_input(s: &str) -> Vec<Point> {
    let mut points = Vec::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                points.push(Point::new(x as i32, y as i32));
            }
        }
    }

    points
}

fn part1(points: &[Point]) -> usize {
    let (point, unique) =
    points
        .iter()
        .map(|point| {
            let angles = angles(point, points);

            // Convert to string so we can compare
            let angles: Vec<String> = angles
                .iter()
                .map(|(point, angle)| format!("{:.3}", angle))
                .collect();

            let unique = count_unique(&angles);

            (point, unique)
        })
        .max_by_key(|&(point, unique)| unique)
        .unwrap();

    // dbg!(point);
    unique
}

fn part2(points: &[Point]) -> usize {
    0
}

fn angles(from: &Point, points: &[Point]) -> Vec<(Point, f64)> {
    points
        .iter()
        .filter_map(|point| from.angle_to(point).map(|angle| (*point, angle)))
        .collect()
}

fn count_unique<T>(entries: &[T]) -> usize
where
    T: Clone + Eq + std::hash::Hash
{
    let set: HashSet<T> = entries.iter().cloned().collect();
    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_part1_example0() {
        let input = indoc!("
            .#..#
            .....
            #####
            ....#
            ...##
        ");

        let points = parse_input(input);
        assert_eq!(part1(&points), 8);
    }

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
    fn test_part1_example2() {
        let input = indoc!("
            #.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.
        ");

        let points = parse_input(input);
        assert_eq!(part1(&points), 35);
    }

    #[test]
    fn test_part1_example3() {
        let input = indoc!("
            .#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..
        ");

        let points = parse_input(input);
        assert_eq!(part1(&points), 41);
    }

    #[test]
    fn test_part1_example4() {
        let input = indoc!("
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
        ");

        let points = parse_input(input);
        assert_eq!(part1(&points), 210);
    }

    #[test]
    fn test_day10() {
        assert_eq!(day10(), (0, 0))
    }
}
