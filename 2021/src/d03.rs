use crate::input;

use std::ops::Add;

const DAY: usize = 3;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug, PartialEq)]
struct Value {
    bits: Vec<i32>,
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
            .collect::<Vec<i32>>();

        Self { bits }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<i32>>();

        Self { bits }
    }
}

#[derive(Clone, Copy)]
enum Criteria {
    MostCommon,
    LeastCommon,
}

#[derive(Clone, Copy)]
enum EqualResult {
    Value(usize),
    Panic,
}

impl Value {
    fn to_usize(self, size: usize, criteria: Criteria, equal_result: EqualResult) -> usize {
        self.bits
            .iter()
            .map(|b| match b {
                x if x > &0 => match criteria {
                    Criteria::MostCommon => 1,
                    Criteria::LeastCommon => 0,
                },
                x if x < &0 => match criteria {
                    Criteria::MostCommon => 0,
                    Criteria::LeastCommon => 1,
                },
                x if x < &0 => 0,
                x if x == &0 => match equal_result {
                    EqualResult::Value(x) => x,
                    EqualResult::Panic => panic!(),
                },
                _ => panic!(),
            })
            .enumerate()
            .map(|(i, v)| v << (size - i - 1))
            .sum()
    }

    fn from_usize(from: usize, size: usize) -> Self {
        let bits = (0..size)
            .rev()
            .map(|bit_idx| {
                let bit_value = get_bit(from, bit_idx);
                match bit_value {
                    0 => -1,
                    1 => 1,
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<i32>>();

        Self { bits }
    }
}

fn p1(input: &str) -> usize {
    let size = input.lines().next().unwrap().len();

    let gamma_rate = input
        .lines()
        .map(Value::from)
        .reduce(|a, b| a + b)
        .unwrap()
        .to_usize(size, Criteria::MostCommon, EqualResult::Panic);

    let mask = (1 << size) - 1;
    let epsilon_rate = !gamma_rate & mask;

    gamma_rate * epsilon_rate
}

fn find_rating(
    values: &[usize],
    size: usize,
    criteria: Criteria,
    equal_result: EqualResult,
) -> usize {
    let mut values = values.to_vec();

    for bit_idx in (0..size).rev() {
        let criteria = values
            .iter()
            .map(|value| Value::from_usize(*value, size))
            .reduce(|a, b| a + b)
            .unwrap()
            .to_usize(size, criteria, equal_result);

        let criteria_bit_value = get_bit(criteria, bit_idx);
        values = values
            .into_iter()
            .filter(|value| {
                let bit_value = get_bit(*value, bit_idx);
                bit_value == criteria_bit_value
            })
            .collect::<Vec<usize>>();

        if values.len() == 1 {
            break;
        }
    }

    values[0]
}

fn get_bit(value: usize, bit_idx: usize) -> usize {
    (value >> bit_idx) & 1
}

fn p2(input: &str) -> usize {
    let size = input.lines().next().unwrap().len();

    let values = input
        .lines()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect::<Vec<usize>>();

    let oxygen_generator_rating =
        find_rating(&values, size, Criteria::MostCommon, EqualResult::Value(1));

    let co2_scrubber_rating =
        find_rating(&values, size, Criteria::LeastCommon, EqualResult::Value(0));

    oxygen_generator_rating * co2_scrubber_rating
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
        assert_eq!(
            super::Value::from_usize(22, 5),
            super::Value {
                bits: vec![1, -1, 1, 1, -1]
            }
        );

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
        assert_eq!(super::p2(input), 230);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 3277956);
    }
}
