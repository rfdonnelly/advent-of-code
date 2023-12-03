use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Number {
    value: u64,
    start: Point,
    end: Point,
}

impl Number {
    fn is_adjacent(&self, rhs: &Symbol) -> bool {
        // NOTE: Assumes numbers are no more than 4 digits
        self.start.is_adjacent(&rhs.start) || self.end.is_adjacent(&rhs.start)
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    start: Point,
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_offset(offset: usize, cols: usize) -> Self {
        let x = offset % cols;
        let y = offset / cols;

        Point { x, y }
    }

    fn is_adjacent(&self, rhs: &Self) -> bool {
        let y_diff = rhs.y.abs_diff(self.y);
        let x_diff = rhs.x.abs_diff(self.x);

        y_diff <= 1 && x_diff <= 1
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    let cols = input.find('\n').unwrap() + 1;
    let numbers = Regex::new(r"\d+").unwrap();
    let symbols = Regex::new(r"[^\d\.\n]").unwrap();

    let numbers = numbers
        .find_iter(input)
        .map(|m| Number {
            value: m.as_str().parse().unwrap(),
            start: Point::from_offset(m.start(), cols),
            end: Point::from_offset(m.end() - 1, cols),
        })
        .collect();
    let symbols = symbols
        .find_iter(input)
        .map(|m| Symbol {
            value: m.as_str().chars().next().unwrap(),
            start: Point::from_offset(m.start(), cols),
        })
        .collect();

    Schematic { numbers, symbols }
}

#[aoc(day3, part1)]
fn part1(input: &Schematic) -> u64 {
    input
        .numbers
        .iter()
        .filter(|number| {
            input
                .symbols
                .iter()
                .any(|symbol| number.is_adjacent(symbol))
        })
        .map(|number| number.value)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Schematic) -> u64 {
    input
        .symbols
        .iter()
        .filter(|symbol| symbol.value == '*')
        .map(|symbol| {
            input
                .numbers
                .iter()
                .filter(|number| number.is_adjacent(symbol))
                .map(|number| number.value)
                .collect::<Vec<_>>()
        })
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.iter().product::<u64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 467835);
    }
}
