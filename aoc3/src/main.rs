use regex::Regex;
use std::io::{self, Read};
use std::cmp;
use std::fmt;
use bit_vec::BitVec;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input
        .lines()
        .collect();

    println!("overlap: {} squares", part1(&lines));
    println!("no-overlap: {}", part2(&lines));

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Rect {
    id: usize,
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl Rect {
    fn new(left: usize, right: usize, top: usize, bottom: usize) -> Rect {
        Rect {
            id: 0,
            left,
            right,
            top,
            bottom,
        }
    }

    fn from_str_with_id(s: &str) -> Rect {
        let re = Regex::new(r"^#(\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

        let mut rect = Rect::from_str(s);

        rect.id = id;

        rect
    }

    fn from_str(s: &str) -> Rect {
        let re = Regex::new(r"(\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let captures = re.captures(s).unwrap();

        let x = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let y = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let w = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let h = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();

        Rect::new(
            x,
            x + w - 1,
            y,
            y + h - 1,
        )
    }

    fn intersection(&self, b: &Rect) -> Option<Rect> {
        let left = cmp::max(self.left, b.left);
        let right = cmp::min(self.right, b.right);
        let top = cmp::max(self.top, b.top);
        let bottom = cmp::min(self.bottom, b.bottom);

        if left <= right && top <= bottom {
            Some(Rect::new(left, right, top, bottom))
        } else {
            None
        }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.top {
            for _ in 0..=self.right {
                write!(f, ".")?;
            }
            writeln!(f)?;
        }

        for _ in self.top..=self.bottom {
            for _ in 0..self.left {
                write!(f, ".")?;
            }
            for _ in self.left..=self.right {
                write!(f, "x")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

struct Fabric {
    bv: BitVec,
}

impl Fabric {
    fn new() -> Fabric {
        Fabric {
            bv: BitVec::from_elem(1000 * 1000, false),
        }
    }

    fn mark(&mut self, r: &Rect) {
        for x in r.left..=r.right {
            let row_start = 1000 * x;
            for y in r.top..=r.bottom {
                self.bv.set(row_start + y, true);
            }
        }
    }

    fn count_marks(&self) -> usize {
        let mut count = 0;

        for cell in self.bv.iter() {
            if cell {
                count += 1;
            }
        }

        count
    }
}

fn part1(lines: &[&str]) -> usize {
    let rects: Vec<Rect> = lines.iter()
        .map(|line| Rect::from_str(line))
        .collect();

    let mut fabric = Fabric::new();

    for (a, b) in rects.iter().tuple_combinations() {
        if let Some(intersection) = a.intersection(&b) {
            fabric.mark(&intersection);
        }
    }

    fabric.count_marks()
}

fn part2(lines: &[&str]) -> usize {
    let rects: Vec<Rect> = lines.iter()
        .map(|line| Rect::from_str_with_id(line))
        .collect();

    rects.iter().find(|a| {
        rects.iter().any(|b| a.intersection(b).is_some())
    }).unwrap().id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Rect::from_str(&"#1 @ 1,3: 4x4"),
            Rect::new(1, 4, 3, 6)
        );

        let lines = vec![
            "#1 @ 1,3: 4x4",
            "#2 @ 3,1: 4x4",
            "#3 @ 5,5: 2x2",
        ];

        assert_eq!(
            Rect::from_str(lines[0])
                .intersection(&Rect::from_str(lines[1])),
            Some(Rect::new(3, 4, 3, 4))
        );

        assert_eq!(
            Rect::from_str(lines[0])
                .intersection(&Rect::from_str(lines[2])),
            None
        );

        assert_eq!(
            Rect::from_str(lines[1])
                .intersection(&Rect::from_str(lines[2])),
            None
        );

        assert_eq!(part1(&lines), 4);

        let lines = vec![
            "#1 @ 1,3: 4x4",
            "#2 @ 1,3: 4x4",
        ];
        assert_eq!(
            Rect::from_str(lines[0])
                .intersection(&Rect::from_str(lines[1])),
            Some(Rect::new(1, 4, 3, 6))
        );
        assert_eq!(part1(&lines), 16);
    }

    fn part1_assert(
        a: Rect,
        b: Rect,
        expected: Option<Rect>
    ) {
        println!("a\n{}", a);
        println!("b\n{}", b);
        match &expected {
            Some(expected) => println!("expected\n{}", expected),
            None => println!("expected:{:?}", expected),
        }
        assert_eq!(a.intersection(&b), expected);
    }


    #[test]
    fn part1_intersection_partial_1() {
        part1_assert(
            Rect::new(1, 2, 1, 2),
            Rect::new(2, 3, 2, 3),
            Some(Rect::new(2, 2, 2, 2)));
    }

    #[test]
    fn part1_intersection_contained() {
        part1_assert(
            Rect::new(1, 3, 1, 3),
            Rect::new(2, 2, 2, 2),
            Some(Rect::new(2, 2, 2, 2)));
    }

    #[test]
    fn part2_ids() {
        assert_eq!(Rect::from_str_with_id("#4562 @ 1,1: 4x4").id, 4562);
    }
}
