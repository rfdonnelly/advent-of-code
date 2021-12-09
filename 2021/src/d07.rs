use crate::input;

use tap::prelude::*;

const DAY: usize = 7;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

struct Positions(Vec<usize>);

impl From<&str> for Positions {
    fn from(s: &str) -> Self {
        let positions = s
            .trim()
            .split(",")
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect::<Vec<usize>>()
            .tap_mut(|vec| vec.sort());

        Self(positions)
    }
}

impl Positions {
    fn cost<F>(&self, cost_fn: F) -> usize
    where
        F: FnMut(&usize) -> usize
    {
        self.0
            .iter()
            .map(cost_fn)
            .sum()
    }

    fn median(&self) -> usize {
        let index = self.0.len() / 2;
        self.0[index]
    }
}

fn p1_cost(a: usize, b: usize) -> usize {
    ((a as i32) - (b as i32)).abs() as usize
}

fn p1(input: &str) -> usize {
    let positions = Positions::from(input);
    let to = positions.median();
    positions.cost(|&from| p1_cost(from, to))
}

fn p2_cost(a: usize, b: usize) -> usize {
    let diff = p1_cost(a, b);
    (diff * (diff + 1)) / 2
}

fn p2(input: &str) -> usize {
    let positions = Positions::from(input);

    let min = *positions.0.first().unwrap();
    let max = *positions.0.last().unwrap();

    (min..=max)
        .map(|to| positions.cost(|&from| p2_cost(from, to)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        16,1,2,0,4,2,7,1,2,14
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 37);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 344297);
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2(INPUT), 168);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 97164301);
    }
}
