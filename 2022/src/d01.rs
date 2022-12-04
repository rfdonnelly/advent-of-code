use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use tap::Tap;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|list| {
            list.lines()
                .map(u32::from_str)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
fn p1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|v| v.iter().sum::<u32>()).max().unwrap()
}

#[aoc(day1, part2)]
fn p2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<u32>>()
        .tap_mut(|v| v.sort())
        .iter()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "};

    #[test]
    fn test_parse() {
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 24000);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), 45000);
    }
}
