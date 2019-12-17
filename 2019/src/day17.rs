use std::fs;
use std::io;

use itertools::Itertools;

use crate::computer::{
    Computer,
    Program,
    State,
};
use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day17();

    println!("day17::part1: {}", part1);
    println!("day17::part2: {}", part2);

    Ok(())
}

fn day17() -> (usize, i64) {
    let input = fs::read_to_string("input/17").unwrap();

    let program: Program = Program::from(&input);
    (part1(program.clone()), part2(program))
}

fn part1(program: Program) -> usize {
    let outputs =
        Computer::new(program, vec![])
        .run()
        .unwrap()
        .outputs;

    let image = Image::from(outputs.as_slice());
    alignment_parameter_sum(&image)
}

type ImageData = Vec<Vec<char>>;
struct Image {
    data: ImageData,
}

impl From<&[i64]> for Image {
    fn from(a: &[i64]) -> Self {
        let data = a
            .iter()
            .map(|&i| (i as u8) as char)
            .collect::<String>()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<ImageData>();

        Self {data}
    }
}

impl Image {
    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn get(&self, x: usize, y: usize) -> char {
        match self.data.get(y) {
            Some(row) => *row.get(x).unwrap_or(&' '),
            None => ' ',
        }
    }
}

trait IsScaffold {
    fn is_scaffold(&self) -> bool;
}

impl IsScaffold for char {
    fn is_scaffold(&self) -> bool {
        *self == '#'
    }
}

fn intersections(image: &Image) -> Vec<(usize, usize)> {
    let mut intersections = Vec::new();

    for x in 0..image.width() {
        for y in 0..image.height() {
            if image.get(x, y).is_scaffold() {
                let is_intersection =
                    image.get(x.wrapping_sub(1), y).is_scaffold()
                    && image.get(x + 1, y).is_scaffold()
                    && image.get(x, y.wrapping_sub(1)).is_scaffold()
                    && image.get(x, y + 1).is_scaffold();

                if is_intersection {
                    intersections.push((x, y));
                }
            }
        }
    }

    intersections
}

fn alignment_parameter_sum(image: &Image) -> usize {
    intersections(image)
        .iter()
        .map(|(x, y)| x * y)
        .sum()
}

fn part2(program: Program) -> i64 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    impl From<&str> for Image {
        fn from(s: &str) -> Self {
            let data = s
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<ImageData>();

            Self {data}
        }
    }

    #[test]
    fn test_part1_example1() {
        let input = indoc!("
            ..#..........
            ..#..........
            #######...###
            #.#...#...#.#
            #############
            ..#...#...#..
            ..#####...^..
        ");
        assert_eq!(alignment_parameter_sum(&Image::from(input)), 76);
    }

    #[test]
    fn test_day17() {
        assert_eq!(day17(), (4112, 1))
    }
}
