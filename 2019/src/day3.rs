use std::fs;
use std::io;

pub(crate) fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/3")?;
    let wires = parse_input(&input);

    println!("day3::part1: {}", part1(&wires));
    println!("day3::part2: {}", part2(&wires));

    Ok(())
}

fn parse_input(s: &str) -> Vec<Wire> {
    s
        .lines()
        .map(parse_line)
        .collect()
}

fn part1(wires: &Vec<Wire>) -> i32 {
    let wires: Vec<Vec<Segment>> = wires
        .iter()
        .map(wire_to_segments)
        .collect();

    intersections(&wires[0], &wires[1])
        .iter()
        .map(|(_a, _b, p)| p.manhattan_distance())
        .min()
        .unwrap()
}

fn part2(wires: &Vec<Wire>) -> i32 {
    let wires: Vec<Vec<Segment>> = wires
        .iter()
        .map(wire_to_segments)
        .collect();

    intersections(&wires[0], &wires[1])
        .iter()
        .map(|(a, b, p)| a.steps_to(*p) + b.steps_to(*p))
        .min()
        .unwrap()
}

#[derive(Copy, Clone, Debug)]
enum GridVector {
    X(i32),
    Y(i32),
}

type Wire = Vec<GridVector>;

impl GridVector {
    fn from_str(s: &str) -> Self {
        let direction = s.chars().next().unwrap();
        let magnitude = s[1 as usize ..].parse::<i32>().unwrap();

        match direction {
            'L' => Self::X(-magnitude),
            'R' => Self::X(magnitude),
            'U' => Self::Y(-magnitude),
            'D' => Self::Y(magnitude),
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {x, y}
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    p0: Point,
    p1: Point,
    // The number of steps leading up to but not including this segment
    steps: i32,
}

impl Segment {
    fn from_points(p0: Point, p1: Point, steps: i32) -> Self {
        Segment {p0, p1, steps}
    }

    fn steps_to(&self, p: Point) -> i32 {
        self.steps + (self.p0.x - p.x).abs() + (self.p0.y - p.y).abs()
    }
}

fn wire_to_segments(wire: &Wire) -> Vec<Segment> {
    let mut p1 = Point::new(0, 0);
    let mut segments = Vec::new();
    let mut steps = 0;
    let mut next_steps = 0;

    for vector in wire {
        let p2 =
            match vector {
                GridVector::X(magnitude) => {
                    next_steps += magnitude.abs();
                    Point::new(p1.x + magnitude, p1.y)
                }
                GridVector::Y(magnitude) => {
                    next_steps += magnitude.abs();
                    Point::new(p1.x, p1.y + magnitude)
                }
            };

        segments.push(Segment::from_points(p1, p2, steps));

        p1 = p2;
        steps = next_steps;
    }

    segments
}

fn intersections(a_s: &[Segment], b_s: &[Segment]) -> Vec<(Segment, Segment, Point)> {
    let mut intersections = Vec::new();

    for &a in a_s {
        for &b in b_s {
            if let Some(intersection) = intersection(a, b) {
                if !intersection.is_origin() {
                    intersections.push((a, b, intersection));
                }
            }
        }
    }

    intersections
}

fn intersection(a: Segment, b: Segment) -> Option<Point> {
    if a.p0.x == a.p1.x && b.p0.x == b.p1.x {
        // Parallel in the x dimension
        // Assumes parallel lines do not overlap
        None
    } else if a.p0.y == a.p1.y && b.p0.y == b.p1.y {
        // Parallel in the y dimension
        // Assumes parallel lines do not overlap
        None
    } else {
        // Normalize so that a always runs in the x dimension and b always runs in the y dimension
        let (a, b) =
            if a.p0.x != a.p1.x {
                // Segment a runs in the x dimension, b runs in the y dimension
                (a, b)
            } else {
                // Segment a runs in the y dimension, b runs in the x dimension
                (b, a)
            };

        let c = Point::new(b.p0.x, a.p0.y);

        if ((c.y >= b.p0.y && c.y <= b.p1.y) || (c.y >= b.p1.y && c.y <= b.p0.y)) &&
           ((c.x >= a.p0.x && c.x <= a.p1.x) || (c.x >= a.p1.x && c.x <= a.p0.x))
        {
            Some(c)
        } else {
            None
        }
    }
}

fn parse_line(line: &str) -> Wire {
    line
        .split(",")
        .map(GridVector::from_str)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Segment {
        fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Segment {
            Segment {
                p0: Point {x: x0, y: y0},
                p1: Point {x: x1, y: y1},
                steps: 0,
            }
        }
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            intersection(
                Segment::new(1, 0, 1, 3),
                Segment::new(0, 1, 2, 1),
            ),
            Some(Point::new(1, 1))
        );
    }

    #[test]
    fn test_intersection_parallel_x() {
        assert_eq!(
            intersection(
                Segment::new(2, 0, 2, 3),
                Segment::new(1, 0, 1, 3),
            ),
            None
        );
    }

    #[test]
    fn test_intersection_parallel_y() {
        // Parallel in y dimension
        assert_eq!(
            intersection(
                Segment::new(0, 2, 2, 2),
                Segment::new(0, 1, 2, 1),
            ),
            None
        );
    }

    #[test]
    fn test_intersection_short() {
        assert_eq!(
            intersection(
                Segment::new(1, 0, 1, 3),
                Segment::new(2, 1, 4, 1),
            ),
            None
        );
    }

    #[test]
    fn test_part1_example1() {
        let input =
            "R8,U5,L5,D3\n\
             U7,R6,D4,L4";

        let wires = parse_input(input);
        assert_eq!(part1(&wires), 6);
    }

    #[test]
    fn test_part1_example2() {
        let input =
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
             U62,R66,U55,R34,D71,R55,D58,R83";

        let wires = parse_input(input);
        assert_eq!(part1(&wires), 159);
    }

    #[test]
    fn test_part1_example3() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let wires = parse_input(input);
        assert_eq!(part1(&wires), 135);
    }

    #[test]
    fn test_part2_example1() {
        let input =
            "R8,U5,L5,D3\n\
             U7,R6,D4,L4";

        let wires = parse_input(input);
        assert_eq!(part2(&wires), 30);
    }

    #[test]
    fn test_part2_example2() {
        let input =
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
             U62,R66,U55,R34,D71,R55,D58,R83";

        let wires = parse_input(input);
        assert_eq!(part2(&wires), 610);
    }

    #[test]
    fn test_part2_example3() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let wires = parse_input(input);
        assert_eq!(part2(&wires), 410);
    }
}
