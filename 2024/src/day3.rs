use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;

type Number = i32;
type Parsed = Vec<(Number, Number)>;

enum Token {
    Do,
    Dont,
    Mul(Number, Number),
}

#[aoc_generator(day3, part1)]
fn parse_p1(input: &str) -> Parsed {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

#[aoc_generator(day3, part2)]
fn parse_p2(input: &str) -> Parsed {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let initial_state = (true, Vec::new());
    re.captures_iter(input)
        .map(|c| match &c[0][0..3] {
            "do(" => Token::Do,
            "don" => Token::Dont,
            "mul" => Token::Mul(c[1].parse().unwrap(), c[2].parse().unwrap()),
            _ => unreachable!(),
        })
        .fold(initial_state, |mut state, token| {
            match token {
                Token::Do => state.0 = true,
                Token::Dont => state.0 = false,
                Token::Mul(a, b) => {
                    if state.0 {
                        state.1.push((a, b));
                    }
                }
            }

            state
        })
        .1
}

#[aoc(day3, part1)]
fn part1(input: &Parsed) -> Number {
    input.iter().map(|(a, b)| a * b).sum()
}

#[aoc(day3, part2)]
fn part2(input: &Parsed) -> Number {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_P1: &str = indoc! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};

    const INPUT_P2: &str = indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn test_parse_p1() {
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(parse_p1(INPUT_P1), expected);
    }

    #[test]
    fn test_parse_p2() {
        let expected = vec![(2, 4), (8, 5)];
        assert_eq!(parse_p2(INPUT_P2), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(INPUT_P1)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_p2(INPUT_P2)), 48);
    }
}
