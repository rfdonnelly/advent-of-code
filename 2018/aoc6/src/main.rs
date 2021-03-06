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
    println!("part2: {}", part2(&lines, 10000));

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

type Map = Vec<Option<usize>>;

#[derive(Debug, PartialEq)]
struct Grid {
    x_len: usize,
    y_len: usize,
    map: Map,
    pois: Vec<Point>,
}

impl Grid {
    fn from_unsafe(pois: Vec<Point>) -> Grid {
        let max_xy = Grid::max_xy(&pois);
        let x_len = max_xy.x as usize + 1;
        let y_len = max_xy.y as usize + 1;
        let map = Grid::unsafe_pois(&pois, x_len, y_len);

        Grid {
            x_len,
            y_len,
            map,
            pois,
        }
    }

    fn from_safe(pois: Vec<Point>, with_in_distance: i32) -> Grid {
        let max_xy = Grid::max_xy(&pois);
        let x_len = max_xy.x as usize + 1;
        let y_len = max_xy.y as usize + 1;
        let map = Grid::safe_pois(&pois, x_len, y_len, with_in_distance);

        Grid {
            x_len,
            y_len,
            map,
            pois,
        }
    }

    fn max_xy(pois: &[Point]) -> Point {
        Point {
            x: pois.iter().map(|point| point.x).max().unwrap(),
            y: pois.iter().map(|point| point.y).max().unwrap(),
        }
    }

    fn unsafe_pois(pois: &[Point], x_len: usize, y_len: usize) -> Map {
        let mut cells = vec![None; x_len * y_len];

        for x in 0..x_len {
            for y in 0..y_len {
                let p = Point::new(x as i32, y as i32);
                cells[y * x_len + x] = Grid::nearest_neighbor(pois, p);
            }
        }

        cells
    }

    fn nearest_neighbor(pois: &[Point], p: Point) -> Option<usize> {
        let distances: Vec<(usize, i32)> = pois
            .iter()
            .enumerate()
            .map(|(i, poi)| {
                    let distance = p.distance(*poi);
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

        if single_min {
            Some(*closest_poi_index)
        } else {
            None
        }
    }

    fn safe_pois(pois: &[Point], x_len: usize, y_len: usize, with_in_distance: i32) -> Map {
        let mut cells = vec![None; x_len * y_len];

        for x in 0..x_len {
            for y in 0..y_len {
                let p = Point::new(x as i32, y as i32);
                cells[y * x_len + x] = Grid::distance_sum_less_than(pois, p, with_in_distance);
            }
        }

        cells
    }

    fn distance_sum_less_than(pois: &[Point], p: Point, with_in_distance: i32) -> Option<usize> {
        let distance_sum: i32 = pois.iter()
            .map(|&poi| poi.distance(p))
            .sum();

        if distance_sum < with_in_distance {
            Some(0)
        } else {
            None
        }
    }

    /// Returns area by POI
    fn areas(&self) -> HashMap<usize, u32> {
        let mut areas: HashMap<usize, u32> = HashMap::new();

        for cell in self.map.iter() {
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
            if let Some(poi_index) = self.map[y_min * self.x_len + x] {
                infinite_areas.insert(poi_index);
            }
            if let Some(poi_index) = self.map[y_max * self.x_len + x] {
                infinite_areas.insert(poi_index);
            }
        }

        for y in 0..=y_max {
            if let Some(poi_index) = self.map[y * self.x_len + x_min] {
                infinite_areas.insert(poi_index);
            }
            if let Some(poi_index) = self.map[y * self.x_len + x_max] {
                infinite_areas.insert(poi_index);
            }
        }

        infinite_areas
            .into_iter()
            .collect()
    }

    fn poi_index_with_max_area(&self) -> (usize, u32) {
        let infinite_areas = self.infinite_areas();

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
                if self.pois.contains(&p) {
                    write!(f, " *")?;
                } else {
                    match self.map[y * self.x_len + x] {
                        Some(index) => write!(f, "{:2}", index)?,
                            None => write!(f, " .")?,
                    }
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
    let grid = Grid::from_unsafe(pois);

    println!("max_poi:{}", grid.poi_index_with_max_area().0);
    println!("{}", grid);
    println!("infinite_areas:{:?}", grid.infinite_areas());
    println!("poi_index_with_max_area:{:?}", grid.poi_index_with_max_area());

    grid.max_area()
}

fn part2(lines: &[&str], with_in_distance: i32) -> u32 {
    let pois = parse_lines(lines);
    let grid = Grid::from_safe(pois, with_in_distance);
    let areas = grid.areas();

    println!("{}", grid);
    println!("{:?}", areas);

    areas[&0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines() -> Vec<&'static str> {
        vec![
            "1, 1",
            "1, 6",
            "8, 3",
            "3, 4",
            "5, 5",
            "8, 9",
        ]
    }

    #[test]
    fn part1() {
        let lines = lines();

        assert_eq!(Point::new(0, 0).distance(Point::new(1, 1)), 2);
        assert_eq!(Point::new(1, 1).distance(Point::new(0, 0)), 2);
        assert_eq!(Point::new(5, 1).distance(Point::new(1, 1)), 4);
        assert_eq!(Point::new(1, 1).distance(Point::new(5, 1)), 4);
        assert_eq!(Point::new(5, 1).distance(Point::new(5, 5)), 4);

        assert_eq!(Point::new(1, 1).distance(Point::new(1, 9)), 8);
        assert_eq!(Point::new(1, 6).distance(Point::new(1, 9)), 3);

        let pois = parse_lines(&lines);

        assert_eq!(Grid::nearest_neighbor(&pois, Point::new(1, 9)), Some(1));

        let grid = Grid::from_unsafe(pois.clone());
        let areas = grid.areas();
        let max_area_poi = areas.iter().max_by_key(|kv| kv.1).unwrap();

        println!("grid:{:?}", grid);
        println!("{}", grid);
        println!("{:?}", grid.infinite_areas());
        println!("areas:{:?}", areas);
        println!("max_area_poi:{:?}", max_area_poi);

        assert_eq!(grid.max_area(), 17);

        assert_eq!(super::part1(&lines), 17);
    }

    #[test]
    fn part2() {
        let lines = lines();
        let pois = parse_lines(&lines);
        let grid = Grid::from_safe(pois.clone(), 32);
        println!("{}", grid);

        assert_eq!(super::part2(&lines, 32), 16);
    }
}
