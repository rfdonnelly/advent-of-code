use crate::input;

use std::collections::HashSet;

const DAY: usize = 9;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug, PartialEq)]
struct Map {
    cols: usize,
    values: Vec<u8>,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let cols = s
            .lines()
            .next()
            .unwrap()
            .len();

        let values = s
            .lines()
            .flat_map(|line| {
                line
                    .chars()
                    .map(|c| {
                        c
                            .to_digit(10)
                            .unwrap() as u8
                    })
            })
            .collect::<Vec<u8>>();

        Self { cols, values }
    }
}

impl Map {
    fn minima(&self) -> Vec<(usize, u8)> {
        self.values
            .iter()
            .enumerate()
            .filter(|(i, _v)| self.is_local_min(*i))
            .map(|(i, v)| (i, *v))
            .collect()
    }

    /// Returns true if local minimum exists at index
    fn is_local_min(&self, index: usize) -> bool {
        let min_neighbor = self
            .neighbors(index)
            .iter()
            .filter_map(|x| x.as_ref())
            .min_by_key(|(_i, v)| v)
            .unwrap()
            .1;

        self.values[index] < min_neighbor
    }

    fn neighbors(&self, index: usize) -> [Option<(usize, u8)>; 4] {
        let col_index = index % self.cols;

        let at_left_edge = col_index == 0;
        let left =
            if at_left_edge {
                None
            } else {
                let index = index - 1;
                Some((index, self.values[index]))
            };

        let at_right_edge = col_index == self.cols - 1;
        let right =
            if at_right_edge {
                None
            } else {
                let index = index + 1;
                Some((index, self.values[index]))
            };

        let above_index = index.wrapping_sub(self.cols);
        let above = self.values
            .get(above_index)
            .map(|&v| (above_index, v));

        let below_index = index + self.cols;
        let below = self.values
            .get(below_index)
            .map(|&v| (below_index, v));

        [left, right, above, below]
    }

    fn basin_size(&self, index: usize) -> usize {
        let mut visited: HashSet<usize> = HashSet::new();

        let mut to_visit = vec![index];

        loop {
            let index = to_visit.pop().unwrap();

            visited.insert(index);

            let mut neighbors = self
                .neighbors(index)
                .iter()
                .filter_map(|x| x.as_ref())
                .filter(|(i, v)| *i != index && *v < 9 && !visited.contains(i))
                .map(|(i, _v)| *i)
                .collect::<Vec<usize>>();

            to_visit.append(&mut neighbors);

            if to_visit.len() == 0 {
                break;
            }
        }

        visited.len()
    }
}

fn p1(input: &str) -> usize {
    Map::from(input)
        .minima()
        .iter()
        .map(|(_i, v)| (v + 1) as usize)
        .sum()
}

fn p2(input: &str) -> usize {
    let map = Map::from(input);

    let mut sizes = Map::from(input)
        .minima()
        .iter()
        .map(|(i, _v)| map.basin_size(*i))
        .collect::<Vec<usize>>();

    sizes.sort();

    sizes
        .iter()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 15);

        let input = input(super::DAY);
        assert_eq!(super::p1(&input), 465);
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2(INPUT), 1134);

        let input = input(super::DAY);
        assert_eq!(super::p2(&input), 1269555);
    }
}
