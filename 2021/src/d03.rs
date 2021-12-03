use crate::input;

use std::ops::Add;

const DAY: usize = 3;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug)]
struct Value {
    bits: Vec<i8>,
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        let bits = s
            .chars()
            .map(|c| match c {
                '0' => -1,
                '1' => 1,
                _ => panic!(),
            })
            .collect::<Vec<i8>>();

        Self { bits }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let bits = self.bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<i8>>();

        Self { bits }
    }
}

impl Value {
    fn to_usize(self, size: usize) -> usize {
        self.bits
            .iter()
            .map(|b| match b {
                x if x > &0 => 1,
                x if x < &0 => 0,
                _ => panic!(),
            })
            .enumerate()
            .map(|(i, v)| v << (size - i - 1))
            .sum()
    }
}


fn p1(input: &str) -> usize {
    let size = input
        .lines()
        .next()
        .unwrap()
        .len();

    let gamma_rate = input
        .lines()
        .map(Value::from)
        .reduce(|a, b| a + b)
        .unwrap()
        .to_usize(size);

    let mask = (1 << size) - 1;
    let epsilon_rate = !gamma_rate & mask;

    gamma_rate * epsilon_rate
}

fn p2(input: &str) -> i32 {
    input
        .lines()
        .count() as i32
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn p1() {
        let input = indoc! {"
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        "};
        assert_eq!(super::p1(input), 198);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 2498354);
    }

    #[test]
    fn p2() {
        let input = indoc! {"
        "};
        assert_eq!(super::p2(input), -1);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), -1);
    }
}
