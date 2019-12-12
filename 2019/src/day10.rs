use std::collections::HashMap;
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

    // As u32 so we can compare
    fn angle_to(&self, b: &Point) -> Option<u32> {
        if self == b {
            None
        } else {
            let xdiff = (self.x - b.x) as f64;
            let ydiff = (b.y - self.y) as f64;
            Some(f64_to_u32(ydiff.atan2(xdiff)))
        }
    }

    fn distance(&self, b: &Point) -> u32 {
        let xdiff = (self.x - b.x) as u32;
        let ydiff = (b.y - self.y) as u32;
        ((xdiff.pow(2) + ydiff.pow(2)) as f64).sqrt() as u32
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

            // println!("{} {}", point, angles.iter().map(|(point, angle)| format!("{}:{}", point, angle)).collect::<Vec<String>>().join(" "));

            let angles: Vec<u32> = angles
                .into_iter()
                .map(|(_point, angle)| angle)
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

type Radians = u32;
type RadialMap = HashMap<Radians, Vec<Point>>;

fn angles(from: &Point, points: &[Point]) -> Vec<(Point, u32)> {
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

fn f64_to_u32(value: f64) -> u32 {
    (value * 1000.0) as u32
}

fn radial_map(point_angles: &[(Point, u32)]) -> RadialMap {
    let mut map = HashMap::new();

    for &(point, angle) in point_angles {
        map
            .entry(angle)
            .or_insert(Vec::new())
            .push(point);
    }

    map
}

fn sort_radial_map_by_distance(from: &Point, map: &mut RadialMap) {
    for (angle, points) in map {
        points.sort_unstable_by_key(|point| point.distance(from));
    }
}

fn flatten_radial_map(map: &RadialMap) -> Vec<Point> {
    Vec::new()
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
        assert_eq!(day10(), (292, 0))
    }

    #[test]
    fn test_day10_() {
        let above = Point::new(0, 1).angle_to(&Point::new(0, 0));
        let left = Point::new(0, 0).angle_to(&Point::new(1, 0));
        let below = Point::new(0, 0).angle_to(&Point::new(0, 1));
        let right = Point::new(1, 0).angle_to(&Point::new(0, 0));
        assert_eq!((above, right, below, left), (None, None, None, None));
    }
}
