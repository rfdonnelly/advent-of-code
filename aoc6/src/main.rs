use std::io::{self, Read};
use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn from_str(s: &str) -> Point {
        let tokens: Vec<&str> = s.split(", ").collect();
        let x = tokens.first().unwrap().parse::<i32>().unwrap();
        let y = tokens.last().unwrap().parse::<i32>().unwrap();

        Point::new(x, y)
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

#[derive(Debug, PartialEq)]
struct Grid {
    nearest_neighbors: Vec<Vec<Option<usize>>>,
    pois: Vec<Point>,
    border: Rect,
}

struct MinResult {
    multimatch: bool,
    min: i32,
    min_index: usize,
}

impl MinResult {
    fn new() -> MinResult {
        MinResult {
            multimatch: false,
            min: std::i32::MAX,
            min_index: std::usize::MAX,
        }
    }
}

impl Grid {
    fn new(pois: Vec<Point>) -> Grid {
        let border = border(&pois);
        let nearest_neighbors = Grid::nearest_neighbors(&pois, &border);

        Grid {
            nearest_neighbors,
            pois,
            border,
        }
    }

    fn nearest_neighbors(pois: &[Point], border: &Rect) -> Vec<Vec<Option<usize>>> {
        let mut cells = vec![vec![None; border.bottom as usize]; border.right as usize];

        for x in 0..cells.len() {
            for y in 0..cells[x].len() {
                let cell_position = Point::new(x as i32, y as i32);

                let min = pois
                    .iter()
                    .enumerate()
                    .fold(MinResult::new(), |mut result, (index, poi)| {
                        let distance = cell_position.distance(*poi);

                        if distance < result.min {
                            result.min = distance;
                            result.min_index = index;
                            result.multimatch = false;
                        } else if distance == result.min {
                            result.multimatch = true;
                        }

                        result
                    });

                if !min.multimatch {
                    cells[x][y] = Some(min.min_index);
                }
            }
        }

        cells
    }

    fn corners(&self) -> Vec<Point> {
        vec![
            *self.pois.iter().min_by_key(|poi| (poi.x, poi.y)).unwrap(),
            *self.pois.iter().min_by_key(|poi| (poi.x, -poi.y)).unwrap(),
            *self.pois.iter().min_by_key(|poi| (-poi.x, poi.y)).unwrap(),
            *self.pois.iter().min_by_key(|poi| (-poi.x, -poi.y)).unwrap(),
        ]
    }

    /// Returns area by POI
    fn areas(&self) -> HashMap<Point, u32> {
        let mut areas: HashMap<Point, u32> = HashMap::new();

        for row in self.nearest_neighbors.iter() {
            for cell in row {
                if let Some(poi_index) = cell {
                    let poi = self.pois[*poi_index];

                    areas.entry(poi)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }

        areas
    }

    fn max_area(&self) -> u32 {
        let corners = self.corners();

        self.areas()
            .iter()
            .filter(|(&poi, _)| {
                !corners.contains(&poi)
            })
            .max_by_key(|&(_, area)| area)
            .unwrap()
            .1
            .clone()
    }
}

fn parse_lines(lines: &[&str]) -> Vec<Point> {
    lines
        .iter()
        .map(|line| Point::from_str(line))
        .collect()
}

fn part1(lines: &[&str]) -> u32 {
    let pois = parse_lines(lines);

    Grid::new(pois).max_area()
}

fn part2(lines: &[&str]) -> i32 {
    0
}

fn border(pois: &[Point]) -> Rect {
    Rect {
        left: pois.iter().map(|point| point.x).min().unwrap(),
        right: pois.iter().map(|point| point.x).max().unwrap(),
        top: pois.iter().map(|point| point.y).min().unwrap(),
        bottom: pois.iter().map(|point| point.y).max().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let lines = vec![
            "1, 1",
            "1, 6",
            "8, 3",
            "3, 4",
            "5, 5",
            "8, 9",
        ];

        let pois = parse_lines(&lines);

        let grid = Grid::new(pois.clone());
        let areas = grid.areas();
        let corners = grid.corners();
        let max_area_poi = areas.iter().max_by_key(|kv| kv.1).unwrap();

        println!("grid:{:?}", grid);
        println!("areas:{:?}", areas);
        println!("corners:{:?}", corners);
        println!("max_area_poi:{:?}", max_area_poi);

        assert_eq!(grid.max_area(), 17);

        assert_eq!(super::part1(&lines), 17);
    }
}
