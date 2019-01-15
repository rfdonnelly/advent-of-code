use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

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

type NearestNeighbors = Vec<Option<usize>>;

#[derive(Debug, PartialEq)]
struct Grid {
    x_len: usize,
    y_len: usize,
    nearest_neighbors: NearestNeighbors,
    pois: Vec<Point>,
    border: Rect,
}

impl Grid {
    fn new(pois: Vec<Point>) -> Grid {
        let border = Grid::border(&pois);
        let x_len = border.right as usize + 1;
        let y_len = border.bottom as usize + 1;
        let nearest_neighbors = Grid::nearest_neighbors(&pois, x_len, y_len);

        Grid {
            x_len,
            y_len,
            nearest_neighbors,
            pois,
            border,
        }
    }

    fn border(pois: &[Point]) -> Rect {
        Rect {
            left: pois.iter().map(|point| point.x).min().unwrap(),
            right: pois.iter().map(|point| point.x).max().unwrap(),
            top: pois.iter().map(|point| point.y).min().unwrap(),
            bottom: pois.iter().map(|point| point.y).max().unwrap(),
        }
    }

    fn nearest_neighbors(pois: &[Point], x_len: usize, y_len: usize) -> NearestNeighbors {
        let mut cells = vec![None; x_len * y_len];

        for x in 0..x_len {
            for y in 0..y_len {
                let cell_position = Point::new(x as i32, y as i32);

                let distances: Vec<(usize, i32)> = pois
                    .iter()
                    .enumerate()
                    .map(|(i, poi)| {
                        let distance = cell_position.distance(*poi);
                        (i, distance)
                    })
                    .collect();

                let (closest_poi_index, min_distance) = distances
                    .iter()
                    .min_by_key(|(_, distance)| distance)
                    .unwrap();

                let instances = distances
                    .iter()
                    .filter(|(_, distance)| distance == min_distance)
                    .count();

                let single_min = instances == 1;

                cells[x * x_len + y] =
                    if single_min {
                        Some(*closest_poi_index)
                    } else {
                        None
                    };
            }
        }

        cells
    }

    fn corners(&self) -> Vec<usize> {
        vec![
            self.pois.iter().enumerate().min_by_key(|(_, poi)| (poi.x, poi.y)).unwrap().0,
            self.pois.iter().enumerate().min_by_key(|(_, poi)| (poi.x, -poi.y)).unwrap().0,
            self.pois.iter().enumerate().min_by_key(|(_, poi)| (-poi.x, poi.y)).unwrap().0,
            self.pois.iter().enumerate().min_by_key(|(_, poi)| (-poi.x, -poi.y)).unwrap().0,
        ]
    }

    /// Returns area by POI
    fn areas(&self) -> HashMap<usize, u32> {
        let mut areas: HashMap<usize, u32> = HashMap::new();

        for cell in self.nearest_neighbors.iter() {
            if let Some(poi_index) = cell {
                areas.entry(*poi_index)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }

        areas
    }

    fn infinite_areas(&self) -> Vec<usize> {
        let mut infinite_areas: HashSet<usize> = HashSet::new();

        let x_min = 0;
        let x_max = self.x_len - 1;
        let y_min = 0;
        let y_max = self.y_len - 1;

        for x in 0..=x_max {
            if let Some(poi_index) = self.nearest_neighbors[x * self.x_len + y_min] {
                infinite_areas.insert(poi_index);
            }
            if let Some(poi_index) = self.nearest_neighbors[x * self.x_len + y_max] {
                infinite_areas.insert(poi_index);
            }
        }

        for y in 0..=y_max {
            if let Some(poi_index) = self.nearest_neighbors[x_min * self.x_len + y] {
                infinite_areas.insert(poi_index);
            }
            if let Some(poi_index) = self.nearest_neighbors[x_max * self.x_len + y] {
                infinite_areas.insert(poi_index);
            }
        }

        infinite_areas
            .into_iter()
            .collect()
    }

    fn poi_index_with_max_area(&self) -> (usize, u32) {
        let infinite_areas = self.corners();

        let (&a, &b) = self.areas()
            .iter()
            .filter(|(&poi_index, _)| {
                !infinite_areas.contains(&poi_index)
            })
            .max_by_key(|&(_, area)| area)
            .unwrap()
            .clone();

        (a, b)
    }

    fn max_area(&self) -> u32 {
        self.poi_index_with_max_area().1
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                let p = Point::new(x as i32, y as i32);
                match self.nearest_neighbors[x * self.x_len + y] {
                    Some(index) => {
                        let c = std::char::from_u32(index as u32 + 97).unwrap();
                        if self.pois.contains(&p) {
                            write!(f, "{}", c.to_ascii_uppercase())?;
                        } else {
                            write!(f, "{:1}", c)?;
                        }
                    }
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
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
    let grid = Grid::new(pois);

    println!("border:{:?}", grid.border);
    println!("max_poi:{}", grid.poi_index_with_max_area().0);
    println!("{}", grid);

    grid.max_area()
}

fn part2(lines: &[&str]) -> i32 {
    0
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
        println!("{}", grid);
        println!("areas:{:?}", areas);
        println!("corners:{:?}", corners);
        println!("max_area_poi:{:?}", max_area_poi);

        assert_eq!(grid.max_area(), 17);

        assert_eq!(super::part1(&lines), 17);
    }
}