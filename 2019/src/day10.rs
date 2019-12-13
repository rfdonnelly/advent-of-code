use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::io;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day10();

    println!("day10::part1: {}", part1);
    println!("day10::part2: {}", part2);

    Ok(())
}

fn day10() -> (usize, i32) {
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
            let ydiff = (self.y - b.y) as f64;
            Some(f64_to_u32(xdiff.atan2(ydiff)))
        }
    }

    fn distance(&self, b: &Point) -> u32 {
        let xdiff = self.x - b.x;
        let ydiff = b.y - self.y;
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
    let (_point, num_detected) = asteroid_with_best_location(points);

    num_detected
}

fn part2(points: &[Point]) -> i32 {
    let (point, _num_detected) = asteroid_with_best_location(points);
    let point_angles = point_angles(point, points);
    let mut radial_map = radial_map(&point_angles);
    sort_radial_map_by_distance(point, &mut radial_map);
    let flat_map = flatten_radial_map(&mut radial_map);
    let asteroid_200 = flat_map.get(200).unwrap();

    asteroid_200.x * 100 + asteroid_200.y
}

type Radians = u32;
type RadialMap = HashMap<Radians, Vec<Point>>;

fn asteroid_with_best_location(points: &[Point]) -> (&Point, usize) {
    points
        .iter()
        .map(|point| {
            let point_angles = point_angles(point, points);

            let angles: Vec<u32> = point_angles
                .into_iter()
                .map(|(_point, angle)| angle)
                .collect();

            let unique = count_unique(&angles);


            (point, unique)
        })
        .max_by_key(|&(_point, unique)| unique)
        .unwrap()
}

fn point_angles(from: &Point, points: &[Point]) -> Vec<(Point, u32)> {
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
    for (_angle, points) in map {
        points.sort_unstable_by_key(|point| point.distance(from));
    }
}

fn flatten_radial_map(map: &mut RadialMap) -> Vec<Point> {
    let mut vecdeque_map: HashMap<Radians, VecDeque<Point>> = HashMap::new();
    for (k, v) in map.drain() {
        vecdeque_map.insert(k, VecDeque::from(v));
    }

    let mut angles: Vec<Radians> = vecdeque_map.keys().cloned().collect();
    angles.sort();

    let mut flat_map = Vec::new();
    let mut done = false;
    while !done {
        done = true;

        for angle in &angles {
            let points_with_angle = vecdeque_map.get_mut(&angle).unwrap();
            if let Some(next_point_with_angle) = points_with_angle.pop_front() {
                done = false;
                flat_map.push(next_point_with_angle);
            }
        }
    }

    flat_map
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
    fn test_day10_angles_ascending() {
        let above = Point::new(0, 1).angle_to(&Point::new(0, 0));
        let right = Point::new(1, 0).angle_to(&Point::new(0, 0));
        let below = Point::new(0, 0).angle_to(&Point::new(0, 1));
        let left = Point::new(0, 0).angle_to(&Point::new(1, 0));
        let above_slight_left = Point::new(0, 10).angle_to(&Point::new(1, 0));
        assert_eq!((above, right, below, left, above_slight_left), (Some(0), Some(1570), Some(3141), Some(4294965726), Some(4294967197)));
    }
}
